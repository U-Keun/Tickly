use rusqlite::{params, Connection, OptionalExtension};
use serde_json::{json, Value};

use crate::models::ChecklistSyncRecord;

pub struct SyncRepository;

impl SyncRepository {
    pub fn create_tables(conn: &Connection) -> Result<(), rusqlite::Error> {
        conn.execute_batch(
            "CREATE TABLE IF NOT EXISTS checklist_sync_metadata (
                entity_type TEXT NOT NULL,
                local_id TEXT NOT NULL,
                sync_id TEXT NOT NULL UNIQUE,
                sync_status TEXT NOT NULL DEFAULT 'pending',
                deleted_at TEXT,
                last_synced_at TEXT,
                updated_at TEXT NOT NULL,
                PRIMARY KEY (entity_type, local_id)
            );

            CREATE INDEX IF NOT EXISTS idx_checklist_sync_metadata_status
                ON checklist_sync_metadata(sync_status, deleted_at);

            CREATE TABLE IF NOT EXISTS checklist_sync_state (
                key TEXT PRIMARY KEY,
                value TEXT NOT NULL
            );",
        )?;
        Self::create_triggers(conn)
    }

    fn create_triggers(conn: &Connection) -> Result<(), rusqlite::Error> {
        conn.execute_batch(
            "CREATE TRIGGER IF NOT EXISTS sync_checklist_categories_insert
             AFTER INSERT ON checklist_categories
             BEGIN
                INSERT INTO checklist_sync_metadata
                    (entity_type, local_id, sync_id, sync_status, deleted_at, updated_at)
                VALUES ('category', CAST(NEW.id AS TEXT), lower(hex(randomblob(16))), 'pending', NULL, NEW.updated_at)
                ON CONFLICT(entity_type, local_id) DO UPDATE SET
                    sync_status = 'pending',
                    deleted_at = NULL,
                    updated_at = excluded.updated_at;
             END;

             CREATE TRIGGER IF NOT EXISTS sync_checklist_categories_update
             AFTER UPDATE ON checklist_categories
             BEGIN
                INSERT INTO checklist_sync_metadata
                    (entity_type, local_id, sync_id, sync_status, deleted_at, updated_at)
                VALUES ('category', CAST(NEW.id AS TEXT), lower(hex(randomblob(16))), 'pending', NULL, NEW.updated_at)
                ON CONFLICT(entity_type, local_id) DO UPDATE SET
                    sync_status = 'pending',
                    deleted_at = NULL,
                    updated_at = excluded.updated_at;
             END;

             CREATE TRIGGER IF NOT EXISTS sync_checklist_categories_delete
             BEFORE DELETE ON checklist_categories
             BEGIN
                INSERT INTO checklist_sync_metadata
                    (entity_type, local_id, sync_id, sync_status, deleted_at, updated_at)
                VALUES ('category', CAST(OLD.id AS TEXT), lower(hex(randomblob(16))), 'pending', strftime('%Y-%m-%dT%H:%M:%SZ','now'), strftime('%Y-%m-%dT%H:%M:%SZ','now'))
                ON CONFLICT(entity_type, local_id) DO UPDATE SET
                    sync_status = 'pending',
                    deleted_at = strftime('%Y-%m-%dT%H:%M:%SZ','now'),
                    updated_at = strftime('%Y-%m-%dT%H:%M:%SZ','now');
             END;

             CREATE TRIGGER IF NOT EXISTS sync_checklist_todos_insert
             AFTER INSERT ON checklist_todos
             BEGIN
                INSERT INTO checklist_sync_metadata
                    (entity_type, local_id, sync_id, sync_status, deleted_at, updated_at)
                VALUES ('todo', CAST(NEW.id AS TEXT), lower(hex(randomblob(16))), 'pending', NULL, NEW.updated_at)
                ON CONFLICT(entity_type, local_id) DO UPDATE SET
                    sync_status = 'pending',
                    deleted_at = NULL,
                    updated_at = excluded.updated_at;
             END;

             CREATE TRIGGER IF NOT EXISTS sync_checklist_todos_update
             AFTER UPDATE ON checklist_todos
             BEGIN
                INSERT INTO checklist_sync_metadata
                    (entity_type, local_id, sync_id, sync_status, deleted_at, updated_at)
                VALUES ('todo', CAST(NEW.id AS TEXT), lower(hex(randomblob(16))), 'pending', NULL, NEW.updated_at)
                ON CONFLICT(entity_type, local_id) DO UPDATE SET
                    sync_status = 'pending',
                    deleted_at = NULL,
                    updated_at = excluded.updated_at;
             END;

             CREATE TRIGGER IF NOT EXISTS sync_checklist_todos_delete
             BEFORE DELETE ON checklist_todos
             BEGIN
                INSERT INTO checklist_sync_metadata
                    (entity_type, local_id, sync_id, sync_status, deleted_at, updated_at)
                VALUES ('todo', CAST(OLD.id AS TEXT), lower(hex(randomblob(16))), 'pending', strftime('%Y-%m-%dT%H:%M:%SZ','now'), strftime('%Y-%m-%dT%H:%M:%SZ','now'))
                ON CONFLICT(entity_type, local_id) DO UPDATE SET
                    sync_status = 'pending',
                    deleted_at = strftime('%Y-%m-%dT%H:%M:%SZ','now'),
                    updated_at = strftime('%Y-%m-%dT%H:%M:%SZ','now');
             END;

             CREATE TRIGGER IF NOT EXISTS sync_checklist_tags_insert
             AFTER INSERT ON checklist_tags
             BEGIN
                INSERT INTO checklist_sync_metadata
                    (entity_type, local_id, sync_id, sync_status, deleted_at, updated_at)
                VALUES ('tag', CAST(NEW.id AS TEXT), lower(hex(randomblob(16))), 'pending', NULL, NEW.updated_at)
                ON CONFLICT(entity_type, local_id) DO UPDATE SET
                    sync_status = 'pending',
                    deleted_at = NULL,
                    updated_at = excluded.updated_at;
             END;

             CREATE TRIGGER IF NOT EXISTS sync_checklist_tags_update
             AFTER UPDATE ON checklist_tags
             BEGIN
                INSERT INTO checklist_sync_metadata
                    (entity_type, local_id, sync_id, sync_status, deleted_at, updated_at)
                VALUES ('tag', CAST(NEW.id AS TEXT), lower(hex(randomblob(16))), 'pending', NULL, NEW.updated_at)
                ON CONFLICT(entity_type, local_id) DO UPDATE SET
                    sync_status = 'pending',
                    deleted_at = NULL,
                    updated_at = excluded.updated_at;
             END;

             CREATE TRIGGER IF NOT EXISTS sync_checklist_tags_delete
             BEFORE DELETE ON checklist_tags
             BEGIN
                INSERT INTO checklist_sync_metadata
                    (entity_type, local_id, sync_id, sync_status, deleted_at, updated_at)
                VALUES ('tag', CAST(OLD.id AS TEXT), lower(hex(randomblob(16))), 'pending', strftime('%Y-%m-%dT%H:%M:%SZ','now'), strftime('%Y-%m-%dT%H:%M:%SZ','now'))
                ON CONFLICT(entity_type, local_id) DO UPDATE SET
                    sync_status = 'pending',
                    deleted_at = strftime('%Y-%m-%dT%H:%M:%SZ','now'),
                    updated_at = strftime('%Y-%m-%dT%H:%M:%SZ','now');
             END;

             CREATE TRIGGER IF NOT EXISTS sync_checklist_todo_tags_insert
             AFTER INSERT ON checklist_todo_tags
             BEGIN
                INSERT INTO checklist_sync_metadata
                    (entity_type, local_id, sync_id, sync_status, deleted_at, updated_at)
                VALUES ('todo_tag', CAST(NEW.todo_id AS TEXT) || ':' || CAST(NEW.tag_id AS TEXT), lower(hex(randomblob(16))), 'pending', NULL, NEW.created_at)
                ON CONFLICT(entity_type, local_id) DO UPDATE SET
                    sync_status = 'pending',
                    deleted_at = NULL,
                    updated_at = excluded.updated_at;
             END;

             CREATE TRIGGER IF NOT EXISTS sync_checklist_todo_tags_delete
             BEFORE DELETE ON checklist_todo_tags
             BEGIN
                INSERT INTO checklist_sync_metadata
                    (entity_type, local_id, sync_id, sync_status, deleted_at, updated_at)
                VALUES ('todo_tag', CAST(OLD.todo_id AS TEXT) || ':' || CAST(OLD.tag_id AS TEXT), lower(hex(randomblob(16))), 'pending', strftime('%Y-%m-%dT%H:%M:%SZ','now'), strftime('%Y-%m-%dT%H:%M:%SZ','now'))
                ON CONFLICT(entity_type, local_id) DO UPDATE SET
                    sync_status = 'pending',
                    deleted_at = strftime('%Y-%m-%dT%H:%M:%SZ','now'),
                    updated_at = strftime('%Y-%m-%dT%H:%M:%SZ','now');
             END;

             CREATE TRIGGER IF NOT EXISTS sync_checklist_completion_logs_insert
             AFTER INSERT ON checklist_completion_logs
             BEGIN
                INSERT INTO checklist_sync_metadata
                    (entity_type, local_id, sync_id, sync_status, deleted_at, updated_at)
                VALUES ('completion_log', CAST(NEW.item_id AS TEXT) || ':' || NEW.completed_on, lower(hex(randomblob(16))), 'pending', NULL, NEW.updated_at)
                ON CONFLICT(entity_type, local_id) DO UPDATE SET
                    sync_status = 'pending',
                    deleted_at = NULL,
                    updated_at = excluded.updated_at;
             END;

             CREATE TRIGGER IF NOT EXISTS sync_checklist_completion_logs_update
             AFTER UPDATE ON checklist_completion_logs
             BEGIN
                INSERT INTO checklist_sync_metadata
                    (entity_type, local_id, sync_id, sync_status, deleted_at, updated_at)
                VALUES ('completion_log', CAST(NEW.item_id AS TEXT) || ':' || NEW.completed_on, lower(hex(randomblob(16))), 'pending', NULL, NEW.updated_at)
                ON CONFLICT(entity_type, local_id) DO UPDATE SET
                    sync_status = 'pending',
                    deleted_at = NULL,
                    updated_at = excluded.updated_at;
             END;

             CREATE TRIGGER IF NOT EXISTS sync_checklist_completion_logs_delete
             BEFORE DELETE ON checklist_completion_logs
             BEGIN
                INSERT INTO checklist_sync_metadata
                    (entity_type, local_id, sync_id, sync_status, deleted_at, updated_at)
                VALUES ('completion_log', CAST(OLD.item_id AS TEXT) || ':' || OLD.completed_on, lower(hex(randomblob(16))), 'pending', strftime('%Y-%m-%dT%H:%M:%SZ','now'), strftime('%Y-%m-%dT%H:%M:%SZ','now'))
                ON CONFLICT(entity_type, local_id) DO UPDATE SET
                    sync_status = 'pending',
                    deleted_at = strftime('%Y-%m-%dT%H:%M:%SZ','now'),
                    updated_at = strftime('%Y-%m-%dT%H:%M:%SZ','now');
             END;

             CREATE TRIGGER IF NOT EXISTS sync_settings_reset_time_insert
             AFTER INSERT ON settings
             WHEN NEW.key = 'reset_time'
             BEGIN
                INSERT INTO checklist_sync_metadata
                    (entity_type, local_id, sync_id, sync_status, deleted_at, updated_at)
                VALUES ('setting', NEW.key, lower(hex(randomblob(16))), 'pending', NULL, strftime('%Y-%m-%dT%H:%M:%SZ','now'))
                ON CONFLICT(entity_type, local_id) DO UPDATE SET
                    sync_status = 'pending',
                    deleted_at = NULL,
                    updated_at = excluded.updated_at;
             END;

             CREATE TRIGGER IF NOT EXISTS sync_settings_reset_time_update
             AFTER UPDATE ON settings
             WHEN NEW.key = 'reset_time'
             BEGIN
                INSERT INTO checklist_sync_metadata
                    (entity_type, local_id, sync_id, sync_status, deleted_at, updated_at)
                VALUES ('setting', NEW.key, lower(hex(randomblob(16))), 'pending', NULL, strftime('%Y-%m-%dT%H:%M:%SZ','now'))
                ON CONFLICT(entity_type, local_id) DO UPDATE SET
                    sync_status = 'pending',
                    deleted_at = NULL,
                    updated_at = excluded.updated_at;
             END;",
        )?;
        Ok(())
    }

    pub fn get_state(conn: &Connection, key: &str) -> Result<Option<String>, rusqlite::Error> {
        conn.query_row(
            "SELECT value FROM checklist_sync_state WHERE key = ?1",
            params![key],
            |row| row.get(0),
        )
        .optional()
    }

    pub fn set_state(conn: &Connection, key: &str, value: &str) -> Result<(), rusqlite::Error> {
        conn.execute(
            "INSERT INTO checklist_sync_state (key, value)
             VALUES (?1, ?2)
             ON CONFLICT(key) DO UPDATE SET value = excluded.value",
            params![key, value],
        )?;
        Ok(())
    }

    pub fn ensure_baseline(conn: &Connection) -> Result<(), rusqlite::Error> {
        conn.execute(
            "INSERT OR IGNORE INTO checklist_sync_metadata
                (entity_type, local_id, sync_id, sync_status, deleted_at, updated_at)
             SELECT 'category', CAST(id AS TEXT), lower(hex(randomblob(16))), 'pending', NULL, updated_at
             FROM checklist_categories",
            [],
        )?;
        conn.execute(
            "INSERT OR IGNORE INTO checklist_sync_metadata
                (entity_type, local_id, sync_id, sync_status, deleted_at, updated_at)
             SELECT 'todo', CAST(id AS TEXT), lower(hex(randomblob(16))), 'pending', NULL, updated_at
             FROM checklist_todos",
            [],
        )?;
        conn.execute(
            "INSERT OR IGNORE INTO checklist_sync_metadata
                (entity_type, local_id, sync_id, sync_status, deleted_at, updated_at)
             SELECT 'tag', CAST(id AS TEXT), lower(hex(randomblob(16))), 'pending', NULL, updated_at
             FROM checklist_tags",
            [],
        )?;
        conn.execute(
            "INSERT OR IGNORE INTO checklist_sync_metadata
                (entity_type, local_id, sync_id, sync_status, deleted_at, updated_at)
             SELECT 'todo_tag',
                    CAST(todo_id AS TEXT) || ':' || CAST(tag_id AS TEXT),
                    lower(hex(randomblob(16))),
                    'pending',
                    NULL,
                    created_at
             FROM checklist_todo_tags",
            [],
        )?;
        conn.execute(
            "INSERT OR IGNORE INTO checklist_sync_metadata
                (entity_type, local_id, sync_id, sync_status, deleted_at, updated_at)
             SELECT 'completion_log',
                    CAST(item_id AS TEXT) || ':' || completed_on,
                    lower(hex(randomblob(16))),
                    'pending',
                    NULL,
                    updated_at
             FROM checklist_completion_logs",
            [],
        )?;
        conn.execute(
            "INSERT OR IGNORE INTO checklist_sync_metadata
                (entity_type, local_id, sync_id, sync_status, deleted_at, updated_at)
             SELECT 'setting',
                    key,
                    lower(hex(randomblob(16))),
                    'pending',
                    NULL,
                    strftime('%Y-%m-%dT%H:%M:%SZ','now')
             FROM settings
             WHERE key = 'reset_time'",
            [],
        )?;
        Ok(())
    }

    pub fn export_records(conn: &Connection) -> Result<Vec<ChecklistSyncRecord>, rusqlite::Error> {
        Self::ensure_baseline(conn)?;
        let mut stmt = conn.prepare(
            "SELECT entity_type, local_id, sync_id, updated_at, deleted_at
             FROM checklist_sync_metadata
             ORDER BY
                CASE entity_type
                    WHEN 'category' THEN 1
                    WHEN 'tag' THEN 2
                    WHEN 'todo' THEN 3
                    WHEN 'todo_tag' THEN 4
                    WHEN 'completion_log' THEN 5
                    WHEN 'setting' THEN 6
                    ELSE 99
                END,
                local_id ASC",
        )?;
        let rows = stmt.query_map([], |row| {
            Ok((
                row.get::<_, String>(0)?,
                row.get::<_, String>(1)?,
                row.get::<_, String>(2)?,
                row.get::<_, String>(3)?,
                row.get::<_, Option<String>>(4)?,
            ))
        })?;

        let mut records = Vec::new();
        for row in rows {
            let (entity_type, local_id, sync_id, updated_at, deleted_at) = row?;
            let payload = if deleted_at.is_some() {
                json!({})
            } else {
                match Self::payload_for(conn, &entity_type, &local_id)? {
                    Some(value) => value,
                    None => continue,
                }
            };
            records.push(ChecklistSyncRecord {
                entity_type,
                sync_id,
                updated_at,
                deleted_at,
                payload,
            });
        }

        Ok(records)
    }

    fn payload_for(
        conn: &Connection,
        entity_type: &str,
        local_id: &str,
    ) -> Result<Option<Value>, rusqlite::Error> {
        match entity_type {
            "category" => Self::category_payload(conn, local_id),
            "todo" => Self::todo_payload(conn, local_id),
            "tag" => Self::tag_payload(conn, local_id),
            "todo_tag" => Self::todo_tag_payload(conn, local_id),
            "completion_log" => Self::completion_log_payload(conn, local_id),
            "setting" => Self::setting_payload(conn, local_id),
            _ => Ok(None),
        }
    }

    fn category_payload(
        conn: &Connection,
        local_id: &str,
    ) -> Result<Option<Value>, rusqlite::Error> {
        conn.query_row(
            "SELECT name, display_order, created_at, updated_at
             FROM checklist_categories
             WHERE id = ?1",
            params![local_id],
            |row| {
                Ok(json!({
                    "name": row.get::<_, String>(0)?,
                    "displayOrder": row.get::<_, i64>(1)?,
                    "createdAt": row.get::<_, String>(2)?,
                    "updatedAt": row.get::<_, String>(3)?,
                }))
            },
        )
        .optional()
    }

    fn todo_payload(conn: &Connection, local_id: &str) -> Result<Option<Value>, rusqlite::Error> {
        conn.query_row(
            "SELECT
                category_id, text, memo, repeat_type, repeat_detail, next_due_at,
                last_completed_at, reminder_at, archived_at, track_streak,
                streak_started_on, done, display_order, created_at, updated_at
             FROM checklist_todos
             WHERE id = ?1",
            params![local_id],
            |row| {
                let category_id: i64 = row.get(0)?;
                let category_sync_id =
                    Self::sync_id_for_local(conn, "category", &category_id.to_string())?;
                let category_name = Self::category_name_for_local(conn, &category_id.to_string())?;
                Ok(json!({
                    "categorySyncId": category_sync_id,
                    "categoryName": category_name,
                    "text": row.get::<_, String>(1)?,
                    "memo": row.get::<_, Option<String>>(2)?,
                    "repeatType": row.get::<_, String>(3)?,
                    "repeatDetail": row.get::<_, Option<String>>(4)?,
                    "nextDueAt": row.get::<_, Option<String>>(5)?,
                    "lastCompletedAt": row.get::<_, Option<String>>(6)?,
                    "reminderAt": row.get::<_, Option<String>>(7)?,
                    "archivedAt": row.get::<_, Option<String>>(8)?,
                    "trackStreak": row.get::<_, bool>(9)?,
                    "streakStartedOn": row.get::<_, Option<String>>(10)?,
                    "done": row.get::<_, bool>(11)?,
                    "displayOrder": row.get::<_, i64>(12)?,
                    "createdAt": row.get::<_, String>(13)?,
                    "updatedAt": row.get::<_, String>(14)?,
                }))
            },
        )
        .optional()
    }

    fn tag_payload(conn: &Connection, local_id: &str) -> Result<Option<Value>, rusqlite::Error> {
        conn.query_row(
            "SELECT name, created_at, updated_at
             FROM checklist_tags
             WHERE id = ?1",
            params![local_id],
            |row| {
                Ok(json!({
                    "name": row.get::<_, String>(0)?,
                    "createdAt": row.get::<_, String>(1)?,
                    "updatedAt": row.get::<_, String>(2)?,
                }))
            },
        )
        .optional()
    }

    fn todo_tag_payload(
        conn: &Connection,
        local_id: &str,
    ) -> Result<Option<Value>, rusqlite::Error> {
        let Some((todo_id, tag_id)) = Self::parse_pair(local_id) else {
            return Ok(None);
        };
        conn.query_row(
            "SELECT created_at
             FROM checklist_todo_tags
             WHERE todo_id = ?1 AND tag_id = ?2",
            params![todo_id, tag_id],
            |row| {
                Ok(json!({
                    "todoSyncId": Self::sync_id_for_local(conn, "todo", &todo_id.to_string())?,
                    "tagSyncId": Self::sync_id_for_local(conn, "tag", &tag_id.to_string())?,
                    "createdAt": row.get::<_, String>(0)?,
                    "updatedAt": row.get::<_, String>(0)?,
                }))
            },
        )
        .optional()
    }

    fn completion_log_payload(
        conn: &Connection,
        local_id: &str,
    ) -> Result<Option<Value>, rusqlite::Error> {
        let Some((item_id, completed_on)) = Self::parse_log_key(local_id) else {
            return Ok(None);
        };
        conn.query_row(
            "SELECT completed_count, created_at, updated_at
             FROM checklist_completion_logs
             WHERE item_id = ?1 AND completed_on = ?2",
            params![item_id, completed_on],
            |row| {
                Ok(json!({
                    "itemSyncId": Self::sync_id_for_local(conn, "todo", &item_id.to_string())?,
                    "completedOn": completed_on,
                    "completedCount": row.get::<_, i64>(0)?,
                    "createdAt": row.get::<_, String>(1)?,
                    "updatedAt": row.get::<_, String>(2)?,
                }))
            },
        )
        .optional()
    }

    fn setting_payload(
        conn: &Connection,
        local_id: &str,
    ) -> Result<Option<Value>, rusqlite::Error> {
        if local_id != "reset_time" {
            return Ok(None);
        }
        let value: Option<String> = conn
            .query_row(
                "SELECT value FROM settings WHERE key = 'reset_time'",
                [],
                |row| row.get(0),
            )
            .optional()?;
        Ok(value.map(|value| {
            json!({
                "key": "reset_time",
                "value": value,
                "updatedAt": Self::now_iso(),
            })
        }))
    }

    pub fn apply_remote_records(
        conn: &Connection,
        records: &[ChecklistSyncRecord],
    ) -> Result<i64, rusqlite::Error> {
        conn.execute("BEGIN TRANSACTION", [])?;

        let operation = (|| -> Result<i64, rusqlite::Error> {
            let mut applied = 0;
            for record in Self::ordered_records(records, true) {
                if record.deleted_at.is_some() {
                    applied += Self::apply_remote_delete(conn, record)? as i64;
                }
            }
            for record in Self::ordered_records(records, false) {
                if record.deleted_at.is_none() {
                    applied += Self::apply_remote_upsert(conn, record)? as i64;
                }
            }
            Ok(applied)
        })();

        match operation {
            Ok(applied) => {
                conn.execute("COMMIT", [])?;
                Ok(applied)
            }
            Err(error) => {
                let _ = conn.execute("ROLLBACK", []);
                Err(error)
            }
        }
    }

    fn ordered_records(
        records: &[ChecklistSyncRecord],
        deleted: bool,
    ) -> Vec<&ChecklistSyncRecord> {
        let mut ordered: Vec<&ChecklistSyncRecord> = records
            .iter()
            .filter(|record| record.deleted_at.is_some() == deleted)
            .collect();
        ordered.sort_by_key(|record| match (deleted, record.entity_type.as_str()) {
            (true, "todo_tag") => 1,
            (true, "completion_log") => 2,
            (true, "todo") => 3,
            (true, "tag") => 4,
            (true, "category") => 5,
            (false, "category") => 1,
            (false, "tag") => 2,
            (false, "todo") => 3,
            (false, "todo_tag") => 4,
            (false, "completion_log") => 5,
            (false, "setting") => 6,
            _ => 99,
        });
        ordered
    }

    fn apply_remote_delete(
        conn: &Connection,
        record: &ChecklistSyncRecord,
    ) -> Result<bool, rusqlite::Error> {
        let local_id = Self::local_id_for_sync(conn, &record.entity_type, &record.sync_id)?;
        if let Some(local_id) = local_id.as_deref() {
            match record.entity_type.as_str() {
                "category" => {
                    conn.execute(
                        "DELETE FROM checklist_todo_tags
                         WHERE todo_id IN (SELECT id FROM checklist_todos WHERE category_id = ?1)",
                        params![local_id],
                    )?;
                    conn.execute(
                        "DELETE FROM checklist_completion_logs
                         WHERE item_id IN (SELECT id FROM checklist_todos WHERE category_id = ?1)",
                        params![local_id],
                    )?;
                    conn.execute(
                        "DELETE FROM checklist_todos WHERE category_id = ?1",
                        params![local_id],
                    )?;
                    conn.execute(
                        "DELETE FROM checklist_categories WHERE id = ?1",
                        params![local_id],
                    )?;
                }
                "todo" => {
                    conn.execute(
                        "DELETE FROM checklist_todo_tags WHERE todo_id = ?1",
                        params![local_id],
                    )?;
                    conn.execute(
                        "DELETE FROM checklist_completion_logs WHERE item_id = ?1",
                        params![local_id],
                    )?;
                    conn.execute(
                        "DELETE FROM checklist_todos WHERE id = ?1",
                        params![local_id],
                    )?;
                }
                "tag" => {
                    conn.execute(
                        "DELETE FROM checklist_todo_tags WHERE tag_id = ?1",
                        params![local_id],
                    )?;
                    conn.execute(
                        "DELETE FROM checklist_tags WHERE id = ?1",
                        params![local_id],
                    )?;
                }
                "todo_tag" => {
                    if let Some((todo_id, tag_id)) = Self::parse_pair(&local_id) {
                        conn.execute(
                            "DELETE FROM checklist_todo_tags WHERE todo_id = ?1 AND tag_id = ?2",
                            params![todo_id, tag_id],
                        )?;
                    }
                }
                "completion_log" => {
                    if let Some((item_id, completed_on)) = Self::parse_log_key(&local_id) {
                        conn.execute(
                            "DELETE FROM checklist_completion_logs WHERE item_id = ?1 AND completed_on = ?2",
                            params![item_id, completed_on],
                        )?;
                    }
                }
                "setting" if local_id == "reset_time" => {
                    conn.execute("DELETE FROM settings WHERE key = 'reset_time'", [])?;
                }
                _ => {}
            }
        }
        Self::upsert_metadata(
            conn,
            &record.entity_type,
            &local_id.unwrap_or_else(|| format!("deleted:{}", record.sync_id)),
            &record.sync_id,
            "synced",
            record.deleted_at.as_deref(),
            &record.updated_at,
        )?;
        Ok(true)
    }

    fn apply_remote_upsert(
        conn: &Connection,
        record: &ChecklistSyncRecord,
    ) -> Result<bool, rusqlite::Error> {
        match record.entity_type.as_str() {
            "category" => Self::apply_category(conn, record),
            "tag" => Self::apply_tag(conn, record),
            "todo" => Self::apply_todo(conn, record),
            "todo_tag" => Self::apply_todo_tag(conn, record),
            "completion_log" => Self::apply_completion_log(conn, record),
            "setting" => Self::apply_setting(conn, record),
            _ => Ok(false),
        }
    }

    fn apply_category(
        conn: &Connection,
        record: &ChecklistSyncRecord,
    ) -> Result<bool, rusqlite::Error> {
        let name = Self::string_field(&record.payload, "name", "Untitled");
        let display_order = Self::i64_field(&record.payload, "displayOrder", 1000);
        let created_at = Self::string_field(&record.payload, "createdAt", &record.updated_at);
        let updated_at = Self::string_field(&record.payload, "updatedAt", &record.updated_at);

        if let Some(local_id) = Self::local_id_for_sync(conn, "category", &record.sync_id)? {
            conn.execute(
                "UPDATE checklist_categories
                 SET name = ?1, display_order = ?2, created_at = ?3, updated_at = ?4
                 WHERE id = ?5",
                params![name, display_order, created_at, updated_at, local_id],
            )?;
            Self::upsert_metadata(
                conn,
                "category",
                &local_id,
                &record.sync_id,
                "synced",
                None,
                &record.updated_at,
            )?;
            return Ok(true);
        }

        conn.execute(
            "INSERT OR IGNORE INTO checklist_categories (name, display_order, created_at, updated_at)
             VALUES (?1, ?2, ?3, ?4)",
            params![name, display_order, created_at, updated_at],
        )?;
        let local_id = conn
            .query_row(
                "SELECT id FROM checklist_categories WHERE name = ?1",
                params![name],
                |row| row.get::<_, i64>(0),
            )?
            .to_string();
        Self::upsert_metadata(
            conn,
            "category",
            &local_id,
            &record.sync_id,
            "synced",
            None,
            &record.updated_at,
        )?;
        Ok(true)
    }

    fn apply_tag(conn: &Connection, record: &ChecklistSyncRecord) -> Result<bool, rusqlite::Error> {
        let name = Self::string_field(&record.payload, "name", "tag");
        let created_at = Self::string_field(&record.payload, "createdAt", &record.updated_at);
        let updated_at = Self::string_field(&record.payload, "updatedAt", &record.updated_at);

        if let Some(local_id) = Self::local_id_for_sync(conn, "tag", &record.sync_id)? {
            conn.execute(
                "UPDATE checklist_tags
                 SET name = ?1, created_at = ?2, updated_at = ?3
                 WHERE id = ?4",
                params![name, created_at, updated_at, local_id],
            )?;
            Self::upsert_metadata(
                conn,
                "tag",
                &local_id,
                &record.sync_id,
                "synced",
                None,
                &record.updated_at,
            )?;
            return Ok(true);
        }

        conn.execute(
            "INSERT OR IGNORE INTO checklist_tags (name, created_at, updated_at)
             VALUES (?1, ?2, ?3)",
            params![name, created_at, updated_at],
        )?;
        let local_id = conn
            .query_row(
                "SELECT id FROM checklist_tags WHERE name = ?1 COLLATE NOCASE",
                params![name],
                |row| row.get::<_, i64>(0),
            )?
            .to_string();
        Self::upsert_metadata(
            conn,
            "tag",
            &local_id,
            &record.sync_id,
            "synced",
            None,
            &record.updated_at,
        )?;
        Ok(true)
    }

    fn apply_todo(
        conn: &Connection,
        record: &ChecklistSyncRecord,
    ) -> Result<bool, rusqlite::Error> {
        let Some(category_sync_id) = record.payload.get("categorySyncId").and_then(Value::as_str)
        else {
            return Ok(false);
        };
        let category_id = Self::local_id_for_sync(conn, "category", category_sync_id)?
            .or_else(|| {
                Self::optional_string_field(&record.payload, "categoryName")
                    .and_then(|name| Self::local_id_for_category_name(conn, &name).ok().flatten())
            })
            .or_else(|| Self::sole_category_id(conn).ok().flatten());
        let Some(category_id) = category_id else {
            return Ok(false);
        };
        let text = Self::string_field(&record.payload, "text", "Item");
        let memo = Self::optional_string_field(&record.payload, "memo");
        let repeat_type = Self::string_field(&record.payload, "repeatType", "none");
        let repeat_detail = Self::optional_string_field(&record.payload, "repeatDetail");
        let next_due_at = Self::optional_string_field(&record.payload, "nextDueAt");
        let last_completed_at = Self::optional_string_field(&record.payload, "lastCompletedAt");
        let reminder_at = Self::optional_string_field(&record.payload, "reminderAt");
        let archived_at = Self::optional_string_field(&record.payload, "archivedAt");
        let track_streak = Self::bool_field(&record.payload, "trackStreak", false);
        let streak_started_on = Self::optional_string_field(&record.payload, "streakStartedOn");
        let done = Self::bool_field(&record.payload, "done", false);
        let display_order = Self::i64_field(&record.payload, "displayOrder", 1000);
        let created_at = Self::string_field(&record.payload, "createdAt", &record.updated_at);
        let updated_at = Self::string_field(&record.payload, "updatedAt", &record.updated_at);

        if let Some(local_id) = Self::local_id_for_sync(conn, "todo", &record.sync_id)? {
            conn.execute(
                "UPDATE checklist_todos
                 SET category_id = ?1, text = ?2, memo = ?3, repeat_type = ?4,
                     repeat_detail = ?5, next_due_at = ?6, last_completed_at = ?7,
                     reminder_at = ?8, archived_at = ?9, track_streak = ?10,
                     streak_started_on = ?11, done = ?12, display_order = ?13,
                     created_at = ?14, updated_at = ?15
                 WHERE id = ?16",
                params![
                    category_id,
                    text,
                    memo,
                    repeat_type,
                    repeat_detail,
                    next_due_at,
                    last_completed_at,
                    reminder_at,
                    archived_at,
                    track_streak,
                    streak_started_on,
                    done,
                    display_order,
                    created_at,
                    updated_at,
                    local_id
                ],
            )?;
            Self::upsert_metadata(
                conn,
                "todo",
                &local_id,
                &record.sync_id,
                "synced",
                None,
                &record.updated_at,
            )?;
            return Ok(true);
        }

        conn.execute(
            "INSERT INTO checklist_todos
                (category_id, text, memo, repeat_type, repeat_detail, next_due_at,
                 last_completed_at, reminder_at, archived_at, track_streak,
                 streak_started_on, done, display_order, created_at, updated_at)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, ?13, ?14, ?15)",
            params![
                category_id,
                text,
                memo,
                repeat_type,
                repeat_detail,
                next_due_at,
                last_completed_at,
                reminder_at,
                archived_at,
                track_streak,
                streak_started_on,
                done,
                display_order,
                created_at,
                updated_at
            ],
        )?;
        let local_id = conn.last_insert_rowid().to_string();
        Self::upsert_metadata(
            conn,
            "todo",
            &local_id,
            &record.sync_id,
            "synced",
            None,
            &record.updated_at,
        )?;
        Ok(true)
    }

    fn apply_todo_tag(
        conn: &Connection,
        record: &ChecklistSyncRecord,
    ) -> Result<bool, rusqlite::Error> {
        let Some(todo_sync_id) = record.payload.get("todoSyncId").and_then(Value::as_str) else {
            return Ok(false);
        };
        let Some(tag_sync_id) = record.payload.get("tagSyncId").and_then(Value::as_str) else {
            return Ok(false);
        };
        let Some(todo_id) = Self::local_id_for_sync(conn, "todo", todo_sync_id)? else {
            return Ok(false);
        };
        let Some(tag_id) = Self::local_id_for_sync(conn, "tag", tag_sync_id)? else {
            return Ok(false);
        };
        let created_at = Self::string_field(&record.payload, "createdAt", &record.updated_at);
        conn.execute(
            "INSERT OR IGNORE INTO checklist_todo_tags (todo_id, tag_id, created_at)
             VALUES (?1, ?2, ?3)",
            params![todo_id, tag_id, created_at],
        )?;
        let local_id = format!("{todo_id}:{tag_id}");
        Self::upsert_metadata(
            conn,
            "todo_tag",
            &local_id,
            &record.sync_id,
            "synced",
            None,
            &record.updated_at,
        )?;
        Ok(true)
    }

    fn apply_completion_log(
        conn: &Connection,
        record: &ChecklistSyncRecord,
    ) -> Result<bool, rusqlite::Error> {
        let Some(item_sync_id) = record.payload.get("itemSyncId").and_then(Value::as_str) else {
            return Ok(false);
        };
        let Some(item_id) = Self::local_id_for_sync(conn, "todo", item_sync_id)? else {
            return Ok(false);
        };
        let completed_on = Self::string_field(&record.payload, "completedOn", "");
        if completed_on.is_empty() {
            return Ok(false);
        }
        let completed_count = Self::i64_field(&record.payload, "completedCount", 1);
        let created_at = Self::string_field(&record.payload, "createdAt", &record.updated_at);
        let updated_at = Self::string_field(&record.payload, "updatedAt", &record.updated_at);
        conn.execute(
            "INSERT INTO checklist_completion_logs
                (item_id, completed_on, completed_count, created_at, updated_at)
             VALUES (?1, ?2, ?3, ?4, ?5)
             ON CONFLICT(item_id, completed_on) DO UPDATE SET
                completed_count = MAX(checklist_completion_logs.completed_count, excluded.completed_count),
                updated_at = CASE
                    WHEN excluded.updated_at > checklist_completion_logs.updated_at THEN excluded.updated_at
                    ELSE checklist_completion_logs.updated_at
                END",
            params![item_id, completed_on, completed_count, created_at, updated_at],
        )?;
        let local_id = format!("{item_id}:{completed_on}");
        Self::upsert_metadata(
            conn,
            "completion_log",
            &local_id,
            &record.sync_id,
            "synced",
            None,
            &record.updated_at,
        )?;
        Ok(true)
    }

    fn apply_setting(
        conn: &Connection,
        record: &ChecklistSyncRecord,
    ) -> Result<bool, rusqlite::Error> {
        let key = Self::string_field(&record.payload, "key", "");
        if key != "reset_time" {
            return Ok(false);
        }
        let value = Self::string_field(&record.payload, "value", "00:00");
        conn.execute(
            "INSERT INTO settings (key, value)
             VALUES ('reset_time', ?1)
             ON CONFLICT(key) DO UPDATE SET value = excluded.value",
            params![value],
        )?;
        Self::upsert_metadata(
            conn,
            "setting",
            "reset_time",
            &record.sync_id,
            "synced",
            None,
            &record.updated_at,
        )?;
        Ok(true)
    }

    pub fn mark_records_synced(
        conn: &Connection,
        sync_ids: &[String],
    ) -> Result<(), rusqlite::Error> {
        let now = Self::now_iso();
        for sync_id in sync_ids {
            conn.execute(
                "UPDATE checklist_sync_metadata
                 SET sync_status = 'synced', last_synced_at = ?1
                 WHERE sync_id = ?2",
                params![&now, sync_id],
            )?;
        }
        Self::set_state(conn, "last_synced_at", &now)?;
        Self::set_state(conn, "last_error", "")?;
        Ok(())
    }

    fn upsert_metadata(
        conn: &Connection,
        entity_type: &str,
        local_id: &str,
        sync_id: &str,
        sync_status: &str,
        deleted_at: Option<&str>,
        updated_at: &str,
    ) -> Result<(), rusqlite::Error> {
        conn.execute(
            "INSERT INTO checklist_sync_metadata
                (entity_type, local_id, sync_id, sync_status, deleted_at, updated_at)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6)
             ON CONFLICT(entity_type, local_id) DO UPDATE SET
                sync_id = excluded.sync_id,
                sync_status = excluded.sync_status,
                deleted_at = excluded.deleted_at,
                updated_at = excluded.updated_at",
            params![
                entity_type,
                local_id,
                sync_id,
                sync_status,
                deleted_at,
                updated_at
            ],
        )?;
        Ok(())
    }

    fn sync_id_for_local(
        conn: &Connection,
        entity_type: &str,
        local_id: &str,
    ) -> Result<Option<String>, rusqlite::Error> {
        conn.query_row(
            "SELECT sync_id FROM checklist_sync_metadata
             WHERE entity_type = ?1 AND local_id = ?2",
            params![entity_type, local_id],
            |row| row.get(0),
        )
        .optional()
    }

    fn category_name_for_local(
        conn: &Connection,
        local_id: &str,
    ) -> Result<Option<String>, rusqlite::Error> {
        conn.query_row(
            "SELECT name FROM checklist_categories WHERE id = ?1",
            params![local_id],
            |row| row.get(0),
        )
        .optional()
    }

    fn local_id_for_category_name(
        conn: &Connection,
        name: &str,
    ) -> Result<Option<String>, rusqlite::Error> {
        conn.query_row(
            "SELECT id FROM checklist_categories WHERE name = ?1",
            params![name],
            |row| row.get::<_, i64>(0),
        )
        .optional()
        .map(|value| value.map(|id| id.to_string()))
    }

    fn sole_category_id(conn: &Connection) -> Result<Option<String>, rusqlite::Error> {
        let count: i64 =
            conn.query_row("SELECT COUNT(*) FROM checklist_categories", [], |row| {
                row.get(0)
            })?;
        if count != 1 {
            return Ok(None);
        }
        conn.query_row("SELECT id FROM checklist_categories LIMIT 1", [], |row| {
            row.get::<_, i64>(0)
        })
        .optional()
        .map(|value| value.map(|id| id.to_string()))
    }

    fn local_id_for_sync(
        conn: &Connection,
        entity_type: &str,
        sync_id: &str,
    ) -> Result<Option<String>, rusqlite::Error> {
        conn.query_row(
            "SELECT local_id FROM checklist_sync_metadata
             WHERE entity_type = ?1 AND sync_id = ?2",
            params![entity_type, sync_id],
            |row| row.get(0),
        )
        .optional()
    }

    fn parse_pair(local_id: &str) -> Option<(i64, i64)> {
        let (left, right) = local_id.split_once(':')?;
        Some((left.parse().ok()?, right.parse().ok()?))
    }

    fn parse_log_key(local_id: &str) -> Option<(i64, String)> {
        let (left, right) = local_id.split_once(':')?;
        Some((left.parse().ok()?, right.to_string()))
    }

    fn string_field(payload: &Value, key: &str, fallback: &str) -> String {
        payload
            .get(key)
            .and_then(Value::as_str)
            .unwrap_or(fallback)
            .to_string()
    }

    fn optional_string_field(payload: &Value, key: &str) -> Option<String> {
        payload
            .get(key)
            .and_then(Value::as_str)
            .map(ToString::to_string)
    }

    fn i64_field(payload: &Value, key: &str, fallback: i64) -> i64 {
        payload.get(key).and_then(Value::as_i64).unwrap_or(fallback)
    }

    fn bool_field(payload: &Value, key: &str, fallback: bool) -> bool {
        payload
            .get(key)
            .and_then(Value::as_bool)
            .unwrap_or(fallback)
    }

    fn now_iso() -> String {
        chrono::Utc::now().format("%Y-%m-%dT%H:%M:%SZ").to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::repository::ChecklistRepository;

    fn setup_conn() -> Connection {
        let conn = Connection::open_in_memory().expect("in-memory db");
        conn.execute(
            "CREATE TABLE IF NOT EXISTS settings (
                key TEXT PRIMARY KEY,
                value TEXT NOT NULL
            )",
            [],
        )
        .expect("settings schema");
        ChecklistRepository::create_tables(&conn).expect("checklist schema");
        SyncRepository::create_tables(&conn).expect("sync schema");
        ChecklistRepository::ensure_default_category(&conn).expect("default category");
        conn
    }

    #[test]
    fn export_records_assigns_sync_ids_to_existing_rows() {
        let conn = setup_conn();

        let records = SyncRepository::export_records(&conn).expect("records");
        let home = records
            .iter()
            .find(|record| record.entity_type == "category")
            .expect("home category record");

        assert!(!home.sync_id.is_empty());
        assert_eq!(home.deleted_at, None);
        assert_eq!(home.payload["name"], "Home");
    }

    #[test]
    fn export_records_keeps_synced_records_for_full_foreground_exchange() {
        let conn = setup_conn();
        let initial_records = SyncRepository::export_records(&conn).expect("initial records");
        let initial_sync_ids = initial_records
            .iter()
            .map(|record| record.sync_id.clone())
            .collect::<Vec<_>>();

        SyncRepository::mark_records_synced(&conn, &initial_sync_ids).expect("mark synced");
        let synced_records = SyncRepository::export_records(&conn).expect("synced records");
        assert_eq!(synced_records.len(), initial_records.len());

        let home_id: i64 = conn
            .query_row("SELECT id FROM checklist_categories LIMIT 1", [], |row| {
                row.get(0)
            })
            .expect("home id");
        ChecklistRepository::create_item(&conn, home_id, "Wallet", &[]).expect("created item");

        let records = SyncRepository::export_records(&conn).expect("records");
        let wallet = records
            .iter()
            .find(|record| record.entity_type == "todo" && record.payload["text"] == "Wallet")
            .expect("wallet record");
        assert_eq!(wallet.payload["categoryName"], "Home");
    }

    #[test]
    fn deleting_item_exports_tombstone_record() {
        let conn = setup_conn();
        let home_id: i64 = conn
            .query_row("SELECT id FROM checklist_categories LIMIT 1", [], |row| {
                row.get(0)
            })
            .expect("home id");
        let item =
            ChecklistRepository::create_item(&conn, home_id, "Wallet", &[]).expect("created item");
        let before_delete = SyncRepository::export_records(&conn).expect("before delete");
        let item_sync_id = before_delete
            .iter()
            .find(|record| record.entity_type == "todo" && record.payload["text"] == "Wallet")
            .map(|record| record.sync_id.clone())
            .expect("item sync id");

        ChecklistRepository::delete_item(&conn, item.id).expect("delete item");
        let after_delete = SyncRepository::export_records(&conn).expect("after delete");
        let tombstone = after_delete
            .iter()
            .find(|record| record.sync_id == item_sync_id)
            .expect("tombstone");

        assert_eq!(tombstone.entity_type, "todo");
        assert!(tombstone.deleted_at.is_some());
    }

    #[test]
    fn apply_remote_completion_log_keeps_larger_count() {
        let conn = setup_conn();
        let home_id: i64 = conn
            .query_row("SELECT id FROM checklist_categories LIMIT 1", [], |row| {
                row.get(0)
            })
            .expect("home id");
        let item =
            ChecklistRepository::create_item(&conn, home_id, "Read", &[]).expect("created item");
        let records = SyncRepository::export_records(&conn).expect("records");
        let item_sync_id = records
            .iter()
            .find(|record| record.entity_type == "todo" && record.payload["text"] == "Read")
            .map(|record| record.sync_id.clone())
            .expect("item sync id");
        let remote_record = ChecklistSyncRecord {
            entity_type: "completion_log".to_string(),
            sync_id: "remote-log-1".to_string(),
            updated_at: "2026-06-19T00:00:00Z".to_string(),
            deleted_at: None,
            payload: json!({
                "itemSyncId": item_sync_id,
                "completedOn": "2026-06-19",
                "completedCount": 3,
                "createdAt": "2026-06-19T00:00:00Z",
                "updatedAt": "2026-06-19T00:00:00Z"
            }),
        };

        SyncRepository::apply_remote_records(&conn, &[remote_record]).expect("apply remote");
        let count: i64 = conn
            .query_row(
                "SELECT completed_count FROM checklist_completion_logs WHERE item_id = ?1 AND completed_on = '2026-06-19'",
                params![item.id],
                |row| row.get(0),
            )
            .expect("completion count");

        assert_eq!(count, 3);
    }

    #[test]
    fn apply_remote_tag_reuses_existing_name() {
        let conn = setup_conn();
        conn.execute(
            "INSERT INTO checklist_tags (name, created_at, updated_at)
             VALUES ('work', '2026-06-19T00:00:00Z', '2026-06-19T00:00:00Z')",
            [],
        )
        .expect("local tag");

        let remote_record = ChecklistSyncRecord {
            entity_type: "tag".to_string(),
            sync_id: "remote-tag-work".to_string(),
            updated_at: "2026-06-19T01:00:00Z".to_string(),
            deleted_at: None,
            payload: json!({
                "name": "Work",
                "createdAt": "2026-06-19T01:00:00Z",
                "updatedAt": "2026-06-19T01:00:00Z"
            }),
        };

        SyncRepository::apply_remote_records(&conn, &[remote_record]).expect("apply remote tag");
        let tag_count: i64 = conn
            .query_row(
                "SELECT COUNT(*) FROM checklist_tags WHERE name = 'work' COLLATE NOCASE",
                [],
                |row| row.get(0),
            )
            .expect("tag count");

        assert_eq!(tag_count, 1);
    }

    #[test]
    fn apply_remote_todo_falls_back_to_category_name_when_sync_mapping_is_missing() {
        let conn = setup_conn();
        let remote_record = ChecklistSyncRecord {
            entity_type: "todo".to_string(),
            sync_id: "remote-todo-wallet".to_string(),
            updated_at: "2026-06-19T01:00:00Z".to_string(),
            deleted_at: None,
            payload: json!({
                "categorySyncId": "remote-home-category",
                "categoryName": "Home",
                "text": "Wallet",
                "memo": null,
                "repeatType": "none",
                "repeatDetail": null,
                "nextDueAt": null,
                "lastCompletedAt": null,
                "reminderAt": null,
                "archivedAt": null,
                "trackStreak": false,
                "streakStartedOn": null,
                "done": false,
                "displayOrder": 1000,
                "createdAt": "2026-06-19T01:00:00Z",
                "updatedAt": "2026-06-19T01:00:00Z"
            }),
        };

        let applied = SyncRepository::apply_remote_records(&conn, &[remote_record])
            .expect("apply remote todo");
        let item_count: i64 = conn
            .query_row(
                "SELECT COUNT(*) FROM checklist_todos WHERE text = 'Wallet'",
                [],
                |row| row.get(0),
            )
            .expect("item count");

        assert_eq!(applied, 1);
        assert_eq!(item_count, 1);
    }

    #[test]
    fn apply_remote_category_delete_cascades_children() {
        let conn = setup_conn();
        let home_id: i64 = conn
            .query_row("SELECT id FROM checklist_categories LIMIT 1", [], |row| {
                row.get(0)
            })
            .expect("home id");
        let item =
            ChecklistRepository::create_item(&conn, home_id, "Wallet", &["work".to_string()])
                .expect("created item");
        conn.execute(
            "INSERT INTO checklist_completion_logs
                (item_id, completed_on, completed_count, created_at, updated_at)
             VALUES (?1, '2026-06-19', 1, '2026-06-19T00:00:00Z', '2026-06-19T00:00:00Z')",
            params![item.id],
        )
        .expect("completion log");
        let records = SyncRepository::export_records(&conn).expect("records");
        let category_sync_id = records
            .iter()
            .find(|record| record.entity_type == "category" && record.payload["name"] == "Home")
            .map(|record| record.sync_id.clone())
            .expect("category sync id");
        let tombstone = ChecklistSyncRecord {
            entity_type: "category".to_string(),
            sync_id: category_sync_id,
            updated_at: "2026-06-19T01:00:00Z".to_string(),
            deleted_at: Some("2026-06-19T01:00:00Z".to_string()),
            payload: json!({}),
        };

        SyncRepository::apply_remote_records(&conn, &[tombstone]).expect("apply remote delete");
        let todo_count: i64 = conn
            .query_row("SELECT COUNT(*) FROM checklist_todos", [], |row| row.get(0))
            .expect("todo count");
        let relation_count: i64 = conn
            .query_row("SELECT COUNT(*) FROM checklist_todo_tags", [], |row| {
                row.get(0)
            })
            .expect("relation count");
        let log_count: i64 = conn
            .query_row(
                "SELECT COUNT(*) FROM checklist_completion_logs",
                [],
                |row| row.get(0),
            )
            .expect("log count");

        assert_eq!(todo_count, 0);
        assert_eq!(relation_count, 0);
        assert_eq!(log_count, 0);
    }
}
