use super::*;

impl SyncService {
    pub(super) fn apply_remote_changes(
        conn: &Connection,
        local_categories: &[Category],
        local_todos: &[TodoItem],
        remote_categories: Vec<RemoteCategory>,
        remote_todos: Vec<RemoteTodo>,
    ) -> Result<usize, String> {
        let mut count = 0;

        let category_sync_map: HashMap<String, i64> = local_categories
            .iter()
            .filter_map(|c| c.sync_id.as_ref().map(|s| (s.clone(), c.id)))
            .collect();

        for remote in &remote_categories {
            if let Some(&local_id) = category_sync_map.get(&remote.id) {
                if let Some(local) = local_categories.iter().find(|c| c.id == local_id) {
                    if Self::is_remote_newer(&local.updated_at, &remote.updated_at) {
                        Self::update_local_category(conn, local, remote)?;
                        count += 1;
                    }
                }
            } else {
                Self::insert_category_from_remote(conn, remote)?;
                count += 1;
            }
        }

        let updated_categories = CategoryRepository::get_all(conn).map_err(|e| e.to_string())?;
        let category_sync_map: HashMap<String, i64> = updated_categories
            .iter()
            .filter_map(|c| c.sync_id.as_ref().map(|s| (s.clone(), c.id)))
            .collect();

        let todo_sync_map: HashMap<String, i64> = local_todos
            .iter()
            .filter_map(|t| t.sync_id.as_ref().map(|s| (s.clone(), t.id)))
            .collect();

        for remote in &remote_todos {
            let local_category_id = remote
                .category_id
                .as_ref()
                .and_then(|sync_id| category_sync_map.get(sync_id).copied());

            if let Some(&local_id) = todo_sync_map.get(&remote.id) {
                if let Some(local) = local_todos.iter().find(|t| t.id == local_id) {
                    if Self::is_remote_newer(&local.updated_at, &remote.updated_at) {
                        Self::update_local_todo(conn, local, remote, local_category_id)?;
                        count += 1;
                    }
                }
            } else {
                Self::insert_todo_from_remote(conn, remote, local_category_id)?;
                count += 1;
            }
        }

        Ok(count)
    }

    fn is_remote_newer(local_updated: &Option<String>, remote_updated: &str) -> bool {
        match local_updated {
            Some(local) => {
                let local_dt = chrono::DateTime::parse_from_rfc3339(local).ok();
                let remote_dt = chrono::DateTime::parse_from_rfc3339(remote_updated).ok();

                match (local_dt, remote_dt) {
                    (Some(l), Some(r)) => r > l,
                    _ => true,
                }
            }
            None => true,
        }
    }

    fn update_local_category(
        conn: &Connection,
        local: &Category,
        remote: &RemoteCategory,
    ) -> Result<(), String> {
        conn.execute(
            "UPDATE categories SET name = ?1, display_order = ?2, updated_at = ?3, sync_status = 'synced' WHERE id = ?4",
            rusqlite::params![remote.name, remote.display_order, remote.updated_at, local.id],
        )
        .map_err(|e| e.to_string())?;
        Ok(())
    }

    fn insert_category_from_remote(
        conn: &Connection,
        remote: &RemoteCategory,
    ) -> Result<(), String> {
        conn.execute(
            "INSERT INTO categories (name, display_order, sync_id, created_at, updated_at, sync_status)
             VALUES (?1, ?2, ?3, ?4, ?5, 'synced')
             ON CONFLICT(sync_id) DO UPDATE SET
                name = excluded.name,
                display_order = excluded.display_order,
                updated_at = excluded.updated_at,
                sync_status = 'synced'",
            rusqlite::params![
                remote.name,
                remote.display_order,
                remote.id,
                remote.created_at,
                remote.updated_at
            ],
        )
        .map_err(|e| format!("Failed to insert category: {}", e))?;
        Ok(())
    }

    fn update_local_todo(
        conn: &Connection,
        local: &TodoItem,
        remote: &RemoteTodo,
        local_category_id: Option<i64>,
    ) -> Result<(), String> {
        conn.execute(
            "UPDATE todos SET text = ?1, done = ?2, category_id = ?3, display_order = ?4, memo = ?5,
             repeat_type = ?6, repeat_detail = ?7, next_due_at = ?8, last_completed_at = ?9,
             track_streak = ?10, reminder_at = ?11, linked_app = ?12, updated_at = ?13, sync_status = 'synced' WHERE id = ?14",
            rusqlite::params![
                remote.text,
                remote.done,
                local_category_id,
                remote.display_order,
                remote.memo,
                remote.repeat_type,
                remote.repeat_detail,
                remote.next_due_at,
                remote.last_completed_at,
                remote.track_streak,
                remote.reminder_at,
                remote.linked_app,
                remote.updated_at,
                local.id
            ],
        )
        .map_err(|e| e.to_string())?;
        Ok(())
    }

