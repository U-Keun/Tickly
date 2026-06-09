use rusqlite::{params, Connection};

use crate::models::{V2Category, V2ItemSearchResult, V2TodoItem};

pub struct V2ChecklistRepository;

impl V2ChecklistRepository {
    const ORDER_STEP: i64 = 1000;
    const CATEGORY_COLUMNS: &'static str = "id, name, display_order, created_at, updated_at";
    const ITEM_COLUMNS: &'static str =
        "id, category_id, text, done, display_order, created_at, updated_at";

    pub fn create_tables(conn: &Connection) -> Result<(), rusqlite::Error> {
        conn.execute_batch(
            "CREATE TABLE IF NOT EXISTS v2_categories (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                name TEXT NOT NULL UNIQUE,
                display_order INTEGER NOT NULL,
                created_at TEXT NOT NULL,
                updated_at TEXT NOT NULL
            );

            CREATE TABLE IF NOT EXISTS v2_todos (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                category_id INTEGER NOT NULL,
                text TEXT NOT NULL,
                done BOOLEAN NOT NULL DEFAULT 0,
                display_order INTEGER NOT NULL,
                created_at TEXT NOT NULL,
                updated_at TEXT NOT NULL,
                FOREIGN KEY (category_id) REFERENCES v2_categories(id) ON DELETE CASCADE
            );

            CREATE INDEX IF NOT EXISTS idx_v2_todos_category_order
                ON v2_todos(category_id, done, display_order);",
        )?;

