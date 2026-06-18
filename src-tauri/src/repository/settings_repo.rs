use rusqlite::{params, Connection};

pub struct SettingsRepository;

impl SettingsRepository {
    pub fn get(conn: &Connection, key: &str) -> Result<Option<String>, rusqlite::Error> {
        let result: Result<String, _> = conn.query_row(
            "SELECT value FROM settings WHERE key = ?1",
            params![key],
            |row| row.get(0),
        );

        match result {
            Ok(value) => Ok(Some(value)),
            Err(rusqlite::Error::QueryReturnedNoRows) => Ok(None),
            Err(e) => Err(e),
        }
    }

    pub fn set(conn: &Connection, key: &str, value: &str) -> Result<(), rusqlite::Error> {
        conn.execute(
            "INSERT OR REPLACE INTO settings (key, value) VALUES (?1, ?2)",
            params![key, value],
        )?;
        Ok(())
    }
}
