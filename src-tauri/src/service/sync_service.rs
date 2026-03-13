use chrono::Utc;
use rusqlite::Connection;
use std::collections::HashMap;
use uuid::Uuid;

use crate::models::{Category, SyncResult, SyncStatus, TodoItem};
use crate::repository::{
    CategoryRepository, CompletionLogRepository, SyncRepository, TagRepository, TodoRepository,
    TodoTagRepository,
};

use super::supabase_client::{
    RemoteCategory, RemoteCompletionLog, RemoteTag, RemoteTodo, RemoteTodoTag, SupabaseClient,
};

mod apply;
mod collect;
mod push;

pub struct SyncService;

/// Data structures to pass between sync phases (to avoid holding connection across async)
#[derive(Debug, Clone)]
struct PendingCategorySync {
    id: i64,
    sync_id: Option<String>,
    name: String,
    display_order: i64,
    created_at: Option<String>,
    updated_at: Option<String>,
    sync_status: SyncStatus,
}

#[derive(Debug, Clone)]
struct PendingTodoSync {
    id: i64,
    sync_id: Option<String>,
    category_sync_id: Option<String>,
    text: String,
    done: bool,
    display_order: i64,
    memo: Option<String>,
    repeat_type: String,
    repeat_detail: Option<String>,
    next_due_at: Option<String>,
    last_completed_at: Option<String>,
    track_streak: bool,
    reminder_at: Option<String>,
    linked_app: Option<String>,
    created_at: Option<String>,
    updated_at: Option<String>,
    sync_status: SyncStatus,
}

#[derive(Debug, Clone)]
struct PendingTagSync {
    id: i64,
    sync_id: Option<String>,
    name: String,
    created_at: Option<String>,
    updated_at: Option<String>,
    sync_status: SyncStatus,
}

#[derive(Debug, Clone)]
#[allow(dead_code)]
struct PendingTodoTagSync {
    todo_id: i64,
    tag_id: i64,
    sync_id: Option<String>,
    todo_sync_id: Option<String>,
    tag_sync_id: Option<String>,
    created_at: Option<String>,
    sync_status: SyncStatus,
}

#[derive(Debug, Clone)]
#[allow(dead_code)]
struct LocalCompletionLogSync {
    item_id: i64,
    todo_sync_id: String,
    completed_on: String,
    completed_count: i32,
}