        Ok(())
    }

    pub fn ensure_default_category(conn: &Connection) -> Result<(), rusqlite::Error> {
        let category_count: i64 =
            conn.query_row("SELECT COUNT(*) FROM v2_categories", [], |row| row.get(0))?;

        if category_count == 0 {
            let now = Self::now_iso();
            conn.execute(
                "INSERT INTO v2_categories (name, display_order, created_at, updated_at)
                 VALUES (?1, ?2, ?3, ?4)",
                params!["Home", Self::ORDER_STEP, &now, &now],
            )?;
        }

        Ok(())
    }

    fn now_iso() -> String {
        chrono::Utc::now().format("%Y-%m-%dT%H:%M:%SZ").to_string()
    }

    fn row_to_category(row: &rusqlite::Row) -> Result<V2Category, rusqlite::Error> {
        Ok(V2Category {
            id: row.get(0)?,
            name: row.get(1)?,
            display_order: row.get(2)?,
            created_at: row.get(3)?,
            updated_at: row.get(4)?,
        })
    }

    fn row_to_item(row: &rusqlite::Row) -> Result<V2TodoItem, rusqlite::Error> {
        Ok(V2TodoItem {
            id: row.get(0)?,
            category_id: row.get(1)?,
            text: row.get(2)?,
            done: row.get(3)?,
            display_order: row.get(4)?,
            created_at: row.get(5)?,
            updated_at: row.get(6)?,
        })
    }

    fn row_to_search_result(row: &rusqlite::Row) -> Result<V2ItemSearchResult, rusqlite::Error> {
        Ok(V2ItemSearchResult {
            item: V2TodoItem {
                id: row.get(0)?,
                category_id: row.get(1)?,
                text: row.get(2)?,
                done: row.get(3)?,
                display_order: row.get(4)?,
                created_at: row.get(5)?,
                updated_at: row.get(6)?,
            },
            category: V2Category {
                id: row.get(7)?,
                name: row.get(8)?,
                display_order: row.get(9)?,
                created_at: row.get(10)?,
                updated_at: row.get(11)?,
            },
        })
    }

    fn like_pattern(query: &str) -> String {
        let mut pattern = String::from("%");
        for character in query.chars() {
            if matches!(character, '%' | '_' | '\\') {
                pattern.push('\\');
            }
            pattern.push(character);
        }
        pattern.push('%');
        pattern
    }

    pub fn count_categories(conn: &Connection) -> Result<i64, rusqlite::Error> {
        conn.query_row("SELECT COUNT(*) FROM v2_categories", [], |row| row.get(0))
    }

    pub fn get_categories(conn: &Connection) -> Result<Vec<V2Category>, rusqlite::Error> {
        let sql = format!(
            "SELECT {} FROM v2_categories ORDER BY display_order ASC",
            Self::CATEGORY_COLUMNS
        );
        let mut stmt = conn.prepare(&sql)?;
        let categories = stmt
            .query_map([], Self::row_to_category)?
            .collect::<Result<Vec<_>, _>>()?;

        Ok(categories)
    }

    pub fn get_category_by_id(
        conn: &Connection,
        id: i64,
    ) -> Result<Option<V2Category>, rusqlite::Error> {
        let sql = format!(
            "SELECT {} FROM v2_categories WHERE id = ?1",
            Self::CATEGORY_COLUMNS
        );
        let mut stmt = conn.prepare(&sql)?;
        let mut rows = stmt.query_map(params![id], Self::row_to_category)?;

        if let Some(category) = rows.next() {
            Ok(Some(category?))
        } else {
            Ok(None)
        }
    }

    pub fn create_category(conn: &Connection, name: &str) -> Result<V2Category, rusqlite::Error> {
        let max_order: i64 = conn
            .query_row(
                "SELECT COALESCE(MAX(display_order), 0) FROM v2_categories",
                [],
                |row| row.get(0),
            )
            .unwrap_or(0);
        let display_order = max_order + Self::ORDER_STEP;
        let now = Self::now_iso();

        conn.execute(
            "INSERT INTO v2_categories (name, display_order, created_at, updated_at)
             VALUES (?1, ?2, ?3, ?4)",
            params![name, display_order, &now, &now],
        )?;

        Ok(V2Category {
            id: conn.last_insert_rowid(),
            name: name.to_string(),
            display_order,
            created_at: now.clone(),
            updated_at: now,
        })
    }

    pub fn update_category(conn: &Connection, id: i64, name: &str) -> Result<(), rusqlite::Error> {
        let now = Self::now_iso();
        let updated = conn.execute(
            "UPDATE v2_categories SET name = ?1, updated_at = ?2 WHERE id = ?3",
            params![name, now, id],
        )?;
        if updated == 0 {
            return Err(rusqlite::Error::QueryReturnedNoRows);
        }
        Ok(())
    }

    pub fn delete_category(conn: &Connection, id: i64) -> Result<(), rusqlite::Error> {
        conn.execute("BEGIN TRANSACTION", [])?;

        if let Err(error) = conn.execute("DELETE FROM v2_todos WHERE category_id = ?1", params![id])
        {
            let _ = conn.execute("ROLLBACK", []);
            return Err(error);
        }

        match conn.execute("DELETE FROM v2_categories WHERE id = ?1", params![id]) {
            Ok(0) => {
                let _ = conn.execute("ROLLBACK", []);
                Err(rusqlite::Error::QueryReturnedNoRows)
            }
            Ok(_) => {
                conn.execute("COMMIT", [])?;
                Ok(())
            }
            Err(error) => {
                let _ = conn.execute("ROLLBACK", []);
                Err(error)
            }
        }
    }

    pub fn reorder_categories(
        conn: &Connection,
        category_ids: &[i64],
    ) -> Result<(), rusqlite::Error> {
        conn.execute("BEGIN TRANSACTION", [])?;
        let now = Self::now_iso();

        for (index, category_id) in category_ids.iter().enumerate() {
            let display_order = (index as i64 + 1) * Self::ORDER_STEP;
            match conn.execute(
                "UPDATE v2_categories
                 SET display_order = ?1, updated_at = ?2
                 WHERE id = ?3",
                params![display_order, &now, category_id],
            ) {
                Ok(1) => {}
                Ok(_) => {
                    let _ = conn.execute("ROLLBACK", []);
                    return Err(rusqlite::Error::QueryReturnedNoRows);
                }
                Err(error) => {
                    let _ = conn.execute("ROLLBACK", []);
                    return Err(error);
                }
            }
        }

        conn.execute("COMMIT", [])?;
        Ok(())
    }

    pub fn get_items(
        conn: &Connection,
        category_id: i64,
    ) -> Result<Vec<V2TodoItem>, rusqlite::Error> {
        let sql = format!(
            "SELECT {} FROM v2_todos
             WHERE category_id = ?1
             ORDER BY done ASC, display_order ASC",
            Self::ITEM_COLUMNS
        );
        let mut stmt = conn.prepare(&sql)?;
        let items = stmt
            .query_map(params![category_id], Self::row_to_item)?
            .collect::<Result<Vec<_>, _>>()?;

        Ok(items)
    }

    pub fn search_items(
        conn: &Connection,
        query: &str,
        limit: i64,
    ) -> Result<Vec<V2ItemSearchResult>, rusqlite::Error> {
        let pattern = Self::like_pattern(query);
        let sql = "\
            SELECT
                t.id,
                t.category_id,
                t.text,
                t.done,
                t.display_order,
                t.created_at,
                t.updated_at,
                c.id,
                c.name,
                c.display_order,
                c.created_at,
                c.updated_at
             FROM v2_todos t
             INNER JOIN v2_categories c ON c.id = t.category_id
             WHERE t.text LIKE ?1 ESCAPE '\\'
             ORDER BY c.display_order ASC, t.done ASC, t.display_order ASC
             LIMIT ?2";
        let mut stmt = conn.prepare(&sql)?;
        let results = stmt
            .query_map(params![pattern, limit], Self::row_to_search_result)?
            .collect::<Result<Vec<_>, _>>()?;

        Ok(results)
    }

    pub fn get_item_by_id(
        conn: &Connection,
        id: i64,
    ) -> Result<Option<V2TodoItem>, rusqlite::Error> {
        let sql = format!("SELECT {} FROM v2_todos WHERE id = ?1", Self::ITEM_COLUMNS);
        let mut stmt = conn.prepare(&sql)?;
        let mut rows = stmt.query_map(params![id], Self::row_to_item)?;

        if let Some(item) = rows.next() {
            Ok(Some(item?))
        } else {
            Ok(None)
        }
    }

    pub fn create_item(
        conn: &Connection,
        category_id: i64,
        text: &str,
    ) -> Result<V2TodoItem, rusqlite::Error> {
        let max_order: i64 = conn
            .query_row(
                "SELECT COALESCE(MAX(display_order), 0)
                 FROM v2_todos
                 WHERE category_id = ?1",
                params![category_id],
                |row| row.get(0),
            )
            .unwrap_or(0);
        let display_order = max_order + Self::ORDER_STEP;
        let now = Self::now_iso();

        conn.execute(
            "INSERT INTO v2_todos
                (category_id, text, done, display_order, created_at, updated_at)
             VALUES (?1, ?2, 0, ?3, ?4, ?5)",
            params![category_id, text, display_order, &now, &now],
        )?;

        Ok(V2TodoItem {
            id: conn.last_insert_rowid(),
            category_id,
            text: text.to_string(),
            done: false,
            display_order,
            created_at: now.clone(),
            updated_at: now,
        })
    }

    pub fn update_item_text(conn: &Connection, id: i64, text: &str) -> Result<(), rusqlite::Error> {
        let now = Self::now_iso();
        let updated = conn.execute(
            "UPDATE v2_todos SET text = ?1, updated_at = ?2 WHERE id = ?3",
            params![text, now, id],
        )?;
        if updated == 0 {
            return Err(rusqlite::Error::QueryReturnedNoRows);
        }
        Ok(())
    }

    pub fn set_item_done(conn: &Connection, id: i64, done: bool) -> Result<(), rusqlite::Error> {
        let now = Self::now_iso();
        let updated = conn.execute(
            "UPDATE v2_todos SET done = ?1, updated_at = ?2 WHERE id = ?3",
            params![done, now, id],
        )?;
        if updated == 0 {
            return Err(rusqlite::Error::QueryReturnedNoRows);
        }
        Ok(())
    }

    pub fn delete_item(conn: &Connection, id: i64) -> Result<(), rusqlite::Error> {
        let updated = conn.execute("DELETE FROM v2_todos WHERE id = ?1", params![id])?;
        if updated == 0 {
            return Err(rusqlite::Error::QueryReturnedNoRows);
        }
        Ok(())
    }

    pub fn reorder_items(
        conn: &Connection,
        category_id: i64,
        item_ids: &[i64],
    ) -> Result<(), rusqlite::Error> {
        conn.execute("BEGIN TRANSACTION", [])?;
        let now = Self::now_iso();

        for (index, item_id) in item_ids.iter().enumerate() {
            let display_order = (index as i64 + 1) * Self::ORDER_STEP;
            match conn.execute(
                "UPDATE v2_todos
                 SET display_order = ?1, updated_at = ?2
                 WHERE id = ?3 AND category_id = ?4",
                params![display_order, &now, item_id, category_id],
            ) {
                Ok(1) => {}
                Ok(_) => {
                    let _ = conn.execute("ROLLBACK", []);
                    return Err(rusqlite::Error::QueryReturnedNoRows);
                }
                Err(error) => {
                    let _ = conn.execute("ROLLBACK", []);
                    return Err(error);
                }
            }
        }

        conn.execute("COMMIT", [])?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn setup_conn() -> Connection {
        let conn = Connection::open_in_memory().expect("in-memory db");
        V2ChecklistRepository::create_tables(&conn).expect("v2 schema");
        V2ChecklistRepository::ensure_default_category(&conn).expect("default category");
        conn
    }

    #[test]
    fn creates_default_home_category() {
        let conn = setup_conn();
        let categories = V2ChecklistRepository::get_categories(&conn).expect("categories");

        assert_eq!(categories.len(), 1);
        assert_eq!(categories[0].name, "Home");
        assert_eq!(categories[0].display_order, 1000);
    }

    #[test]
    fn keeps_done_items_after_pending_items() {
        let conn = setup_conn();
        let category_id = V2ChecklistRepository::get_categories(&conn).unwrap()[0].id;
        let first = V2ChecklistRepository::create_item(&conn, category_id, "First").unwrap();
        let second = V2ChecklistRepository::create_item(&conn, category_id, "Second").unwrap();

        V2ChecklistRepository::set_item_done(&conn, first.id, true).unwrap();
        let items = V2ChecklistRepository::get_items(&conn, category_id).unwrap();

        assert_eq!(items[0].id, second.id);
        assert_eq!(items[1].id, first.id);
    }

    #[test]
    fn reorders_categories_with_order_step() {
        let conn = setup_conn();
        let home = V2ChecklistRepository::get_categories(&conn).unwrap()[0].clone();
        let work = V2ChecklistRepository::create_category(&conn, "Work").unwrap();

        V2ChecklistRepository::reorder_categories(&conn, &[work.id, home.id]).unwrap();
        let categories = V2ChecklistRepository::get_categories(&conn).unwrap();

        assert_eq!(categories[0].id, work.id);
        assert_eq!(categories[0].display_order, 1000);
        assert_eq!(categories[1].id, home.id);
        assert_eq!(categories[1].display_order, 2000);
    }
}
