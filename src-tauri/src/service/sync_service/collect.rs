use super::*;

impl SyncService {
    pub(super) fn collect_pending_categories(
        conn: &Connection,
    ) -> Result<Vec<PendingCategorySync>, String> {
        let categories = CategoryRepository::get_pending_sync(conn).map_err(|e| e.to_string())?;
        Ok(categories
            .into_iter()
            .map(|c| PendingCategorySync {
                id: c.id,
                sync_id: c.sync_id,
                name: c.name,
                display_order: c.display_order,
                created_at: c.created_at,
                updated_at: c.updated_at,
                sync_status: c.sync_status,
            })
            .collect())
    }

    #[allow(dead_code)]
    fn collect_pending_todos(conn: &Connection) -> Result<Vec<PendingTodoSync>, String> {
        let cat_sync_map: HashMap<i64, String> = CategoryRepository::get_all(conn)
            .map_err(|e| e.to_string())?
            .iter()
            .filter_map(|c| c.sync_id.as_ref().map(|s| (c.id, s.clone())))
            .collect();

        Self::collect_pending_todos_with_map(conn, &cat_sync_map)
    }

    pub(super) fn collect_pending_todos_with_map(
        conn: &Connection,
        cat_sync_map: &HashMap<i64, String>,
    ) -> Result<Vec<PendingTodoSync>, String> {
        let todos = TodoRepository::get_pending_sync(conn).map_err(|e| e.to_string())?;

        Ok(todos
            .into_iter()
            .map(|t| PendingTodoSync {
                id: t.id,
                sync_id: t.sync_id,
                category_sync_id: t
                    .category_id
                    .and_then(|cid| cat_sync_map.get(&cid).cloned()),
                text: t.text,
                done: t.done,
                display_order: t.display_order,
                memo: t.memo,
                repeat_type: t.repeat_type.to_str().to_string(),
                repeat_detail: t.repeat_detail,
                next_due_at: t.next_due_at,
                last_completed_at: t.last_completed_at,
                track_streak: t.track_streak,
                reminder_at: t.reminder_at,
                linked_app: t.linked_app,
                created_at: t.created_at,
                updated_at: t.updated_at,
                sync_status: t.sync_status,
            })
            .collect())
    }

    pub(super) fn collect_pending_tags(conn: &Connection) -> Result<Vec<PendingTagSync>, String> {
        let tags = TagRepository::get_pending_sync(conn).map_err(|e| e.to_string())?;
        Ok(tags
            .into_iter()
            .map(|t| PendingTagSync {
                id: t.id,
                sync_id: t.sync_id,
                name: t.name,
                created_at: t.created_at,
                updated_at: t.updated_at,
                sync_status: t.sync_status,
            })
            .collect())
    }

    pub(super) fn collect_pending_todo_tags(
        conn: &Connection,
        todo_id_to_sync_id: &HashMap<i64, String>,
        tag_id_to_sync_id: &HashMap<i64, String>,
    ) -> Result<Vec<PendingTodoTagSync>, String> {
        let todo_tags = TodoTagRepository::get_pending_sync(conn).map_err(|e| e.to_string())?;
        Ok(todo_tags
            .into_iter()
            .filter_map(|tt| {
                let todo_sync_id = todo_id_to_sync_id.get(&tt.todo_id).cloned();
                let tag_sync_id = tag_id_to_sync_id.get(&tt.tag_id).cloned();
                if todo_sync_id.is_none() || tag_sync_id.is_none() {
                    return None;
                }
                Some(PendingTodoTagSync {
                    todo_id: tt.todo_id,
                    tag_id: tt.tag_id,
                    sync_id: tt.sync_id,
                    todo_sync_id,
                    tag_sync_id,
                    created_at: tt.created_at,
                    sync_status: tt.sync_status,
                })
            })
            .collect())
    }

    pub(super) fn collect_completion_logs(
        conn: &Connection,
        todo_id_to_sync_id: &HashMap<i64, String>,
    ) -> Result<Vec<LocalCompletionLogSync>, String> {
        let logs = CompletionLogRepository::get_all(conn).map_err(|e| e.to_string())?;

        Ok(logs
            .into_iter()
            .filter_map(|log| {
                todo_id_to_sync_id
                    .get(&log.item_id)
                    .map(|sync_id| LocalCompletionLogSync {
                        item_id: log.item_id,
                        todo_sync_id: sync_id.clone(),
                        completed_on: log.completed_on,
                        completed_count: log.completed_count,
                    })
            })
            .collect())
    }
}
