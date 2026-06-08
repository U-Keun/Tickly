use rusqlite::Connection;

use super::V2ChecklistRepository;

pub fn run_migrations(conn: &Connection) -> Result<(), rusqlite::Error> {
    migrate_add_category_id(conn)?;
    migrate_add_display_order_to_todos(conn)?;
    migrate_add_memo(conn)?;
    migrate_add_display_order_to_categories(conn)?;
    migrate_add_reminder_at(conn)?;
    migrate_add_repeat_columns(conn)?;
    migrate_create_completion_logs(conn)?;
    migrate_add_track_streak(conn)?;
    migrate_completion_logs_add_item_id(conn)?;
    migrate_add_sync_fields(conn)?;
    migrate_create_auth_session(conn)?;
    migrate_create_sync_metadata(conn)?;
    migrate_create_tags(conn)?;
    migrate_create_todo_tags(conn)?;
    migrate_add_linked_app(conn)?;
    migrate_create_v2_checklist_tables(conn)?;
    Ok(())
}

fn should_add_column(conn: &Connection, table: &str, column: &str) -> bool {
    let query = format!("SELECT COUNT(*) FROM pragma_table_info('{table}') WHERE name = ?1");
    let column_exists: Result<i64, _> = conn.query_row(&query, [column], |row| row.get(0));
    matches!(column_exists, Ok(0))
}

fn migrate_add_category_id(conn: &Connection) -> Result<(), rusqlite::Error> {
    if should_add_column(conn, "todos", "category_id") {
        conn.execute("ALTER TABLE todos ADD COLUMN category_id INTEGER", [])?;
    }

    Ok(())
}

fn migrate_add_display_order_to_todos(conn: &Connection) -> Result<(), rusqlite::Error> {
    if should_add_column(conn, "todos", "display_order") {
        conn.execute(
            "ALTER TABLE todos ADD COLUMN display_order INTEGER NOT NULL DEFAULT 0",
            [],
        )?;

        // Assign initial order based on id (preserves current chronological order)
        conn.execute("UPDATE todos SET display_order = id * 1000", [])?;
    }

    Ok(())
}

fn migrate_add_memo(conn: &Connection) -> Result<(), rusqlite::Error> {
    if should_add_column(conn, "todos", "memo") {
        conn.execute("ALTER TABLE todos ADD COLUMN memo TEXT", [])?;
    }

    Ok(())
}

fn migrate_add_display_order_to_categories(conn: &Connection) -> Result<(), rusqlite::Error> {
    if should_add_column(conn, "categories", "display_order") {
        conn.execute(
            "ALTER TABLE categories ADD COLUMN display_order INTEGER NOT NULL DEFAULT 0",
            [],
        )?;

        // Assign initial order based on id
        conn.execute("UPDATE categories SET display_order = id * 1000", [])?;
    }

    Ok(())
}

fn migrate_add_reminder_at(conn: &Connection) -> Result<(), rusqlite::Error> {
    if should_add_column(conn, "todos", "reminder_at") {
        conn.execute("ALTER TABLE todos ADD COLUMN reminder_at TEXT", [])?;
    }

    Ok(())
}

fn migrate_add_repeat_columns(conn: &Connection) -> Result<(), rusqlite::Error> {
    // Add repeat_type column (none/daily/weekly/monthly)
    if should_add_column(conn, "todos", "repeat_type") {
        conn.execute(
            "ALTER TABLE todos ADD COLUMN repeat_type TEXT NOT NULL DEFAULT 'none'",
            [],
        )?;
    }

    // Add repeat_detail column (JSON: weekly [0,3,5], monthly [1,15])
    if should_add_column(conn, "todos", "repeat_detail") {
        conn.execute("ALTER TABLE todos ADD COLUMN repeat_detail TEXT", [])?;
    }

    // Add next_due_at column (YYYY-MM-DD format)
    if should_add_column(conn, "todos", "next_due_at") {
        conn.execute("ALTER TABLE todos ADD COLUMN next_due_at TEXT", [])?;
    }

    // Add last_completed_at column (YYYY-MM-DD format)
    if should_add_column(conn, "todos", "last_completed_at") {
        conn.execute("ALTER TABLE todos ADD COLUMN last_completed_at TEXT", [])?;
    }

    Ok(())
}

fn migrate_create_completion_logs(conn: &Connection) -> Result<(), rusqlite::Error> {
    conn.execute(
        "CREATE TABLE IF NOT EXISTS completion_logs (
            completed_on TEXT PRIMARY KEY,
            completed_count INTEGER NOT NULL DEFAULT 0
        )",
        [],
    )?;
    Ok(())
}

fn migrate_add_track_streak(conn: &Connection) -> Result<(), rusqlite::Error> {
    if should_add_column(conn, "todos", "track_streak") {
        conn.execute(
            "ALTER TABLE todos ADD COLUMN track_streak INTEGER NOT NULL DEFAULT 0",
            [],
        )?;
    }

    Ok(())
}

fn migrate_completion_logs_add_item_id(conn: &Connection) -> Result<(), rusqlite::Error> {
    // Check if item_id column already exists
    if should_add_column(conn, "completion_logs", "item_id") {
        // Drop old table and create new one with item_id
        // (We don't need to preserve old data since we're switching to per-item tracking)
        conn.execute("DROP TABLE IF EXISTS completion_logs", [])?;
        conn.execute(
            "CREATE TABLE completion_logs (
                item_id INTEGER NOT NULL,
                completed_on TEXT NOT NULL,
                completed_count INTEGER NOT NULL DEFAULT 0,
                PRIMARY KEY (item_id, completed_on)
            )",
            [],
        )?;
    }

    Ok(())
}

