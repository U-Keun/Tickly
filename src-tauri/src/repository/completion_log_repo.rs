use rusqlite::{params, Connection};

use crate::models::CompletionLog;

pub struct CompletionLogRepository;

impl CompletionLogRepository {
    /// Increment the completion count for a given item and date
    pub fn increment(conn: &Connection, item_id: i64, date: &str) -> Result<(), rusqlite::Error> {
        conn.execute(
            "INSERT INTO completion_logs (item_id, completed_on, completed_count)
             VALUES (?1, ?2, 1)
             ON CONFLICT(item_id, completed_on) DO UPDATE SET completed_count = completed_count + 1",
            params![item_id, date],
        )?;
        Ok(())
    }

    /// Decrement the completion count for a given item and date (minimum 0)
    pub fn decrement(conn: &Connection, item_id: i64, date: &str) -> Result<(), rusqlite::Error> {
        conn.execute(
            "UPDATE completion_logs
             SET completed_count = MAX(completed_count - 1, 0)
             WHERE item_id = ?1 AND completed_on = ?2",
            params![item_id, date],
        )?;
        Ok(())
    }

    /// Get completion logs for a specific item for the last N days
    pub fn get_logs_for_item(
        conn: &Connection,
        item_id: i64,
        days: i32,
    ) -> Result<Vec<CompletionLog>, rusqlite::Error> {
        let mut stmt = conn.prepare(
            "SELECT item_id, completed_on, completed_count FROM completion_logs
             WHERE item_id = ?1
             AND completed_on >= date('now', '-' || ?2 || ' days')
             AND completed_count > 0
             ORDER BY completed_on ASC",
        )?;

        let logs = stmt
            .query_map(params![item_id, days], |row| {
                Ok(CompletionLog {
                    item_id: row.get(0)?,
                    completed_on: row.get(1)?,
                    completed_count: row.get(2)?,
                })
            })?
            .filter_map(Result::ok)
            .collect();

        Ok(logs)
    }

    /// Get all completion logs for a specific item (for streak calculation)
    pub fn get_all_logs_for_item(
        conn: &Connection,
        item_id: i64,
    ) -> Result<Vec<CompletionLog>, rusqlite::Error> {
        let mut stmt = conn.prepare(
            "SELECT item_id, completed_on, completed_count FROM completion_logs
             WHERE item_id = ?1
             AND completed_count > 0
             ORDER BY completed_on ASC",
        )?;

        let logs = stmt
            .query_map(params![item_id], |row| {
                Ok(CompletionLog {
                    item_id: row.get(0)?,
                    completed_on: row.get(1)?,
                    completed_count: row.get(2)?,
                })
            })?
            .filter_map(Result::ok)
            .collect();

        Ok(logs)
    }

    /// Get all completion logs (for sync push)
    pub fn get_all(conn: &Connection) -> Result<Vec<CompletionLog>, rusqlite::Error> {
        let mut stmt = conn.prepare(
            "SELECT item_id, completed_on, completed_count FROM completion_logs
             WHERE completed_count > 0
             ORDER BY item_id, completed_on",
        )?;

        let logs = stmt
            .query_map([], |row| {
                Ok(CompletionLog {
                    item_id: row.get(0)?,
                    completed_on: row.get(1)?,
                    completed_count: row.get(2)?,
                })
            })?
            .filter_map(Result::ok)
            .collect();

        Ok(logs)
    }

    pub fn delete_for_item(conn: &Connection, item_id: i64) -> Result<(), rusqlite::Error> {
        conn.execute(
            "DELETE FROM completion_logs WHERE item_id = ?1",
            params![item_id],
        )?;
        Ok(())
    }

    pub fn delete_for_category(conn: &Connection, category_id: i64) -> Result<(), rusqlite::Error> {
        conn.execute(
            "DELETE FROM completion_logs
             WHERE item_id IN (SELECT id FROM todos WHERE category_id = ?1)",
            params![category_id],
        )?;
        Ok(())
    }

    pub fn prune_stale_logs(conn: &Connection) -> Result<(), rusqlite::Error> {
        conn.execute(
            "DELETE FROM completion_logs
             WHERE item_id NOT IN (SELECT id FROM todos)
                OR item_id IN (SELECT id FROM todos WHERE sync_status = 'deleted')",
            [],
        )?;
        Ok(())
    }

    /// Upsert a completion log (for sync pull)
    pub fn upsert(
        conn: &Connection,
        item_id: i64,
        completed_on: &str,
        completed_count: i32,
    ) -> Result<(), rusqlite::Error> {
        conn.execute(
            "INSERT INTO completion_logs (item_id, completed_on, completed_count)
             VALUES (?1, ?2, ?3)
             ON CONFLICT(item_id, completed_on) DO UPDATE SET
                completed_count = MAX(completion_logs.completed_count, excluded.completed_count)",
            params![item_id, completed_on, completed_count],
        )?;
        Ok(())
    }
}