    fn insert_todo_from_remote(
        conn: &Connection,
        remote: &RemoteTodo,
        local_category_id: Option<i64>,
    ) -> Result<(), String> {
        conn.execute(
            "INSERT INTO todos (text, done, category_id, display_order, memo, repeat_type, repeat_detail,
             next_due_at, last_completed_at, track_streak, reminder_at, linked_app, sync_id, created_at, updated_at, sync_status)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, ?13, ?14, ?15, 'synced')
             ON CONFLICT(sync_id) DO UPDATE SET
                text = excluded.text,
                done = excluded.done,
                category_id = excluded.category_id,
                display_order = excluded.display_order,
                memo = excluded.memo,
                repeat_type = excluded.repeat_type,
                repeat_detail = excluded.repeat_detail,
                next_due_at = excluded.next_due_at,
                last_completed_at = excluded.last_completed_at,
                track_streak = excluded.track_streak,
                reminder_at = excluded.reminder_at,
                linked_app = excluded.linked_app,
                updated_at = excluded.updated_at,
                sync_status = 'synced'",
            rusqlite::params![
                remote.text,
                remote.done,
                local_category_id,
                remote.display_order,
                remote.memo,
                remote.repeat_type,
                remote.repeat_detail,
                remote.next_due_at,
                remote.last_completed_at,
                remote.track_streak,
                remote.reminder_at,
                remote.linked_app,
                remote.id,
                remote.created_at,
                remote.updated_at
            ],
        )
        .map_err(|e| format!("Failed to insert todo: {}", e))?;
        Ok(())
    }

    pub(super) fn apply_remote_tags(
        conn: &Connection,
        remote_tags: Vec<RemoteTag>,
    ) -> Result<usize, String> {
        let mut count = 0;

        for remote in remote_tags {
            let existing =
                TagRepository::get_by_sync_id(conn, &remote.id).map_err(|e| e.to_string())?;
            if let Some(local) = existing {
                if Self::is_remote_newer(&local.updated_at, &remote.updated_at) {
                    conn.execute(
                        "UPDATE tags SET name = ?1, updated_at = ?2, sync_status = 'synced' WHERE id = ?3",
                        rusqlite::params![remote.name, remote.updated_at, local.id],
                    )
                    .map_err(|e| e.to_string())?;
                    count += 1;
                }
            } else {
                if let Some(local_by_name) =
                    TagRepository::get_by_name(conn, &remote.name).map_err(|e| e.to_string())?
                {
                    TagRepository::update_sync_id(conn, local_by_name.id, &remote.id)
                        .map_err(|e| e.to_string())?;
                } else {
                    conn.execute(
                        "INSERT INTO tags (name, sync_id, created_at, updated_at, sync_status) VALUES (?1, ?2, ?3, ?4, 'synced')",
                        rusqlite::params![remote.name, remote.id, remote.created_at, remote.updated_at],
                    )
                    .map_err(|e| e.to_string())?;
                }
                count += 1;
            }
        }

        Ok(count)
    }

    pub(super) fn apply_remote_todo_tags(
        conn: &Connection,
        remote_todo_tags: Vec<RemoteTodoTag>,
    ) -> Result<usize, String> {
        let mut count = 0;

        let all_todos = TodoRepository::get_all(conn).map_err(|e| e.to_string())?;
        let todo_sync_to_local: HashMap<String, i64> = all_todos
            .iter()
            .filter_map(|t| t.sync_id.as_ref().map(|s| (s.clone(), t.id)))
            .collect();

        let all_tags = TagRepository::get_all(conn).map_err(|e| e.to_string())?;
        let tag_sync_to_local: HashMap<String, i64> = all_tags
            .iter()
            .filter_map(|t| t.sync_id.as_ref().map(|s| (s.clone(), t.id)))
            .collect();

        for remote in remote_todo_tags {
            let local_todo_id = todo_sync_to_local.get(&remote.todo_id).copied();
            let local_tag_id = tag_sync_to_local.get(&remote.tag_id).copied();

            if let (Some(todo_id), Some(tag_id)) = (local_todo_id, local_tag_id) {
                let existing = TodoTagRepository::get_by_sync_id(conn, &remote.id)
                    .map_err(|e| e.to_string())?;
                if existing.is_none() {
                    TodoTagRepository::add_tag(conn, todo_id, tag_id).map_err(|e| e.to_string())?;
                    TodoTagRepository::update_sync_id(conn, todo_id, tag_id, &remote.id)
                        .map_err(|e| e.to_string())?;
                    count += 1;
                }
            }
        }

        Ok(count)
    }

    pub(super) fn apply_remote_completion_logs(
        conn: &Connection,
        local_todos: &[TodoItem],
        remote_logs: Vec<RemoteCompletionLog>,
    ) -> Result<usize, String> {
        let mut count = 0;

        let sync_id_to_local_id: HashMap<String, i64> = local_todos
            .iter()
            .filter_map(|t| t.sync_id.as_ref().map(|s| (s.clone(), t.id)))
            .collect();

        for remote in remote_logs {
            if let Some(&local_id) = sync_id_to_local_id.get(&remote.todo_id) {
                CompletionLogRepository::upsert(
                    conn,
                    local_id,
                    &remote.completed_on,
                    remote.completed_count,
                )
                .map_err(|e| e.to_string())?;
                count += 1;
            }
        }

        Ok(count)
    }
}
