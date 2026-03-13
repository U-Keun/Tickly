use rusqlite::{params, Connection};

use crate::models::{SyncStatus, Tag, TodoTag};

pub struct TodoTagRepository;

impl TodoTagRepository {
    fn row_to_todo_tag(row: &rusqlite::Row) -> Result<TodoTag, rusqlite::Error> {
        let sync_status_str: Option<String> = row.get(4)?;
        Ok(TodoTag {
            todo_id: row.get(0)?,
            tag_id: row.get(1)?,
            sync_id: row.get(2)?,
            created_at: row.get(3)?,
            sync_status: sync_status_str
                .map(|s| SyncStatus::from_str(&s))
                .unwrap_or_default(),
        })
    }

    pub fn add_tag(conn: &Connection, todo_id: i64, tag_id: i64) -> Result<(), rusqlite::Error> {
        let now = chrono::Utc::now().format("%Y-%m-%dT%H:%M:%SZ").to_string();
        conn.execute(
            "INSERT OR IGNORE INTO todo_tags (todo_id, tag_id, created_at, sync_status) VALUES (?1, ?2, ?3, 'pending')",
            params![todo_id, tag_id, &now],
        )?;
        Ok(())
    }

    pub fn remove_tag(conn: &Connection, todo_id: i64, tag_id: i64) -> Result<(), rusqlite::Error> {
        conn.execute(
            "DELETE FROM todo_tags WHERE todo_id = ?1 AND tag_id = ?2",
            params![todo_id, tag_id],
        )?;
        Ok(())
    }

    pub fn mark_deleted(
        conn: &Connection,
        todo_id: i64,
        tag_id: i64,
    ) -> Result<(), rusqlite::Error> {
        conn.execute(
            "UPDATE todo_tags SET sync_status = 'deleted' WHERE todo_id = ?1 AND tag_id = ?2",
            params![todo_id, tag_id],
        )?;
        Ok(())
    }

    pub fn get_tags_for_item(conn: &Connection, todo_id: i64) -> Result<Vec<Tag>, rusqlite::Error> {
        let mut stmt = conn.prepare(
            "SELECT t.id, t.name, t.sync_id, t.created_at, t.updated_at, t.sync_status
             FROM tags t
             INNER JOIN todo_tags tt ON t.id = tt.tag_id
             WHERE tt.todo_id = ?1
               AND (tt.sync_status != 'deleted' OR tt.sync_status IS NULL)
               AND (t.sync_status != 'deleted' OR t.sync_status IS NULL)
             ORDER BY t.name ASC",
        )?;
        let tags = stmt
            .query_map(params![todo_id], |row| {
                let sync_status_str: Option<String> = row.get(5)?;
                Ok(Tag {
                    id: row.get(0)?,
                    name: row.get(1)?,
                    sync_id: row.get(2)?,
                    created_at: row.get(3)?,
                    updated_at: row.get(4)?,
                    sync_status: sync_status_str
                        .map(|s| SyncStatus::from_str(&s))
                        .unwrap_or_default(),
                })
            })?
            .collect::<Result<Vec<_>, _>>()?;
        Ok(tags)
    }

    pub fn get_todo_ids_by_tag(
        conn: &Connection,
        tag_id: i64,
    ) -> Result<Vec<i64>, rusqlite::Error> {
        let mut stmt = conn.prepare(
            "SELECT tt.todo_id FROM todo_tags tt
             INNER JOIN todos t ON t.id = tt.todo_id
             WHERE tt.tag_id = ?1
               AND (tt.sync_status != 'deleted' OR tt.sync_status IS NULL)
               AND (t.sync_status != 'deleted' OR t.sync_status IS NULL)",
        )?;
        let ids = stmt
            .query_map(params![tag_id], |row| row.get(0))?
            .collect::<Result<Vec<i64>, _>>()?;
        Ok(ids)
    }

    pub fn get_pending_sync(conn: &Connection) -> Result<Vec<TodoTag>, rusqlite::Error> {
        let mut stmt = conn.prepare(
            "SELECT todo_id, tag_id, sync_id, created_at, sync_status FROM todo_tags WHERE sync_status = 'pending' OR sync_status = 'deleted' OR sync_status IS NULL",
        )?;
        let todo_tags = stmt
            .query_map([], Self::row_to_todo_tag)?
            .collect::<Result<Vec<_>, _>>()?;
        Ok(todo_tags)
    }

    pub fn get_by_sync_id(
        conn: &Connection,
        sync_id: &str,
    ) -> Result<Option<TodoTag>, rusqlite::Error> {
        let mut stmt = conn.prepare(
            "SELECT todo_id, tag_id, sync_id, created_at, sync_status FROM todo_tags WHERE sync_id = ?1",
        )?;
        let mut rows = stmt.query_map(params![sync_id], Self::row_to_todo_tag)?;
        if let Some(tt) = rows.next() {
            Ok(Some(tt?))
        } else {
            Ok(None)
        }
    }

    pub fn update_sync_id(
        conn: &Connection,
        todo_id: i64,
        tag_id: i64,
        sync_id: &str,
    ) -> Result<(), rusqlite::Error> {
        conn.execute(
            "UPDATE todo_tags SET sync_id = ?1, sync_status = 'synced' WHERE todo_id = ?2 AND tag_id = ?3",
            params![sync_id, todo_id, tag_id],
        )?;
        Ok(())
    }

    pub fn assign_sync_id(
        conn: &Connection,
        todo_id: i64,
        tag_id: i64,
        sync_id: &str,
    ) -> Result<(), rusqlite::Error> {
        conn.execute(
            "UPDATE todo_tags SET sync_id = ?1 WHERE todo_id = ?2 AND tag_id = ?3",
            params![sync_id, todo_id, tag_id],
        )?;
        Ok(())
    }

    #[allow(dead_code)]
    pub fn mark_synced(
        conn: &Connection,
        todo_id: i64,
        tag_id: i64,
    ) -> Result<(), rusqlite::Error> {
        conn.execute(
            "UPDATE todo_tags SET sync_status = 'synced' WHERE todo_id = ?1 AND tag_id = ?2",
            params![todo_id, tag_id],
        )?;
        Ok(())
    }

    #[allow(dead_code)]
    pub fn delete(conn: &Connection, todo_id: i64, tag_id: i64) -> Result<(), rusqlite::Error> {
        conn.execute(
            "DELETE FROM todo_tags WHERE todo_id = ?1 AND tag_id = ?2",
            params![todo_id, tag_id],
        )?;
        Ok(())
    }
}
