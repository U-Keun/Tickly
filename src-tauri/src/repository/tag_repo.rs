use rusqlite::{params, Connection};

use crate::models::{SyncStatus, Tag};

pub struct TagRepository;

impl TagRepository {
    const SELECT_COLUMNS: &'static str = "id, name, sync_id, created_at, updated_at, sync_status";

    fn row_to_tag(row: &rusqlite::Row) -> Result<Tag, rusqlite::Error> {
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
    }

    pub fn get_all(conn: &Connection) -> Result<Vec<Tag>, rusqlite::Error> {
        let sql = format!(
            "SELECT {} FROM tags WHERE sync_status != 'deleted' OR sync_status IS NULL ORDER BY name ASC",
            Self::SELECT_COLUMNS
        );
        let mut stmt = conn.prepare(&sql)?;
        let tags = stmt
            .query_map([], Self::row_to_tag)?
            .collect::<Result<Vec<_>, _>>()?;
        Ok(tags)
    }

    pub fn get_by_id(conn: &Connection, id: i64) -> Result<Option<Tag>, rusqlite::Error> {
        let sql = format!("SELECT {} FROM tags WHERE id = ?1", Self::SELECT_COLUMNS);
        let mut stmt = conn.prepare(&sql)?;
        let mut rows = stmt.query_map(params![id], Self::row_to_tag)?;
        if let Some(tag) = rows.next() {
            Ok(Some(tag?))
        } else {
            Ok(None)
        }
    }

    pub fn get_by_name(conn: &Connection, name: &str) -> Result<Option<Tag>, rusqlite::Error> {
        let sql = format!(
            "SELECT {} FROM tags WHERE name = ?1 AND (sync_status != 'deleted' OR sync_status IS NULL)",
            Self::SELECT_COLUMNS
        );
        let mut stmt = conn.prepare(&sql)?;
        let mut rows = stmt.query_map(params![name], Self::row_to_tag)?;
        if let Some(tag) = rows.next() {
            Ok(Some(tag?))
        } else {
            Ok(None)
        }
    }

    pub fn create(conn: &Connection, name: &str) -> Result<Tag, rusqlite::Error> {
        let now = chrono::Utc::now().format("%Y-%m-%dT%H:%M:%SZ").to_string();
        conn.execute(
            "INSERT INTO tags (name, created_at, updated_at, sync_status) VALUES (?1, ?2, ?3, 'pending')",
            params![name, &now, &now],
        )?;
        let id = conn.last_insert_rowid();
        Ok(Tag {
            id,
            name: name.to_string(),
            sync_id: None,
            created_at: Some(now.clone()),
            updated_at: Some(now),
            sync_status: SyncStatus::Pending,
        })
    }

    pub fn delete(conn: &Connection, id: i64) -> Result<(), rusqlite::Error> {
        // Delete all todo_tags associations first
        conn.execute("DELETE FROM todo_tags WHERE tag_id = ?1", params![id])?;
        conn.execute("DELETE FROM tags WHERE id = ?1", params![id])?;
        Ok(())
    }

    pub fn mark_deleted(conn: &Connection, id: i64) -> Result<(), rusqlite::Error> {
        let now = chrono::Utc::now().format("%Y-%m-%dT%H:%M:%SZ").to_string();
        conn.execute(
            "UPDATE tags SET sync_status = 'deleted', updated_at = ?1 WHERE id = ?2",
            params![now, id],
        )?;
        // Also mark all todo_tags for this tag as deleted
        conn.execute(
            "UPDATE todo_tags SET sync_status = 'deleted' WHERE tag_id = ?1 AND sync_id IS NOT NULL",
            params![id],
        )?;
        conn.execute(
            "DELETE FROM todo_tags WHERE tag_id = ?1 AND sync_id IS NULL",
            params![id],
        )?;
        Ok(())
    }

    pub fn get_pending_sync(conn: &Connection) -> Result<Vec<Tag>, rusqlite::Error> {
        let sql = format!(
            "SELECT {} FROM tags WHERE sync_status = 'pending' OR sync_status = 'deleted' OR sync_status IS NULL",
            Self::SELECT_COLUMNS
        );
        let mut stmt = conn.prepare(&sql)?;
        let tags = stmt
            .query_map([], Self::row_to_tag)?
            .collect::<Result<Vec<_>, _>>()?;
        Ok(tags)
    }

    pub fn get_by_sync_id(
        conn: &Connection,
        sync_id: &str,
    ) -> Result<Option<Tag>, rusqlite::Error> {
        let sql = format!(
            "SELECT {} FROM tags WHERE sync_id = ?1",
            Self::SELECT_COLUMNS
        );
        let mut stmt = conn.prepare(&sql)?;
        let mut rows = stmt.query_map(params![sync_id], Self::row_to_tag)?;
        if let Some(tag) = rows.next() {
            Ok(Some(tag?))
        } else {
            Ok(None)
        }
    }

    pub fn update_sync_id(
        conn: &Connection,
        id: i64,
        sync_id: &str,
    ) -> Result<(), rusqlite::Error> {
        conn.execute(
            "UPDATE tags SET sync_id = ?1, sync_status = 'synced' WHERE id = ?2",
            params![sync_id, id],
        )?;
        Ok(())
    }

    pub fn assign_sync_id(
        conn: &Connection,
        id: i64,
        sync_id: &str,
    ) -> Result<(), rusqlite::Error> {
        conn.execute(
            "UPDATE tags SET sync_id = ?1 WHERE id = ?2",
            params![sync_id, id],
        )?;
        Ok(())
    }

    pub fn mark_synced(conn: &Connection, id: i64) -> Result<(), rusqlite::Error> {
        conn.execute(
            "UPDATE tags SET sync_status = 'synced' WHERE id = ?1",
            params![id],
        )?;
        Ok(())
    }
}