fn migrate_add_sync_fields(conn: &Connection) -> Result<(), rusqlite::Error> {
    // Add sync fields to todos table
    if should_add_column(conn, "todos", "sync_id") {
        conn.execute("ALTER TABLE todos ADD COLUMN sync_id TEXT", [])?;
    }

    conn.execute(
        "CREATE UNIQUE INDEX IF NOT EXISTS idx_todos_sync_id ON todos(sync_id)",
        [],
    )?;

    if should_add_column(conn, "todos", "created_at") {
        conn.execute("ALTER TABLE todos ADD COLUMN created_at TEXT", [])?;
        conn.execute(
            "UPDATE todos SET created_at = datetime('now') WHERE created_at IS NULL",
            [],
        )?;
    }

    if should_add_column(conn, "todos", "updated_at") {
        conn.execute("ALTER TABLE todos ADD COLUMN updated_at TEXT", [])?;
        conn.execute(
            "UPDATE todos SET updated_at = datetime('now') WHERE updated_at IS NULL",
            [],
        )?;
    }

    if should_add_column(conn, "todos", "sync_status") {
        conn.execute(
            "ALTER TABLE todos ADD COLUMN sync_status TEXT DEFAULT 'pending'",
            [],
        )?;
    }

    // Add sync fields to categories table
    if should_add_column(conn, "categories", "sync_id") {
        conn.execute("ALTER TABLE categories ADD COLUMN sync_id TEXT", [])?;
    }

    conn.execute(
        "CREATE UNIQUE INDEX IF NOT EXISTS idx_categories_sync_id ON categories(sync_id)",
        [],
    )?;

    if should_add_column(conn, "categories", "created_at") {
        conn.execute("ALTER TABLE categories ADD COLUMN created_at TEXT", [])?;
        conn.execute(
            "UPDATE categories SET created_at = datetime('now') WHERE created_at IS NULL",
            [],
        )?;
    }

    if should_add_column(conn, "categories", "updated_at") {
        conn.execute("ALTER TABLE categories ADD COLUMN updated_at TEXT", [])?;
        conn.execute(
            "UPDATE categories SET updated_at = datetime('now') WHERE updated_at IS NULL",
            [],
        )?;
    }

    if should_add_column(conn, "categories", "sync_status") {
        conn.execute(
            "ALTER TABLE categories ADD COLUMN sync_status TEXT DEFAULT 'pending'",
            [],
        )?;
    }

    Ok(())
}

fn migrate_create_auth_session(conn: &Connection) -> Result<(), rusqlite::Error> {
    conn.execute(
        "CREATE TABLE IF NOT EXISTS auth_session (
            id INTEGER PRIMARY KEY CHECK (id = 1),
            user_id TEXT NOT NULL,
            access_token TEXT NOT NULL,
            refresh_token TEXT NOT NULL,
            expires_at TEXT NOT NULL,
            provider TEXT NOT NULL
        )",
        [],
    )?;
    Ok(())
}

fn migrate_create_sync_metadata(conn: &Connection) -> Result<(), rusqlite::Error> {
    conn.execute(
        "CREATE TABLE IF NOT EXISTS sync_metadata (
            key TEXT PRIMARY KEY,
            value TEXT
        )",
        [],
    )?;
    Ok(())
}

fn migrate_create_tags(conn: &Connection) -> Result<(), rusqlite::Error> {
    conn.execute(
        "CREATE TABLE IF NOT EXISTS tags (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            name TEXT NOT NULL UNIQUE,
            sync_id TEXT,
            created_at TEXT,
            updated_at TEXT,
            sync_status TEXT DEFAULT 'pending'
        )",
        [],
    )?;
    conn.execute(
        "CREATE UNIQUE INDEX IF NOT EXISTS idx_tags_sync_id ON tags(sync_id)",
        [],
    )?;
    Ok(())
}

fn migrate_add_linked_app(conn: &Connection) -> Result<(), rusqlite::Error> {
    if should_add_column(conn, "todos", "linked_app") {
        conn.execute("ALTER TABLE todos ADD COLUMN linked_app TEXT", [])?;
    }

    Ok(())
}

fn migrate_create_todo_tags(conn: &Connection) -> Result<(), rusqlite::Error> {
    conn.execute(
        "CREATE TABLE IF NOT EXISTS todo_tags (
            todo_id INTEGER NOT NULL,
            tag_id INTEGER NOT NULL,
            sync_id TEXT,
            created_at TEXT,
            sync_status TEXT DEFAULT 'pending',
            PRIMARY KEY (todo_id, tag_id),
            FOREIGN KEY (todo_id) REFERENCES todos(id) ON DELETE CASCADE,
            FOREIGN KEY (tag_id) REFERENCES tags(id) ON DELETE CASCADE
        )",
        [],
    )?;
    conn.execute(
        "CREATE UNIQUE INDEX IF NOT EXISTS idx_todo_tags_sync_id ON todo_tags(sync_id)",
        [],
    )?;
    Ok(())
}

fn migrate_create_v2_checklist_tables(conn: &Connection) -> Result<(), rusqlite::Error> {
    V2ChecklistRepository::create_tables(conn)?;
    V2ChecklistRepository::ensure_default_category(conn)?;
    Ok(())
}
