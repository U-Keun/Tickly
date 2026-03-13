use rusqlite::{params, Connection};

use crate::models::{Category, SyncStatus};

pub struct CategoryRepository;

impl CategoryRepository {
    const SELECT_COLUMNS: &'static str =
        "id, name, display_order, sync_id, created_at, updated_at, sync_status";

    fn row_to_category(row: &rusqlite::Row) -> Result<Category, rusqlite::Error> {
        let sync_status_str: Option<String> = row.get(6)?;
        Ok(Category {
            id: row.get(0)?,
            name: row.get(1)?,
            display_order: row.get(2)?,
            sync_id: row.get(3)?,
            created_at: row.get(4)?,
            updated_at: row.get(5)?,
            sync_status: sync_status_str
                .map(|s| SyncStatus::from_str(&s))
                .unwrap_or_default(),
        })
    }

    pub fn get_all(conn: &Connection) -> Result<Vec<Category>, rusqlite::Error> {
        let sql = format!(
            "SELECT {} FROM categories WHERE sync_status != 'deleted' OR sync_status IS NULL ORDER BY display_order ASC",
            Self::SELECT_COLUMNS
        );
        let mut stmt = conn.prepare(&sql)?;

        let categories = stmt
            .query_map([], Self::row_to_category)?
            .collect::<Result<Vec<_>, _>>()?;

        Ok(categories)
    }

    /// Get all categories including deleted ones (for sync purposes)
    pub fn get_all_including_deleted(conn: &Connection) -> Result<Vec<Category>, rusqlite::Error> {
        let sql = format!(
            "SELECT {} FROM categories ORDER BY display_order ASC",
            Self::SELECT_COLUMNS
        );
        let mut stmt = conn.prepare(&sql)?;

        let categories = stmt
            .query_map([], Self::row_to_category)?
            .collect::<Result<Vec<_>, _>>()?;

        Ok(categories)
    }

    pub fn get_by_id(conn: &Connection, id: i64) -> Result<Option<Category>, rusqlite::Error> {
        let sql = format!(
            "SELECT {} FROM categories WHERE id = ?1",
            Self::SELECT_COLUMNS
        );
        let mut stmt = conn.prepare(&sql)?;

        let mut rows = stmt.query_map(params![id], Self::row_to_category)?;

        if let Some(cat) = rows.next() {
            Ok(Some(cat?))
        } else {
            Ok(None)
        }
    }

    #[allow(dead_code)]
    pub fn get_by_sync_id(
        conn: &Connection,
        sync_id: &str,
    ) -> Result<Option<Category>, rusqlite::Error> {
        let sql = format!(
            "SELECT {} FROM categories WHERE sync_id = ?1",
            Self::SELECT_COLUMNS
        );
        let mut stmt = conn.prepare(&sql)?;

        let mut rows = stmt.query_map(params![sync_id], Self::row_to_category)?;

        if let Some(cat) = rows.next() {
            Ok(Some(cat?))
        } else {
            Ok(None)
        }
    }

    pub fn get_pending_sync(conn: &Connection) -> Result<Vec<Category>, rusqlite::Error> {
        let sql = format!(
            "SELECT {} FROM categories WHERE sync_status = 'pending' OR sync_status = 'deleted' OR sync_status IS NULL",
            Self::SELECT_COLUMNS
        );
        let mut stmt = conn.prepare(&sql)?;

        let categories = stmt
            .query_map([], Self::row_to_category)?
            .collect::<Result<Vec<_>, _>>()?;

        Ok(categories)
    }

    fn mark_updated(conn: &Connection, id: i64) -> Result<(), rusqlite::Error> {
        let now = chrono::Utc::now().format("%Y-%m-%dT%H:%M:%SZ").to_string();
        conn.execute(
            "UPDATE categories SET updated_at = ?1, sync_status = 'pending' WHERE id = ?2",
            params![now, id],
        )?;
        Ok(())
    }

    pub fn create(conn: &Connection, name: &str) -> Result<Category, rusqlite::Error> {
        let max_order: i64 = conn
            .query_row(
                "SELECT COALESCE(MAX(display_order), 0) FROM categories",
                [],
                |row| row.get(0),
            )
            .unwrap_or(0);

        let display_order = max_order + 1000;
        let now = chrono::Utc::now().format("%Y-%m-%dT%H:%M:%SZ").to_string();

        conn.execute(
            "INSERT INTO categories (name, display_order, created_at, updated_at, sync_status) VALUES (?1, ?2, ?3, ?4, 'pending')",
            params![name, display_order, &now, &now],
        )?;

        let id = conn.last_insert_rowid();

        Ok(Category {
            id,
            name: name.to_string(),
            display_order,
            sync_id: None,
            created_at: Some(now.clone()),
            updated_at: Some(now),
            sync_status: SyncStatus::Pending,
        })
    }

    pub fn update(conn: &Connection, id: i64, name: &str) -> Result<(), rusqlite::Error> {
        conn.execute(
            "UPDATE categories SET name = ?1 WHERE id = ?2",
            params![name, id],
        )?;
        Self::mark_updated(conn, id)?;
        Ok(())
    }

    pub fn delete(conn: &Connection, id: i64) -> Result<(), rusqlite::Error> {
        // Delete all todos in this category
        conn.execute("DELETE FROM todos WHERE category_id = ?1", params![id])?;

        // Delete the category
        conn.execute("DELETE FROM categories WHERE id = ?1", params![id])?;

        Ok(())
    }

    pub fn mark_deleted(conn: &Connection, id: i64) -> Result<(), rusqlite::Error> {
        let now = chrono::Utc::now().format("%Y-%m-%dT%H:%M:%SZ").to_string();
        conn.execute(
            "UPDATE categories SET sync_status = 'deleted', updated_at = ?1 WHERE id = ?2",
            params![now, id],
        )?;
        Ok(())
    }

    pub fn reorder(conn: &Connection, category_ids: &[i64]) -> Result<(), rusqlite::Error> {
        conn.execute("BEGIN TRANSACTION", [])?;

        let now = chrono::Utc::now().format("%Y-%m-%dT%H:%M:%SZ").to_string();
        for (index, category_id) in category_ids.iter().enumerate() {
            let new_order = (index as i64 + 1) * 1000;
            if let Err(e) = conn.execute(
                "UPDATE categories SET display_order = ?1, updated_at = ?2, sync_status = 'pending' WHERE id = ?3",
                params![new_order, &now, category_id],
            ) {
                let _ = conn.execute("ROLLBACK", []);
                return Err(e);
            }
        }

        conn.execute("COMMIT", [])?;

        Ok(())
    }

    pub fn update_sync_id(
        conn: &Connection,
        id: i64,
        sync_id: &str,
    ) -> Result<(), rusqlite::Error> {
        conn.execute(
            "UPDATE categories SET sync_id = ?1, sync_status = 'synced' WHERE id = ?2",
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
            "UPDATE categories SET sync_id = ?1 WHERE id = ?2",
            params![sync_id, id],
        )?;
        Ok(())
    }

    pub fn mark_synced(conn: &Connection, id: i64) -> Result<(), rusqlite::Error> {
        conn.execute(
            "UPDATE categories SET sync_status = 'synced' WHERE id = ?1",
            params![id],
        )?;
        Ok(())
    }
}
