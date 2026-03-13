use super::*;

impl SyncService {
    async fn repair_missing_remote_todo_async(
        client: &SupabaseClient,
        access_token: &str,
        todo_sync_id: &str,
        todos: &HashMap<String, RemoteTodo>,
        categories: &HashMap<String, RemoteCategory>,
    ) -> Result<(), String> {
        let todo = todos.get(todo_sync_id).ok_or_else(|| {
            format!(
                "Missing local todo snapshot for completion log todo_id {}",
                todo_sync_id
            )
        })?;

        if let Some(category_sync_id) = &todo.category_id {
            if let Some(category) = categories.get(category_sync_id) {
                client.upsert_category(access_token, category).await?;
            }
        }

        client.upsert_todo(access_token, todo).await?;
        Ok(())
    }

    fn is_missing_remote_todo_error(error: &str) -> bool {
        error.contains("\"23503\"") || error.contains("completion_logs_todo_id_fkey")
    }

    pub(super) async fn push_categories_async(
        client: &SupabaseClient,
        access_token: &str,
        user_id: &str,
        categories: &[PendingCategorySync],
    ) -> Result<Vec<(i64, String)>, String> {
        let mut results = Vec::new();

        for cat in categories {
            match cat.sync_status {
                SyncStatus::Pending => {
                    let sync_id = cat
                        .sync_id
                        .clone()
                        .unwrap_or_else(|| Uuid::new_v4().to_string());

                    let remote =
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
                        };

                    client.upsert_category(access_token, &remote).await?;
                    results.push((cat.id, sync_id));
                }
                SyncStatus::Deleted => {
                    if let Some(sync_id) = &cat.sync_id {
                        client.delete_category(access_token, sync_id).await?;
                        results.push((cat.id, sync_id.clone()));
                    }
                }
                _ => {}
            }
        }

        Ok(results)
    }

    pub(super) async fn push_todos_async(
        client: &SupabaseClient,
        access_token: &str,
        user_id: &str,
        todos: &[PendingTodoSync],
    ) -> Result<Vec<(i64, String)>, String> {
        let mut results = Vec::new();

        for todo in todos {
            match todo.sync_status {
                SyncStatus::Pending => {
                    let sync_id = todo
                        .sync_id
                        .clone()
                        .unwrap_or_else(|| Uuid::new_v4().to_string());

                    let remote =
                        RemoteTodo {
                            id: sync_id.clone(),
                            user_id: user_id.to_string(),
                            category_id: todo.category_sync_id.clone(),
                            text: todo.text.clone(),
                            done: todo.done,
                            display_order: todo.display_order as i32,
                            memo: todo.memo.clone(),
                            repeat_type: todo.repeat_type.clone(),
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
                        };

                    client.upsert_todo(access_token, &remote).await?;
                    results.push((todo.id, sync_id));
                }
                SyncStatus::Deleted => {
                    if let Some(sync_id) = &todo.sync_id {
                        client.delete_todo(access_token, sync_id).await?;
                        results.push((todo.id, sync_id.clone()));
                    }
                }
                _ => {}
            }
        }

        Ok(results)
    }

    pub(super) async fn push_tags_async(
        client: &SupabaseClient,
        access_token: &str,
        user_id: &str,
        tags: &[PendingTagSync],
    ) -> Result<Vec<(i64, String)>, String> {
        let mut results = Vec::new();

        for tag in tags {
            let sync_id = tag
                .sync_id
                .clone()
                .unwrap_or_else(|| Uuid::new_v4().to_string());

            match tag.sync_status {
                SyncStatus::Pending => {
                    let remote =
                        RemoteTag {
                            id: sync_id.clone(),
                            user_id: user_id.to_string(),
                            name: tag.name.clone(),
                            created_at: tag.created_at.clone().unwrap_or_else(|| {
                                Utc::now().format("%Y-%m-%dT%H:%M:%SZ").to_string()
                            }),
                            updated_at: tag.updated_at.clone().unwrap_or_else(|| {
                                Utc::now().format("%Y-%m-%dT%H:%M:%SZ").to_string()
                            }),
                        };
                    client.upsert_tag(access_token, &remote).await?;
                    results.push((tag.id, sync_id));
                }
                SyncStatus::Deleted => {
                    if let Some(sync_id) = &tag.sync_id {
                        client.delete_tag(access_token, sync_id).await?;
                        results.push((tag.id, sync_id.clone()));
                    }
                }
                _ => {}
            }
        }

        Ok(results)
    }

    pub(super) async fn push_todo_tags_async(
        client: &SupabaseClient,
        access_token: &str,
        user_id: &str,
        todo_tags: &[PendingTodoTagSync],
    ) -> Result<Vec<(i64, i64, String)>, String> {
        let mut results = Vec::new();

        for tt in todo_tags {
            let sync_id = tt
                .sync_id
                .clone()
                .unwrap_or_else(|| Uuid::new_v4().to_string());

            match tt.sync_status {
                SyncStatus::Pending => {
                    if let (Some(todo_sync_id), Some(tag_sync_id)) =
                        (&tt.todo_sync_id, &tt.tag_sync_id)
                    {
                        let remote = RemoteTodoTag {
                            id: sync_id.clone(),
                            user_id: user_id.to_string(),
                            todo_id: todo_sync_id.clone(),
                            tag_id: tag_sync_id.clone(),
                            created_at: tt.created_at.clone().unwrap_or_else(|| {
                                Utc::now().format("%Y-%m-%dT%H:%M:%SZ").to_string()
                            }),
                        };
                        client.upsert_todo_tag(access_token, &remote).await?;
                        results.push((tt.todo_id, tt.tag_id, sync_id));
                    }
                }
                SyncStatus::Deleted => {
                    if let Some(sync_id) = &tt.sync_id {
                        client.delete_todo_tag(access_token, sync_id).await?;
                        results.push((tt.todo_id, tt.tag_id, sync_id.clone()));
                    }
                }
                _ => {}
            }
        }

        Ok(results)
    }

    pub(super) async fn push_completion_logs_async(
        client: &SupabaseClient,
        access_token: &str,
        user_id: &str,
        logs: &[LocalCompletionLogSync],
        todos: &HashMap<String, RemoteTodo>,
        categories: &HashMap<String, RemoteCategory>,
    ) -> Result<usize, String> {
        let mut count = 0;

        for log in logs {
            let log_id = format!("{}_{}", log.todo_sync_id, log.completed_on);

            let remote = RemoteCompletionLog {
                id: log_id,
                user_id: user_id.to_string(),
                todo_id: log.todo_sync_id.clone(),
                completed_on: log.completed_on.clone(),
                completed_count: log.completed_count as i32,
            };

            if let Err(error) = client.upsert_completion_log(access_token, &remote).await {
                if Self::is_missing_remote_todo_error(&error) {
                    Self::repair_missing_remote_todo_async(
                        client,
                        access_token,
                        &log.todo_sync_id,
                        todos,
                        categories,
                    )
                    .await?;
                    client.upsert_completion_log(access_token, &remote).await?;
                } else {
                    return Err(error);
                }
            }
            count += 1;
        }

        Ok(count)
    }
}
