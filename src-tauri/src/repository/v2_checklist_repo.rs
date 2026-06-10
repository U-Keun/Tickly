use rusqlite::{params, Connection, OptionalExtension};

use crate::models::{V2Category, V2ItemSearchResult, V2Tag, V2TodoItem};

pub struct V2ChecklistRepository;

impl V2ChecklistRepository {
    const ORDER_STEP: i64 = 1000;
    const CATEGORY_COLUMNS: &'static str = "id, name, display_order, created_at, updated_at";
    const TAG_COLUMNS: &'static str = "id, name, created_at, updated_at";
    const ITEM_COLUMNS: &'static str =
        "id, category_id, text, memo, done, display_order, created_at, updated_at";

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
                memo TEXT,
                done BOOLEAN NOT NULL DEFAULT 0,
                display_order INTEGER NOT NULL,
                created_at TEXT NOT NULL,
                updated_at TEXT NOT NULL,
                FOREIGN KEY (category_id) REFERENCES v2_categories(id) ON DELETE CASCADE
            );

            CREATE INDEX IF NOT EXISTS idx_v2_todos_category_order
                ON v2_todos(category_id, done, display_order);

            CREATE TABLE IF NOT EXISTS v2_tags (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                name TEXT NOT NULL UNIQUE COLLATE NOCASE,
                created_at TEXT NOT NULL,
                updated_at TEXT NOT NULL
            );

            CREATE TABLE IF NOT EXISTS v2_todo_tags (
                todo_id INTEGER NOT NULL,
                tag_id INTEGER NOT NULL,
                created_at TEXT NOT NULL,
                PRIMARY KEY (todo_id, tag_id),
                FOREIGN KEY (todo_id) REFERENCES v2_todos(id) ON DELETE CASCADE,
                FOREIGN KEY (tag_id) REFERENCES v2_tags(id) ON DELETE CASCADE
            );

            CREATE INDEX IF NOT EXISTS idx_v2_todo_tags_tag_id
                ON v2_todo_tags(tag_id);",
        )?;
        Self::ensure_memo_column(conn)?;

        Ok(())
    }

    fn ensure_memo_column(conn: &Connection) -> Result<(), rusqlite::Error> {
        if Self::v2_todos_has_column(conn, "memo")? {
            return Ok(());
        }

        conn.execute("ALTER TABLE v2_todos ADD COLUMN memo TEXT", [])?;
        Ok(())
    }

    fn v2_todos_has_column(conn: &Connection, column_name: &str) -> Result<bool, rusqlite::Error> {
        let mut stmt = conn.prepare("PRAGMA table_info(v2_todos)")?;
        let columns = stmt.query_map([], |row| row.get::<_, String>(1))?;

        for column in columns {
            if column? == column_name {
                return Ok(true);
            }
        }

        Ok(false)
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
            memo: row.get(3)?,
            tags: Vec::new(),
            done: row.get(4)?,
            display_order: row.get(5)?,
            created_at: row.get(6)?,
            updated_at: row.get(7)?,
        })
    }

    fn row_to_tag(row: &rusqlite::Row) -> Result<V2Tag, rusqlite::Error> {
        Ok(V2Tag {
            id: row.get(0)?,
            name: row.get(1)?,
            created_at: row.get(2)?,
            updated_at: row.get(3)?,
        })
    }

    fn row_to_search_result(row: &rusqlite::Row) -> Result<V2ItemSearchResult, rusqlite::Error> {
        Ok(V2ItemSearchResult {
            item: V2TodoItem {
                id: row.get(0)?,
                category_id: row.get(1)?,
                text: row.get(2)?,
                memo: row.get(3)?,
                tags: Vec::new(),
                done: row.get(4)?,
                display_order: row.get(5)?,
                created_at: row.get(6)?,
                updated_at: row.get(7)?,
            },
            category: V2Category {
                id: row.get(8)?,
                name: row.get(9)?,
                display_order: row.get(10)?,
                created_at: row.get(11)?,
                updated_at: row.get(12)?,
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

    fn attach_tags_to_items(
        conn: &Connection,
        mut items: Vec<V2TodoItem>,
    ) -> Result<Vec<V2TodoItem>, rusqlite::Error> {
        for item in items.iter_mut() {
            item.tags = Self::get_tags_for_item(conn, item.id)?;
        }

        Ok(items)
    }

    fn attach_tags_to_search_results(
        conn: &Connection,
        mut results: Vec<V2ItemSearchResult>,
    ) -> Result<Vec<V2ItemSearchResult>, rusqlite::Error> {
        for result in results.iter_mut() {
            result.item.tags = Self::get_tags_for_item(conn, result.item.id)?;
        }

        Ok(results)
    }

    fn get_tag_by_name(conn: &Connection, name: &str) -> Result<Option<V2Tag>, rusqlite::Error> {
        let sql = format!(
            "SELECT {} FROM v2_tags WHERE name = ?1 COLLATE NOCASE",
            Self::TAG_COLUMNS
        );
        conn.query_row(&sql, params![name], Self::row_to_tag)
            .optional()
    }

    fn get_or_create_tag(conn: &Connection, name: &str) -> Result<V2Tag, rusqlite::Error> {
        if let Some(tag) = Self::get_tag_by_name(conn, name)? {
            return Ok(tag);
        }

        let now = Self::now_iso();
        conn.execute(
            "INSERT OR IGNORE INTO v2_tags (name, created_at, updated_at)
             VALUES (?1, ?2, ?3)",
            params![name, &now, &now],
        )?;

        Self::get_tag_by_name(conn, name)?.ok_or(rusqlite::Error::QueryReturnedNoRows)
    }

    fn replace_item_tags(
        conn: &Connection,
        item_id: i64,
        tag_names: &[String],
    ) -> Result<(), rusqlite::Error> {
        conn.execute(
            "DELETE FROM v2_todo_tags WHERE todo_id = ?1",
            params![item_id],
        )?;

        let now = Self::now_iso();
        for tag_name in tag_names {
            let tag = Self::get_or_create_tag(conn, tag_name)?;
            conn.execute(
                "INSERT OR IGNORE INTO v2_todo_tags (todo_id, tag_id, created_at)
                 VALUES (?1, ?2, ?3)",
                params![item_id, tag.id, &now],
            )?;
        }

        Self::cleanup_unused_tags(conn)?;
        Ok(())
    }

    fn cleanup_unused_tags(conn: &Connection) -> Result<(), rusqlite::Error> {
        conn.execute(
            "DELETE FROM v2_tags
             WHERE id NOT IN (SELECT DISTINCT tag_id FROM v2_todo_tags)",
            [],
        )?;
        Ok(())
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

        if let Err(error) = conn.execute(
            "DELETE FROM v2_todo_tags
             WHERE todo_id IN (SELECT id FROM v2_todos WHERE category_id = ?1)",
            params![id],
        ) {
            let _ = conn.execute("ROLLBACK", []);
            return Err(error);
        }

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
                if let Err(error) = Self::cleanup_unused_tags(conn) {
                    let _ = conn.execute("ROLLBACK", []);
                    return Err(error);
                }
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

    pub fn get_tags(conn: &Connection) -> Result<Vec<V2Tag>, rusqlite::Error> {
        let sql = format!(
            "SELECT {} FROM v2_tags ORDER BY name COLLATE NOCASE ASC",
            Self::TAG_COLUMNS
        );
        let mut stmt = conn.prepare(&sql)?;
        let tags = stmt
            .query_map([], Self::row_to_tag)?
            .collect::<Result<Vec<_>, _>>()?;

        Ok(tags)
    }

    pub fn get_tags_for_item(
        conn: &Connection,
        item_id: i64,
    ) -> Result<Vec<V2Tag>, rusqlite::Error> {
        let sql = "SELECT tg.id, tg.name, tg.created_at, tg.updated_at FROM v2_tags tg
             INNER JOIN v2_todo_tags tt ON tt.tag_id = tg.id
             WHERE tt.todo_id = ?1
             ORDER BY tg.name COLLATE NOCASE ASC";
        let mut stmt = conn.prepare(&sql)?;
        let tags = stmt
            .query_map(params![item_id], Self::row_to_tag)?
            .collect::<Result<Vec<_>, _>>()?;

        Ok(tags)
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

        Self::attach_tags_to_items(conn, items)
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
                t.memo,
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
                OR COALESCE(t.memo, '') LIKE ?2 ESCAPE '\\'
                OR EXISTS (
                    SELECT 1
                    FROM v2_todo_tags tt
                    INNER JOIN v2_tags tg ON tg.id = tt.tag_id
                    WHERE tt.todo_id = t.id
                      AND tg.name LIKE ?3 ESCAPE '\\'
                )
             ORDER BY c.display_order ASC, t.done ASC, t.display_order ASC
             LIMIT ?4";
        let mut stmt = conn.prepare(&sql)?;
        let results = stmt
            .query_map(
                params![pattern, pattern, pattern, limit],
                Self::row_to_search_result,
            )?
            .collect::<Result<Vec<_>, _>>()?;

        Self::attach_tags_to_search_results(conn, results)
    }

    pub fn get_item_by_id(
        conn: &Connection,
        id: i64,
    ) -> Result<Option<V2TodoItem>, rusqlite::Error> {
        let sql = format!("SELECT {} FROM v2_todos WHERE id = ?1", Self::ITEM_COLUMNS);
        let mut stmt = conn.prepare(&sql)?;
        let mut rows = stmt.query_map(params![id], Self::row_to_item)?;

        if let Some(item) = rows.next() {
            let item = item?;
            Ok(Some(
                Self::attach_tags_to_items(conn, vec![item])?
                    .into_iter()
                    .next()
                    .ok_or(rusqlite::Error::QueryReturnedNoRows)?,
            ))
        } else {
            Ok(None)
        }
    }

    pub fn create_item(
        conn: &Connection,
        category_id: i64,
        text: &str,
        tag_names: &[String],
    ) -> Result<V2TodoItem, rusqlite::Error> {
        conn.execute("BEGIN TRANSACTION", [])?;

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

        if let Err(error) = conn.execute(
            "INSERT INTO v2_todos
                (category_id, text, memo, done, display_order, created_at, updated_at)
             VALUES (?1, ?2, NULL, 0, ?3, ?4, ?5)",
            params![category_id, text, display_order, &now, &now],
        ) {
            let _ = conn.execute("ROLLBACK", []);
            return Err(error);
        }
        let item_id = conn.last_insert_rowid();

        if let Err(error) = Self::replace_item_tags(conn, item_id, tag_names) {
            let _ = conn.execute("ROLLBACK", []);
            return Err(error);
        }

        conn.execute("COMMIT", [])?;

        let item = V2TodoItem {
            id: item_id,
            category_id,
            text: text.to_string(),
            memo: None,
            tags: Vec::new(),
            done: false,
            display_order,
            created_at: now.clone(),
            updated_at: now,
        };

        Ok(Self::attach_tags_to_items(conn, vec![item])?
            .into_iter()
            .next()
            .ok_or(rusqlite::Error::QueryReturnedNoRows)?)
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

    pub fn update_item_details(
        conn: &Connection,
        id: i64,
        text: &str,
        memo: Option<&str>,
        tag_names: &[String],
    ) -> Result<V2TodoItem, rusqlite::Error> {
        conn.execute("BEGIN TRANSACTION", [])?;
        let now = Self::now_iso();
        let updated = match conn.execute(
            "UPDATE v2_todos SET text = ?1, memo = ?2, updated_at = ?3 WHERE id = ?4",
            params![text, memo, now, id],
        ) {
            Ok(updated) => updated,
            Err(error) => {
                let _ = conn.execute("ROLLBACK", []);
                return Err(error);
            }
        };
        if updated == 0 {
            let _ = conn.execute("ROLLBACK", []);
            return Err(rusqlite::Error::QueryReturnedNoRows);
        }

        if let Err(error) = Self::replace_item_tags(conn, id, tag_names) {
            let _ = conn.execute("ROLLBACK", []);
            return Err(error);
        }

        conn.execute("COMMIT", [])?;

        Self::get_item_by_id(conn, id)?.ok_or(rusqlite::Error::QueryReturnedNoRows)
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
        conn.execute("BEGIN TRANSACTION", [])?;

        if let Err(error) = conn.execute("DELETE FROM v2_todo_tags WHERE todo_id = ?1", params![id])
        {
            let _ = conn.execute("ROLLBACK", []);
            return Err(error);
        }

        let updated = match conn.execute("DELETE FROM v2_todos WHERE id = ?1", params![id]) {
            Ok(updated) => updated,
            Err(error) => {
                let _ = conn.execute("ROLLBACK", []);
                return Err(error);
            }
        };
        if updated == 0 {
            let _ = conn.execute("ROLLBACK", []);
            return Err(rusqlite::Error::QueryReturnedNoRows);
        }

        if let Err(error) = Self::cleanup_unused_tags(conn) {
            let _ = conn.execute("ROLLBACK", []);
            return Err(error);
        }

        conn.execute("COMMIT", [])?;
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
    fn migrates_existing_v2_todos_with_memo_column() {
        let conn = Connection::open_in_memory().expect("in-memory db");
        conn.execute_batch(
            "CREATE TABLE v2_categories (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                name TEXT NOT NULL UNIQUE,
                display_order INTEGER NOT NULL,
                created_at TEXT NOT NULL,
                updated_at TEXT NOT NULL
            );

            CREATE TABLE v2_todos (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                category_id INTEGER NOT NULL,
                text TEXT NOT NULL,
                done BOOLEAN NOT NULL DEFAULT 0,
                display_order INTEGER NOT NULL,
                created_at TEXT NOT NULL,
                updated_at TEXT NOT NULL
            );",
        )
        .expect("old v2 schema");

        V2ChecklistRepository::create_tables(&conn).expect("migrated v2 schema");
        let has_memo = V2ChecklistRepository::v2_todos_has_column(&conn, "memo").unwrap();

        assert!(has_memo);
    }

    #[test]
    fn keeps_done_items_after_pending_items() {
        let conn = setup_conn();
        let category_id = V2ChecklistRepository::get_categories(&conn).unwrap()[0].id;
        let first = V2ChecklistRepository::create_item(&conn, category_id, "First", &[]).unwrap();
        let second = V2ChecklistRepository::create_item(&conn, category_id, "Second", &[]).unwrap();

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

    #[test]
    fn creates_items_with_tags_and_returns_them() {
        let conn = setup_conn();
        let category_id = V2ChecklistRepository::get_categories(&conn).unwrap()[0].id;
        let item = V2ChecklistRepository::create_item(
            &conn,
            category_id,
            "Read",
            &["church".to_string(), "morning".to_string()],
        )
        .unwrap();

        assert_eq!(item.tags.len(), 2);
        assert_eq!(item.tags[0].name, "church");
        assert_eq!(item.tags[1].name, "morning");

        let items = V2ChecklistRepository::get_items(&conn, category_id).unwrap();
        assert_eq!(items[0].tags.len(), 2);
    }

    #[test]
    fn replaces_item_tags_and_cleans_unused_tags() {
        let conn = setup_conn();
        let category_id = V2ChecklistRepository::get_categories(&conn).unwrap()[0].id;
        let item = V2ChecklistRepository::create_item(
            &conn,
            category_id,
            "Read",
            &["church".to_string(), "morning".to_string()],
        )
        .unwrap();

        let updated = V2ChecklistRepository::update_item_details(
            &conn,
            item.id,
            "Read",
            None,
            &["home".to_string()],
        )
        .unwrap();

        assert_eq!(updated.tags.len(), 1);
        assert_eq!(updated.tags[0].name, "home");

        let tags = V2ChecklistRepository::get_tags(&conn).unwrap();
        assert_eq!(tags.len(), 1);
        assert_eq!(tags[0].name, "home");
    }
}
