use rusqlite::Connection;
use tauri::{AppHandle, Manager};

use super::ChecklistRepository;

pub fn init_database(app: &AppHandle) -> Result<Connection, rusqlite::Error> {
    let app_dir = app
        .path()
        .app_data_dir()
        .expect("Failed to get app data directory");
    std::fs::create_dir_all(&app_dir).expect("Failed to create app data directory");

    let db_path = app_dir.join("tickly.db");
    let conn = Connection::open(db_path)?;

    create_tables(&conn)?;

    Ok(conn)
}

fn create_tables(conn: &Connection) -> Result<(), rusqlite::Error> {
    conn.execute(
        "CREATE TABLE IF NOT EXISTS settings (
            key TEXT PRIMARY KEY,
            value TEXT NOT NULL
        )",
        [],
    )?;

    ChecklistRepository::create_tables(conn)?;
    ChecklistRepository::ensure_default_category(conn)?;
    Ok(())
}