impl SyncService {
    /// Blocking version that uses tokio runtime internally
    pub fn sync_all_blocking(
        conn: &Connection,
        client: &SupabaseClient,
        access_token: &str,
        user_id: &str,
    ) -> Result<SyncResult, String> {
        let rt = tokio::runtime::Builder::new_current_thread()
            .enable_all()
            .build()
            .map_err(|e| format!("Failed to create runtime: {}", e))?;

        let mut pending_categories = Self::collect_pending_categories(conn)?;
        for cat in &mut pending_categories {
            if cat.sync_status == SyncStatus::Pending && cat.sync_id.is_none() {
                let new_sync_id = Uuid::new_v4().to_string();
                CategoryRepository::assign_sync_id(conn, cat.id, &new_sync_id)
                    .map_err(|e| e.to_string())?;
                cat.sync_id = Some(new_sync_id.clone());
            }
        }

        let all_categories =
            CategoryRepository::get_all_including_deleted(conn).map_err(|e| e.to_string())?;
        let mut cat_id_to_sync_id: HashMap<i64, String> = HashMap::new();
        for cat in &all_categories {
            if let Some(sync_id) = &cat.sync_id {
                cat_id_to_sync_id.insert(cat.id, sync_id.clone());
            }
        }

        let mut pending_todos = Self::collect_pending_todos_with_map(conn, &cat_id_to_sync_id)?;
        for todo in &mut pending_todos {
            if todo.sync_status == SyncStatus::Pending && todo.sync_id.is_none() {
                let new_sync_id = Uuid::new_v4().to_string();
                TodoRepository::assign_sync_id(conn, todo.id, &new_sync_id)
                    .map_err(|e| e.to_string())?;
                todo.sync_id = Some(new_sync_id);
            }
        }

        let all_todos = TodoRepository::get_all(conn).map_err(|e| e.to_string())?;
        let remote_category_lookup: HashMap<String, RemoteCategory> = all_categories
            .iter()
            .filter(|cat| cat.sync_status != SyncStatus::Deleted)
            .filter_map(|cat| {
                cat.sync_id.as_ref().map(|sync_id| {
                    (
                        sync_id.clone(),
                        RemoteCategory {
                            id: sync_id.clone(),
                            user_id: user_id.to_string(),
                            name: cat.name.clone(),
                            display_order: cat.display_order as i32,
                            created_at: cat.created_at.clone().unwrap_or_else(|| {
                                Utc::now().format("%Y-%m-%dT%H:%M:%SZ").to_string()
                            }),
                            updated_at: cat.updated_at.clone().unwrap_or_else(|| {
                                Utc::now().format("%Y-%m-%dT%H:%M:%SZ").to_string()
                            }),
                        },
                    )
                })
            })
            .collect();
        let remote_todo_lookup: HashMap<String, RemoteTodo> = all_todos
            .iter()
            .filter_map(|todo| {
                todo.sync_id.as_ref().map(|sync_id| {
                    (
                        sync_id.clone(),
                        RemoteTodo {
                            id: sync_id.clone(),
                            user_id: user_id.to_string(),
                            category_id: todo.category_id.and_then(|category_id| {
                                cat_id_to_sync_id.get(&category_id).cloned()
                            }),
                            text: todo.text.clone(),
                            done: todo.done,
                            display_order: todo.display_order as i32,
                            memo: todo.memo.clone(),
                            repeat_type: todo.repeat_type.to_str().to_string(),
                            repeat_detail: todo.repeat_detail.clone(),
                            next_due_at: todo.next_due_at.clone(),
                            last_completed_at: todo.last_completed_at.clone(),
                            track_streak: todo.track_streak,
                            reminder_at: todo.reminder_at.clone(),
                            linked_app: todo.linked_app.clone(),
                            created_at: todo.created_at.clone().unwrap_or_else(|| {
                                Utc::now().format("%Y-%m-%dT%H:%M:%SZ").to_string()
                            }),
                            updated_at: todo.updated_at.clone().unwrap_or_else(|| {
                                Utc::now().format("%Y-%m-%dT%H:%M:%SZ").to_string()
                            }),
                        },
                    )
                })
            })
            .collect();

        CompletionLogRepository::prune_stale_logs(conn).map_err(|e| e.to_string())?;

        let mut todo_id_to_sync_id: HashMap<i64, String> = HashMap::new();
        for todo in &all_todos {
            if let Some(sync_id) = &todo.sync_id {
                todo_id_to_sync_id.insert(todo.id, sync_id.clone());
            }
        }
        for todo in &pending_todos {
            if todo.sync_status != SyncStatus::Deleted {
                if let Some(sync_id) = &todo.sync_id {
                    todo_id_to_sync_id.insert(todo.id, sync_id.clone());
                }
            }
        }

        let local_completion_logs = Self::collect_completion_logs(conn, &todo_id_to_sync_id)?;

        let mut pending_tags = Self::collect_pending_tags(conn)?;
        for tag in &mut pending_tags {
            if tag.sync_status == SyncStatus::Pending && tag.sync_id.is_none() {
                let new_sync_id = Uuid::new_v4().to_string();
                TagRepository::assign_sync_id(conn, tag.id, &new_sync_id)
                    .map_err(|e| e.to_string())?;
                tag.sync_id = Some(new_sync_id);
            }
        }

        let all_tags = TagRepository::get_all(conn).map_err(|e| e.to_string())?;
        let mut tag_id_to_sync_id: HashMap<i64, String> = HashMap::new();
        for tag in &all_tags {
            if let Some(sync_id) = &tag.sync_id {
                tag_id_to_sync_id.insert(tag.id, sync_id.clone());
            }
        }
        for tag in &pending_tags {
            if let Some(sync_id) = &tag.sync_id {
                tag_id_to_sync_id.insert(tag.id, sync_id.clone());
            }
        }

        let mut pending_todo_tags =
            Self::collect_pending_todo_tags(conn, &todo_id_to_sync_id, &tag_id_to_sync_id)?;
        for todo_tag in &mut pending_todo_tags {
            if todo_tag.sync_status == SyncStatus::Pending && todo_tag.sync_id.is_none() {
                let new_sync_id = Uuid::new_v4().to_string();
                TodoTagRepository::assign_sync_id(
                    conn,
                    todo_tag.todo_id,
                    todo_tag.tag_id,
                    &new_sync_id,
                )
                .map_err(|e| e.to_string())?;
                todo_tag.sync_id = Some(new_sync_id);
            }
        }

        let result = rt.block_on(async {
            let mut result = SyncResult::default();

            let pushed_cats =
                Self::push_categories_async(client, access_token, user_id, &pending_categories)
                    .await?;
            let pushed_todos =
                Self::push_todos_async(client, access_token, user_id, &pending_todos).await?;
            let pushed_logs = Self::push_completion_logs_async(
                client,
                access_token,
                user_id,
                &local_completion_logs,
                &remote_todo_lookup,
                &remote_category_lookup,
            )
            .await?;
            let pushed_tags =
                Self::push_tags_async(client, access_token, user_id, &pending_tags).await?;
            let pushed_todo_tags =
                Self::push_todo_tags_async(client, access_token, user_id, &pending_todo_tags)
                    .await?;

            result.pushed = pushed_cats.len()
                + pushed_todos.len()
                + pushed_logs
                + pushed_tags.len()
                + pushed_todo_tags.len();

            let remote_categories = client.fetch_categories(access_token).await?;
            let remote_todos = client.fetch_todos(access_token).await?;
            let remote_completion_logs = client.fetch_all_completion_logs(access_token).await?;
            let remote_tags = client.fetch_tags(access_token).await.unwrap_or_default();
            let remote_todo_tags = client
                .fetch_todo_tags(access_token)
                .await
                .unwrap_or_default();

            Ok::<_, String>((
                result,
                pushed_cats,
                pushed_todos,
                pushed_tags,
                pushed_todo_tags,
                remote_categories,
                remote_todos,
                remote_completion_logs,
                remote_tags,
                remote_todo_tags,
            ))
        })?;

        let (
            mut sync_result,
            pushed_cats,
            pushed_todos,
            pushed_tags,
            pushed_todo_tags,
            remote_categories,
            remote_todos,
            remote_completion_logs,
            remote_tags,
            remote_todo_tags,
        ) = result;

        for (local_id, sync_id) in pushed_cats {
            if let Some(cat) = pending_categories.iter().find(|c| c.id == local_id) {
                if cat.sync_id.is_none() {
                    CategoryRepository::update_sync_id(conn, local_id, &sync_id)
                        .map_err(|e| e.to_string())?;
                }
                if cat.sync_status == SyncStatus::Deleted {
                    CategoryRepository::delete(conn, local_id).map_err(|e| e.to_string())?;
                } else {
                    CategoryRepository::mark_synced(conn, local_id).map_err(|e| e.to_string())?;
                }
            }
        }

        for (local_id, sync_id) in pushed_todos {
            if let Some(todo) = pending_todos.iter().find(|t| t.id == local_id) {
                if todo.sync_id.is_none() {
                    TodoRepository::update_sync_id(conn, local_id, &sync_id)
                        .map_err(|e| e.to_string())?;
                }
                if todo.sync_status == SyncStatus::Deleted {
                    TodoRepository::delete(conn, local_id).map_err(|e| e.to_string())?;
                } else {
                    TodoRepository::mark_synced(conn, local_id).map_err(|e| e.to_string())?;
                }
            }
        }

        for (local_id, sync_id) in pushed_tags {
            if let Some(tag) = pending_tags.iter().find(|t| t.id == local_id) {
                if tag.sync_id.is_none() {
                    TagRepository::update_sync_id(conn, local_id, &sync_id)
                        .map_err(|e| e.to_string())?;
                }
                if tag.sync_status == SyncStatus::Deleted {
                    TagRepository::delete(conn, local_id).map_err(|e| e.to_string())?;
                } else {
                    TagRepository::mark_synced(conn, local_id).map_err(|e| e.to_string())?;
                }
            }
        }

        for (todo_id, tag_id, sync_id) in pushed_todo_tags {
            if let Some(todo_tag) = pending_todo_tags
                .iter()
                .find(|tt| tt.todo_id == todo_id && tt.tag_id == tag_id)
            {
                if todo_tag.sync_id.is_none() {
                    TodoTagRepository::update_sync_id(conn, todo_id, tag_id, &sync_id)
                        .map_err(|e| e.to_string())?;
                }
                if todo_tag.sync_status == SyncStatus::Deleted {
                    TodoTagRepository::delete(conn, todo_id, tag_id).map_err(|e| e.to_string())?;
                } else {
                    TodoTagRepository::mark_synced(conn, todo_id, tag_id)
                        .map_err(|e| e.to_string())?;
                }
            }
        }

        let updated_local_categories =
            CategoryRepository::get_all(conn).map_err(|e| e.to_string())?;
        let updated_local_todos = TodoRepository::get_all(conn).map_err(|e| e.to_string())?;

        let pulled = Self::apply_remote_changes(
            conn,
            &updated_local_categories,
            &updated_local_todos,
            remote_categories,
            remote_todos,
        )?;

        let pulled_logs =
            Self::apply_remote_completion_logs(conn, &updated_local_todos, remote_completion_logs)?;
        let pulled_tags = Self::apply_remote_tags(conn, remote_tags)?;
        let pulled_todo_tags = Self::apply_remote_todo_tags(conn, remote_todo_tags)?;

        sync_result.pulled = pulled + pulled_logs + pulled_tags + pulled_todo_tags;

        let now = Utc::now().format("%Y-%m-%dT%H:%M:%SZ").to_string();
        SyncRepository::set_last_synced_at(conn, &now).map_err(|e| e.to_string())?;
        sync_result.last_synced_at = Some(now);

        Ok(sync_result)
    }

    pub fn get_pending_count(conn: &Connection) -> Result<usize, String> {
        SyncRepository::get_pending_count(conn).map_err(|e| e.to_string())
    }

    pub fn get_last_synced_at(conn: &Connection) -> Result<Option<String>, String> {
        SyncRepository::get_last_synced_at(conn).map_err(|e| e.to_string())
    }

    pub fn is_sync_enabled(conn: &Connection) -> Result<bool, String> {
        SyncRepository::is_sync_enabled(conn).map_err(|e| e.to_string())
    }

    pub fn set_sync_enabled(conn: &Connection, enabled: bool) -> Result<(), String> {
        SyncRepository::set_sync_enabled(conn, enabled).map_err(|e| e.to_string())
    }
}
