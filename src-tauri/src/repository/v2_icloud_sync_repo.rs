use rusqlite::{params, Connection, OptionalExtension};
use serde_json::{json, Value};
use uuid::Uuid;

use crate::models::{V2ICloudRecord, V2ICloudSyncEntity};

pub struct V2ICloudSyncRepository;

impl V2ICloudSyncRepository {
    pub fn create_tables(conn: &Connection) -> Result<(), rusqlite::Error> {
        conn.execute_batch(
            "CREATE TABLE IF NOT EXISTS v2_sync_metadata (
                entity TEXT NOT NULL,
                local_key TEXT NOT NULL,
                sync_id TEXT NOT NULL UNIQUE,
                sync_status TEXT NOT NULL DEFAULT 'pending',
                deleted_at TEXT,
                last_synced_at TEXT,
                PRIMARY KEY (entity, local_key)
            );

            CREATE INDEX IF NOT EXISTS idx_v2_sync_metadata_sync_id
                ON v2_sync_metadata(sync_id);

            CREATE INDEX IF NOT EXISTS idx_v2_sync_metadata_status
                ON v2_sync_metadata(sync_status, deleted_at);",
        )
    }

    pub fn ensure_active_metadata(conn: &Connection) -> Result<(), rusqlite::Error> {
        Self::create_tables(conn)?;
        Self::ensure_simple_entity_metadata(
            conn,
            V2ICloudSyncEntity::Category,
            "SELECT id FROM v2_categories",
        )?;
        Self::ensure_simple_entity_metadata(
            conn,
            V2ICloudSyncEntity::Todo,
            "SELECT id FROM v2_todos",
        )?;
        Self::ensure_simple_entity_metadata(
            conn,
            V2ICloudSyncEntity::Tag,
            "SELECT id FROM v2_tags",
        )?;
        Self::ensure_todo_tag_metadata(conn)?;
        Self::ensure_completion_log_metadata(conn)
    }

    pub fn export_records(conn: &Connection) -> Result<Vec<V2ICloudRecord>, rusqlite::Error> {
        Self::ensure_active_metadata(conn)?;
        let mut records = Vec::new();
        records.extend(Self::export_categories(conn)?);
        records.extend(Self::export_tags(conn)?);
        records.extend(Self::export_todos(conn)?);
        records.extend(Self::export_todo_tags(conn)?);
        records.extend(Self::export_completion_logs(conn)?);
        records.extend(Self::export_tombstones(conn)?);
        Ok(records)
    }

    pub fn apply_remote_records(
        conn: &Connection,
        records: &[V2ICloudRecord],
    ) -> Result<i64, rusqlite::Error> {
        Self::create_tables(conn)?;
        let mut applied = 0;

        for entity in [
            V2ICloudSyncEntity::Category,
            V2ICloudSyncEntity::Tag,
            V2ICloudSyncEntity::Todo,
            V2ICloudSyncEntity::TodoTag,
            V2ICloudSyncEntity::CompletionLog,
        ] {
            for record in records
                .iter()
                .filter(|record| record.entity == entity.as_str() && record.deleted_at.is_none())
            {
                if Self::apply_active_record(conn, record)? {
                    applied += 1;
                }
            }
        }

        for entity in [
            V2ICloudSyncEntity::TodoTag,
            V2ICloudSyncEntity::CompletionLog,
            V2ICloudSyncEntity::Todo,
            V2ICloudSyncEntity::Tag,
            V2ICloudSyncEntity::Category,
        ] {
            for record in records
                .iter()
                .filter(|record| record.entity == entity.as_str() && record.deleted_at.is_some())
            {
                if Self::apply_tombstone(conn, record)? {
                    applied += 1;
                }
            }
        }

        Ok(applied)
    }

    pub fn mark_all_synced(conn: &Connection, synced_at: &str) -> Result<(), rusqlite::Error> {
        Self::create_tables(conn)?;
        conn.execute(
            "UPDATE v2_sync_metadata
             SET sync_status = 'synced',
                 last_synced_at = ?1",
            params![synced_at],
        )?;
        Ok(())
    }

    pub fn tombstone_category_tree(
        conn: &Connection,
        category_id: i64,
    ) -> Result<(), rusqlite::Error> {
        Self::ensure_active_metadata(conn)?;
        let now = Self::now_iso();
        let category_key = category_id.to_string();
        Self::mark_tombstone(conn, V2ICloudSyncEntity::Category, &category_key, &now)?;

        let mut stmt = conn.prepare("SELECT id FROM v2_todos WHERE category_id = ?1")?;
        let todo_ids = stmt
            .query_map(params![category_id], |row| row.get::<_, i64>(0))?
            .collect::<Result<Vec<_>, _>>()?;

        for todo_id in todo_ids {
            Self::tombstone_item_tree_with_time(conn, todo_id, &now)?;
        }
        Ok(())
    }

    pub fn tombstone_item_tree(conn: &Connection, item_id: i64) -> Result<(), rusqlite::Error> {
        Self::ensure_active_metadata(conn)?;
        let now = Self::now_iso();
        Self::tombstone_item_tree_with_time(conn, item_id, &now)
    }

    pub fn tombstone_tag(conn: &Connection, tag_id: i64) -> Result<(), rusqlite::Error> {
        Self::ensure_active_metadata(conn)?;
        let now = Self::now_iso();
        let mut stmt = conn.prepare("SELECT todo_id FROM v2_todo_tags WHERE tag_id = ?1")?;
        let todo_ids = stmt
            .query_map(params![tag_id], |row| row.get::<_, i64>(0))?
            .collect::<Result<Vec<_>, _>>()?;
        for todo_id in todo_ids {
            let key = Self::todo_tag_local_key(todo_id, tag_id);
            Self::mark_tombstone(conn, V2ICloudSyncEntity::TodoTag, &key, &now)?;
        }

        Self::mark_tombstone(conn, V2ICloudSyncEntity::Tag, &tag_id.to_string(), &now)
    }

    pub fn last_synced_at(conn: &Connection) -> Result<Option<String>, rusqlite::Error> {
        Self::create_tables(conn)?;
        conn.query_row(
            "SELECT MAX(last_synced_at) FROM v2_sync_metadata WHERE last_synced_at IS NOT NULL",
            [],
            |row| row.get(0),
        )
    }

    fn ensure_simple_entity_metadata(
        conn: &Connection,
        entity: V2ICloudSyncEntity,
        sql: &str,
    ) -> Result<(), rusqlite::Error> {
        let mut stmt = conn.prepare(sql)?;
        let ids = stmt
            .query_map([], |row| row.get::<_, i64>(0))?
            .collect::<Result<Vec<_>, _>>()?;
        for id in ids {
            Self::ensure_metadata(conn, entity.as_str(), &id.to_string(), None)?;
        }
        Ok(())
    }

    fn ensure_todo_tag_metadata(conn: &Connection) -> Result<(), rusqlite::Error> {
        let mut stmt = conn.prepare("SELECT todo_id, tag_id FROM v2_todo_tags")?;
        let rows = stmt
            .query_map([], |row| Ok((row.get::<_, i64>(0)?, row.get::<_, i64>(1)?)))?
            .collect::<Result<Vec<_>, _>>()?;
        for (todo_id, tag_id) in rows {
            let todo_sync_id = Self::sync_id_for_local(conn, V2ICloudSyncEntity::Todo, todo_id)?;
            let tag_sync_id = Self::sync_id_for_local(conn, V2ICloudSyncEntity::Tag, tag_id)?;
            let sync_id = format!("todo_tag:{todo_sync_id}:{tag_sync_id}");
            Self::ensure_metadata(
                conn,
                V2ICloudSyncEntity::TodoTag.as_str(),
                &Self::todo_tag_local_key(todo_id, tag_id),
                Some(&sync_id),
            )?;
        }
        Ok(())
    }

    fn ensure_completion_log_metadata(conn: &Connection) -> Result<(), rusqlite::Error> {
        let mut stmt = conn.prepare("SELECT item_id, completed_on FROM v2_completion_logs")?;
        let rows = stmt
            .query_map([], |row| {
                Ok((row.get::<_, i64>(0)?, row.get::<_, String>(1)?))
            })?
            .collect::<Result<Vec<_>, _>>()?;
        for (item_id, completed_on) in rows {
            let item_sync_id = Self::sync_id_for_local(conn, V2ICloudSyncEntity::Todo, item_id)?;
            let sync_id = format!("completion_log:{item_sync_id}:{completed_on}");
            Self::ensure_metadata(
                conn,
                V2ICloudSyncEntity::CompletionLog.as_str(),
                &Self::completion_log_local_key(item_id, &completed_on),
                Some(&sync_id),
            )?;
        }
        Ok(())
    }

    fn ensure_metadata(
        conn: &Connection,
        entity: &str,
        local_key: &str,
        preferred_sync_id: Option<&str>,
    ) -> Result<String, rusqlite::Error> {
        if let Some(sync_id) = Self::metadata_sync_id(conn, entity, local_key)? {
            return Ok(sync_id);
        }

        let sync_id = preferred_sync_id
            .map(ToOwned::to_owned)
            .unwrap_or_else(|| Uuid::new_v4().to_string());
        conn.execute(
            "INSERT OR IGNORE INTO v2_sync_metadata
                (entity, local_key, sync_id, sync_status)
             VALUES (?1, ?2, ?3, 'pending')",
            params![entity, local_key, sync_id],
        )?;
        Self::metadata_sync_id(conn, entity, local_key).map(|value| value.unwrap_or(sync_id))
    }

    fn metadata_sync_id(
        conn: &Connection,
        entity: &str,
        local_key: &str,
    ) -> Result<Option<String>, rusqlite::Error> {
        conn.query_row(
            "SELECT sync_id FROM v2_sync_metadata WHERE entity = ?1 AND local_key = ?2",
            params![entity, local_key],
            |row| row.get(0),
        )
        .optional()
    }

    fn sync_id_for_local(
        conn: &Connection,
        entity: V2ICloudSyncEntity,
        local_id: i64,
    ) -> Result<String, rusqlite::Error> {
        Self::ensure_metadata(conn, entity.as_str(), &local_id.to_string(), None)
    }

    fn local_key_for_sync_id(
        conn: &Connection,
        sync_id: &str,
    ) -> Result<Option<(String, String)>, rusqlite::Error> {
        conn.query_row(
            "SELECT entity, local_key FROM v2_sync_metadata WHERE sync_id = ?1",
            params![sync_id],
            |row| Ok((row.get::<_, String>(0)?, row.get::<_, String>(1)?)),
        )
        .optional()
    }

    fn upsert_metadata_for_remote(
        conn: &Connection,
        entity: V2ICloudSyncEntity,
        local_key: &str,
        sync_id: &str,
    ) -> Result<(), rusqlite::Error> {
        conn.execute(
            "INSERT INTO v2_sync_metadata
                (entity, local_key, sync_id, sync_status)
             VALUES (?1, ?2, ?3, 'synced')
             ON CONFLICT(entity, local_key) DO UPDATE SET
                sync_id = excluded.sync_id,
                sync_status = 'synced',
                deleted_at = NULL",
            params![entity.as_str(), local_key, sync_id],
        )?;
        Ok(())
    }

    fn export_categories(conn: &Connection) -> Result<Vec<V2ICloudRecord>, rusqlite::Error> {
        let mut stmt = conn.prepare(
            "SELECT m.sync_id, c.id, c.name, c.display_order, c.created_at, c.updated_at
             FROM v2_categories c
             INNER JOIN v2_sync_metadata m
                ON m.entity = 'category' AND m.local_key = CAST(c.id AS TEXT)
             WHERE m.deleted_at IS NULL",
        )?;
        let rows = stmt.query_map([], |row| {
            let updated_at: String = row.get(5)?;
            Ok(Self::active_record(
                V2ICloudSyncEntity::Category,
                row.get(0)?,
                updated_at,
                json!({
                    "name": row.get::<_, String>(2)?,
                    "display_order": row.get::<_, i64>(3)?,
                    "created_at": row.get::<_, String>(4)?,
                    "updated_at": row.get::<_, String>(5)?,
                }),
            ))
        })?;
        rows.collect()
    }

    fn export_tags(conn: &Connection) -> Result<Vec<V2ICloudRecord>, rusqlite::Error> {
        let mut stmt = conn.prepare(
            "SELECT m.sync_id, tg.name, tg.created_at, tg.updated_at
             FROM v2_tags tg
             INNER JOIN v2_sync_metadata m
                ON m.entity = 'tag' AND m.local_key = CAST(tg.id AS TEXT)
             WHERE m.deleted_at IS NULL",
        )?;
        let rows = stmt.query_map([], |row| {
            let updated_at: String = row.get(3)?;
            Ok(Self::active_record(
                V2ICloudSyncEntity::Tag,
                row.get(0)?,
                updated_at,
                json!({
                    "name": row.get::<_, String>(1)?,
                    "created_at": row.get::<_, String>(2)?,
                    "updated_at": row.get::<_, String>(3)?,
                }),
            ))
        })?;
        rows.collect()
    }

    fn export_todos(conn: &Connection) -> Result<Vec<V2ICloudRecord>, rusqlite::Error> {
        let mut stmt = conn.prepare(
            "SELECT m.sync_id,
                    cm.sync_id,
                    t.text,
                    t.memo,
                    t.repeat_type,
                    t.repeat_detail,
                    t.next_due_at,
                    t.last_completed_at,
                    t.reminder_at,
                    t.archived_at,
                    t.track_streak,
                    t.streak_started_on,
                    t.done,
                    t.display_order,
                    t.created_at,
                    t.updated_at
             FROM v2_todos t
             INNER JOIN v2_sync_metadata m
                ON m.entity = 'todo' AND m.local_key = CAST(t.id AS TEXT)
             INNER JOIN v2_sync_metadata cm
                ON cm.entity = 'category' AND cm.local_key = CAST(t.category_id AS TEXT)
             WHERE m.deleted_at IS NULL",
        )?;
        let rows = stmt.query_map([], |row| {
            let updated_at: String = row.get(15)?;
            Ok(Self::active_record(
                V2ICloudSyncEntity::Todo,
                row.get(0)?,
                updated_at,
                json!({
                    "category_sync_id": row.get::<_, String>(1)?,
                    "text": row.get::<_, String>(2)?,
                    "memo": row.get::<_, Option<String>>(3)?,
                    "repeat_type": row.get::<_, String>(4)?,
                    "repeat_detail": row.get::<_, Option<String>>(5)?,
                    "next_due_at": row.get::<_, Option<String>>(6)?,
                    "last_completed_at": row.get::<_, Option<String>>(7)?,
                    "reminder_at": row.get::<_, Option<String>>(8)?,
                    "archived_at": row.get::<_, Option<String>>(9)?,
                    "track_streak": row.get::<_, bool>(10)?,
                    "streak_started_on": row.get::<_, Option<String>>(11)?,
                    "done": row.get::<_, bool>(12)?,
                    "display_order": row.get::<_, i64>(13)?,
                    "created_at": row.get::<_, String>(14)?,
                    "updated_at": row.get::<_, String>(15)?,
                }),
            ))
        })?;
        rows.collect()
    }

    fn export_todo_tags(conn: &Connection) -> Result<Vec<V2ICloudRecord>, rusqlite::Error> {
        let mut stmt = conn.prepare(
            "SELECT m.sync_id, tm.sync_id, gm.sync_id, tt.created_at
             FROM v2_todo_tags tt
             INNER JOIN v2_sync_metadata m
                ON m.entity = 'todo_tag'
               AND m.local_key = CAST(tt.todo_id AS TEXT) || ':' || CAST(tt.tag_id AS TEXT)
             INNER JOIN v2_sync_metadata tm
                ON tm.entity = 'todo' AND tm.local_key = CAST(tt.todo_id AS TEXT)
             INNER JOIN v2_sync_metadata gm
                ON gm.entity = 'tag' AND gm.local_key = CAST(tt.tag_id AS TEXT)
             WHERE m.deleted_at IS NULL",
        )?;
        let rows = stmt.query_map([], |row| {
            let created_at: String = row.get(3)?;
            Ok(Self::active_record(
                V2ICloudSyncEntity::TodoTag,
                row.get(0)?,
                created_at.clone(),
                json!({
                    "todo_sync_id": row.get::<_, String>(1)?,
                    "tag_sync_id": row.get::<_, String>(2)?,
                    "created_at": created_at,
                    "updated_at": created_at,
                }),
            ))
        })?;
        rows.collect()
    }

    fn export_completion_logs(conn: &Connection) -> Result<Vec<V2ICloudRecord>, rusqlite::Error> {
        let mut stmt = conn.prepare(
            "SELECT m.sync_id,
                    tm.sync_id,
                    cl.completed_on,
                    cl.completed_count,
                    cl.created_at,
                    cl.updated_at
             FROM v2_completion_logs cl
             INNER JOIN v2_sync_metadata m
                ON m.entity = 'completion_log'
               AND m.local_key = CAST(cl.item_id AS TEXT) || ':' || cl.completed_on
             INNER JOIN v2_sync_metadata tm
                ON tm.entity = 'todo' AND tm.local_key = CAST(cl.item_id AS TEXT)
             WHERE m.deleted_at IS NULL",
        )?;
        let rows = stmt.query_map([], |row| {
            let updated_at: String = row.get(5)?;
            Ok(Self::active_record(
                V2ICloudSyncEntity::CompletionLog,
                row.get(0)?,
                updated_at,
                json!({
                    "item_sync_id": row.get::<_, String>(1)?,
                    "completed_on": row.get::<_, String>(2)?,
                    "completed_count": row.get::<_, i64>(3)?,
                    "created_at": row.get::<_, String>(4)?,
                    "updated_at": row.get::<_, String>(5)?,
                }),
            ))
        })?;
        rows.collect()
    }

    fn export_tombstones(conn: &Connection) -> Result<Vec<V2ICloudRecord>, rusqlite::Error> {
        let mut stmt = conn.prepare(
            "SELECT entity, sync_id, deleted_at
             FROM v2_sync_metadata
             WHERE deleted_at IS NOT NULL
               AND sync_status != 'synced'",
        )?;
        let rows = stmt.query_map([], |row| {
            let entity = Self::entity_from_str(&row.get::<_, String>(0)?);
            let deleted_at: String = row.get(2)?;
            Ok(V2ICloudRecord {
                record_type: entity.cloud_record_type().to_string(),
                entity: entity.as_str().to_string(),
                sync_id: row.get(1)?,
                updated_at: deleted_at.clone(),
                deleted_at: Some(deleted_at),
                payload: json!({}),
            })
        })?;
        rows.collect()
    }

    fn active_record(
        entity: V2ICloudSyncEntity,
        sync_id: String,
        updated_at: String,
        payload: Value,
    ) -> V2ICloudRecord {
        V2ICloudRecord {
            record_type: entity.cloud_record_type().to_string(),
            entity: entity.as_str().to_string(),
            sync_id,
            updated_at,
            deleted_at: None,
            payload,
        }
    }

    fn apply_active_record(
        conn: &Connection,
        record: &V2ICloudRecord,
    ) -> Result<bool, rusqlite::Error> {
        match record.entity.as_str() {
            "category" => Self::apply_category(conn, record),
            "tag" => Self::apply_tag(conn, record),
            "todo" => Self::apply_todo(conn, record),
            "todo_tag" => Self::apply_todo_tag(conn, record),
            "completion_log" => Self::apply_completion_log(conn, record),
            _ => Ok(false),
        }
    }

    fn apply_category(conn: &Connection, record: &V2ICloudRecord) -> Result<bool, rusqlite::Error> {
        let name =
            Self::payload_string(&record.payload, "name").unwrap_or_else(|| "Untitled".to_string());
        let display_order = Self::payload_i64(&record.payload, "display_order").unwrap_or(1000);
        let created_at = Self::payload_string(&record.payload, "created_at")
            .unwrap_or_else(|| record.updated_at.clone());
        let updated_at = Self::payload_string(&record.payload, "updated_at")
            .unwrap_or_else(|| record.updated_at.clone());

        if let Some(local_id) = Self::local_id_for_sync(conn, &record.sync_id)? {
            let local_updated_at: String = conn.query_row(
                "SELECT updated_at FROM v2_categories WHERE id = ?1",
                params![local_id],
                |row| row.get(0),
            )?;
            if local_updated_at > updated_at {
                return Ok(false);
            }
            conn.execute(
                "UPDATE v2_categories
                 SET name = ?1, display_order = ?2, updated_at = ?3
                 WHERE id = ?4",
                params![name, display_order, updated_at, local_id],
            )?;
            return Ok(true);
        }

        conn.execute(
            "INSERT OR IGNORE INTO v2_categories (name, display_order, created_at, updated_at)
             VALUES (?1, ?2, ?3, ?4)",
            params![name, display_order, created_at, updated_at],
        )?;
        let local_id: i64 = conn.query_row(
            "SELECT id FROM v2_categories WHERE name = ?1",
            params![name],
            |row| row.get(0),
        )?;
        Self::upsert_metadata_for_remote(
            conn,
            V2ICloudSyncEntity::Category,
            &local_id.to_string(),
            &record.sync_id,
        )?;
        Ok(true)
    }

    fn apply_tag(conn: &Connection, record: &V2ICloudRecord) -> Result<bool, rusqlite::Error> {
        let name =
            Self::payload_string(&record.payload, "name").unwrap_or_else(|| "tag".to_string());
        let created_at = Self::payload_string(&record.payload, "created_at")
            .unwrap_or_else(|| record.updated_at.clone());
        let updated_at = Self::payload_string(&record.payload, "updated_at")
            .unwrap_or_else(|| record.updated_at.clone());

        if let Some(local_id) = Self::local_id_for_sync(conn, &record.sync_id)? {
            let local_updated_at: String = conn.query_row(
                "SELECT updated_at FROM v2_tags WHERE id = ?1",
                params![local_id],
                |row| row.get(0),
            )?;
            if local_updated_at > updated_at {
                return Ok(false);
            }
            conn.execute(
                "UPDATE v2_tags SET name = ?1, updated_at = ?2 WHERE id = ?3",
                params![name, updated_at, local_id],
            )?;
            return Ok(true);
        }

        conn.execute(
            "INSERT OR IGNORE INTO v2_tags (name, created_at, updated_at)
             VALUES (?1, ?2, ?3)",
            params![name, created_at, updated_at],
        )?;
        let local_id: i64 = conn.query_row(
            "SELECT id FROM v2_tags WHERE name = ?1 COLLATE NOCASE",
            params![name],
            |row| row.get(0),
        )?;
        Self::upsert_metadata_for_remote(
            conn,
            V2ICloudSyncEntity::Tag,
            &local_id.to_string(),
            &record.sync_id,
        )?;
        Ok(true)
    }

    fn apply_todo(conn: &Connection, record: &V2ICloudRecord) -> Result<bool, rusqlite::Error> {
        let Some(category_sync_id) = Self::payload_string(&record.payload, "category_sync_id")
        else {
            return Ok(false);
        };
        let Some(category_id) = Self::local_id_for_sync(conn, &category_sync_id)? else {
            return Ok(false);
        };
        let text =
            Self::payload_string(&record.payload, "text").unwrap_or_else(|| "Untitled".to_string());
        let updated_at = Self::payload_string(&record.payload, "updated_at")
            .unwrap_or_else(|| record.updated_at.clone());
        let created_at = Self::payload_string(&record.payload, "created_at")
            .unwrap_or_else(|| updated_at.clone());

        if let Some(local_id) = Self::local_id_for_sync(conn, &record.sync_id)? {
            let local_updated_at: String = conn.query_row(
                "SELECT updated_at FROM v2_todos WHERE id = ?1",
                params![local_id],
                |row| row.get(0),
            )?;
            if local_updated_at > updated_at {
                return Ok(false);
            }
            conn.execute(
                "UPDATE v2_todos
                 SET category_id = ?1,
                     text = ?2,
                     memo = ?3,
                     repeat_type = ?4,
                     repeat_detail = ?5,
                     next_due_at = ?6,
                     last_completed_at = ?7,
                     reminder_at = ?8,
                     archived_at = ?9,
                     track_streak = ?10,
                     streak_started_on = ?11,
                     done = ?12,
                     display_order = ?13,
                     updated_at = ?14
                 WHERE id = ?15",
                params![
                    category_id,
                    text,
                    Self::payload_optional_string(&record.payload, "memo"),
                    Self::payload_string(&record.payload, "repeat_type")
                        .unwrap_or_else(|| "none".to_string()),
                    Self::payload_optional_string(&record.payload, "repeat_detail"),
                    Self::payload_optional_string(&record.payload, "next_due_at"),
                    Self::payload_optional_string(&record.payload, "last_completed_at"),
                    Self::payload_optional_string(&record.payload, "reminder_at"),
                    Self::payload_optional_string(&record.payload, "archived_at"),
                    Self::payload_bool(&record.payload, "track_streak").unwrap_or(false),
                    Self::payload_optional_string(&record.payload, "streak_started_on"),
                    Self::payload_bool(&record.payload, "done").unwrap_or(false),
                    Self::payload_i64(&record.payload, "display_order").unwrap_or(1000),
                    updated_at,
                    local_id
                ],
            )?;
            return Ok(true);
        }

        conn.execute(
            "INSERT INTO v2_todos
                (category_id, text, memo, repeat_type, repeat_detail, next_due_at, last_completed_at, reminder_at, archived_at, track_streak, streak_started_on, done, display_order, created_at, updated_at)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, ?13, ?14, ?15)",
            params![
                category_id,
                text,
                Self::payload_optional_string(&record.payload, "memo"),
                Self::payload_string(&record.payload, "repeat_type").unwrap_or_else(|| "none".to_string()),
                Self::payload_optional_string(&record.payload, "repeat_detail"),
                Self::payload_optional_string(&record.payload, "next_due_at"),
                Self::payload_optional_string(&record.payload, "last_completed_at"),
                Self::payload_optional_string(&record.payload, "reminder_at"),
                Self::payload_optional_string(&record.payload, "archived_at"),
                Self::payload_bool(&record.payload, "track_streak").unwrap_or(false),
                Self::payload_optional_string(&record.payload, "streak_started_on"),
                Self::payload_bool(&record.payload, "done").unwrap_or(false),
                Self::payload_i64(&record.payload, "display_order").unwrap_or(1000),
                created_at,
                updated_at
            ],
        )?;
        let local_id = conn.last_insert_rowid();
        Self::upsert_metadata_for_remote(
            conn,
            V2ICloudSyncEntity::Todo,
            &local_id.to_string(),
            &record.sync_id,
        )?;
        Ok(true)
    }

    fn apply_todo_tag(conn: &Connection, record: &V2ICloudRecord) -> Result<bool, rusqlite::Error> {
        let Some(todo_sync_id) = Self::payload_string(&record.payload, "todo_sync_id") else {
            return Ok(false);
        };
        let Some(tag_sync_id) = Self::payload_string(&record.payload, "tag_sync_id") else {
            return Ok(false);
        };
        let Some(todo_id) = Self::local_id_for_sync(conn, &todo_sync_id)? else {
            return Ok(false);
        };
        let Some(tag_id) = Self::local_id_for_sync(conn, &tag_sync_id)? else {
            return Ok(false);
        };
        let created_at = Self::payload_string(&record.payload, "created_at")
            .unwrap_or_else(|| record.updated_at.clone());
        conn.execute(
            "INSERT OR IGNORE INTO v2_todo_tags (todo_id, tag_id, created_at)
             VALUES (?1, ?2, ?3)",
            params![todo_id, tag_id, created_at],
        )?;
        Self::upsert_metadata_for_remote(
            conn,
            V2ICloudSyncEntity::TodoTag,
            &Self::todo_tag_local_key(todo_id, tag_id),
            &record.sync_id,
        )?;
        Ok(true)
    }

    fn apply_completion_log(
        conn: &Connection,
        record: &V2ICloudRecord,
    ) -> Result<bool, rusqlite::Error> {
        let Some(item_sync_id) = Self::payload_string(&record.payload, "item_sync_id") else {
            return Ok(false);
        };
        let Some(item_id) = Self::local_id_for_sync(conn, &item_sync_id)? else {
            return Ok(false);
        };
        let completed_on = Self::payload_string(&record.payload, "completed_on")
            .unwrap_or_else(|| record.updated_at.clone());
        let remote_count = Self::payload_i64(&record.payload, "completed_count").unwrap_or(1);
        let created_at = Self::payload_string(&record.payload, "created_at")
            .unwrap_or_else(|| record.updated_at.clone());
        let updated_at = Self::payload_string(&record.payload, "updated_at")
            .unwrap_or_else(|| record.updated_at.clone());
        let existing = conn
            .query_row(
                "SELECT completed_count, updated_at
                 FROM v2_completion_logs
                 WHERE item_id = ?1 AND completed_on = ?2",
                params![item_id, completed_on],
                |row| Ok((row.get::<_, i64>(0)?, row.get::<_, String>(1)?)),
            )
            .optional()?;

        match existing {
            Some((local_count, local_updated_at)) => {
                if local_updated_at > updated_at && local_count >= remote_count {
                    return Ok(false);
                }
                conn.execute(
                    "UPDATE v2_completion_logs
                     SET completed_count = ?1,
                         updated_at = ?2
                     WHERE item_id = ?3 AND completed_on = ?4",
                    params![
                        local_count.max(remote_count),
                        local_updated_at.max(updated_at),
                        item_id,
                        completed_on
                    ],
                )?;
            }
            None => {
                conn.execute(
                    "INSERT INTO v2_completion_logs
                        (item_id, completed_on, completed_count, created_at, updated_at)
                     VALUES (?1, ?2, ?3, ?4, ?5)",
                    params![item_id, completed_on, remote_count, created_at, updated_at],
                )?;
            }
        }

        Self::upsert_metadata_for_remote(
            conn,
            V2ICloudSyncEntity::CompletionLog,
            &Self::completion_log_local_key(item_id, &completed_on),
            &record.sync_id,
        )?;
        Ok(true)
    }

    fn apply_tombstone(
        conn: &Connection,
        record: &V2ICloudRecord,
    ) -> Result<bool, rusqlite::Error> {
        let Some((entity, local_key)) = Self::local_key_for_sync_id(conn, &record.sync_id)? else {
            return Ok(false);
        };
        let deleted_at = record
            .deleted_at
            .clone()
            .unwrap_or_else(|| record.updated_at.clone());
        conn.execute(
            "UPDATE v2_sync_metadata
             SET deleted_at = ?1,
                 sync_status = 'synced'
             WHERE sync_id = ?2",
            params![deleted_at, record.sync_id],
        )?;

        match entity.as_str() {
            "todo_tag" => {
                if let Some((todo_id, tag_id)) = Self::parse_pair_key(&local_key) {
                    conn.execute(
                        "DELETE FROM v2_todo_tags WHERE todo_id = ?1 AND tag_id = ?2",
                        params![todo_id, tag_id],
                    )?;
                }
            }
            "completion_log" => {
                if let Some((item_id, completed_on)) = Self::parse_log_key(&local_key) {
                    conn.execute(
                        "DELETE FROM v2_completion_logs WHERE item_id = ?1 AND completed_on = ?2",
                        params![item_id, completed_on],
                    )?;
                }
            }
            "todo" => {
                if let Ok(id) = local_key.parse::<i64>() {
                    conn.execute("DELETE FROM v2_todos WHERE id = ?1", params![id])?;
                }
            }
            "tag" => {
                if let Ok(id) = local_key.parse::<i64>() {
                    conn.execute("DELETE FROM v2_tags WHERE id = ?1", params![id])?;
                }
            }
            "category" => {
                if let Ok(id) = local_key.parse::<i64>() {
                    conn.execute("DELETE FROM v2_categories WHERE id = ?1", params![id])?;
                }
            }
            _ => {}
        }
        Ok(true)
    }

    fn mark_tombstone(
        conn: &Connection,
        entity: V2ICloudSyncEntity,
        local_key: &str,
        deleted_at: &str,
    ) -> Result<(), rusqlite::Error> {
        Self::ensure_metadata(conn, entity.as_str(), local_key, None)?;
        conn.execute(
            "UPDATE v2_sync_metadata
             SET sync_status = 'deleted',
                 deleted_at = ?1
             WHERE entity = ?2 AND local_key = ?3",
            params![deleted_at, entity.as_str(), local_key],
        )?;
        Ok(())
    }

    fn tombstone_item_tree_with_time(
        conn: &Connection,
        item_id: i64,
        deleted_at: &str,
    ) -> Result<(), rusqlite::Error> {
        let mut tag_stmt = conn.prepare("SELECT tag_id FROM v2_todo_tags WHERE todo_id = ?1")?;
        let tag_ids = tag_stmt
            .query_map(params![item_id], |row| row.get::<_, i64>(0))?
            .collect::<Result<Vec<_>, _>>()?;
        for tag_id in tag_ids {
            let key = Self::todo_tag_local_key(item_id, tag_id);
            Self::mark_tombstone(conn, V2ICloudSyncEntity::TodoTag, &key, deleted_at)?;
        }

        let mut log_stmt =
            conn.prepare("SELECT completed_on FROM v2_completion_logs WHERE item_id = ?1")?;
        let completed_days = log_stmt
            .query_map(params![item_id], |row| row.get::<_, String>(0))?
            .collect::<Result<Vec<_>, _>>()?;
        for completed_on in completed_days {
            let key = Self::completion_log_local_key(item_id, &completed_on);
            Self::mark_tombstone(conn, V2ICloudSyncEntity::CompletionLog, &key, deleted_at)?;
        }

        Self::mark_tombstone(
            conn,
            V2ICloudSyncEntity::Todo,
            &item_id.to_string(),
            deleted_at,
        )
    }

    fn local_id_for_sync(conn: &Connection, sync_id: &str) -> Result<Option<i64>, rusqlite::Error> {
        conn.query_row(
            "SELECT local_key FROM v2_sync_metadata WHERE sync_id = ?1 AND deleted_at IS NULL",
            params![sync_id],
            |row| row.get::<_, String>(0),
        )
        .optional()
        .map(|value| value.and_then(|key| key.parse::<i64>().ok()))
    }

    fn entity_from_str(value: &str) -> V2ICloudSyncEntity {
        match value {
            "todo" => V2ICloudSyncEntity::Todo,
            "tag" => V2ICloudSyncEntity::Tag,
            "todo_tag" => V2ICloudSyncEntity::TodoTag,
            "completion_log" => V2ICloudSyncEntity::CompletionLog,
            _ => V2ICloudSyncEntity::Category,
        }
    }

    fn todo_tag_local_key(todo_id: i64, tag_id: i64) -> String {
        format!("{todo_id}:{tag_id}")
    }

    fn completion_log_local_key(item_id: i64, completed_on: &str) -> String {
        format!("{item_id}:{completed_on}")
    }

    fn parse_pair_key(value: &str) -> Option<(i64, i64)> {
        let (left, right) = value.split_once(':')?;
        Some((left.parse().ok()?, right.parse().ok()?))
    }

    fn parse_log_key(value: &str) -> Option<(i64, String)> {
        let (left, right) = value.split_once(':')?;
        Some((left.parse().ok()?, right.to_string()))
    }

    fn payload_string(payload: &Value, key: &str) -> Option<String> {
        payload.get(key)?.as_str().map(ToOwned::to_owned)
    }

    fn payload_optional_string(payload: &Value, key: &str) -> Option<String> {
        payload
            .get(key)
            .and_then(|value| {
                if value.is_null() {
                    None
                } else {
                    value.as_str()
                }
            })
            .map(ToOwned::to_owned)
    }

    fn payload_i64(payload: &Value, key: &str) -> Option<i64> {
        payload.get(key)?.as_i64()
    }

    fn payload_bool(payload: &Value, key: &str) -> Option<bool> {
        payload.get(key)?.as_bool()
    }

    fn now_iso() -> String {
        chrono::Utc::now().format("%Y-%m-%dT%H:%M:%SZ").to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::repository::V2ChecklistRepository;

    fn setup_conn() -> Connection {
        let conn = Connection::open_in_memory().expect("in-memory db");
        V2ChecklistRepository::create_tables(&conn).expect("v2 tables");
        V2ChecklistRepository::ensure_default_category(&conn).expect("default category");
        V2ICloudSyncRepository::create_tables(&conn).expect("sync tables");
        conn
    }

    #[test]
    fn creates_sync_metadata_for_existing_v2_rows() {
        let conn = setup_conn();

        V2ICloudSyncRepository::ensure_active_metadata(&conn).expect("metadata");

        let count: i64 = conn
            .query_row("SELECT COUNT(*) FROM v2_sync_metadata", [], |row| {
                row.get(0)
            })
            .expect("metadata count");
        assert!(count >= 1);
    }

    #[test]
    fn exports_v2_archived_streak_repeat_and_reminder_fields() {
        let conn = setup_conn();
        let category_id: i64 = conn
            .query_row("SELECT id FROM v2_categories LIMIT 1", [], |row| row.get(0))
            .expect("category id");
        conn.execute(
            "INSERT INTO v2_todos
                (category_id, text, memo, repeat_type, repeat_detail, next_due_at, last_completed_at, reminder_at, archived_at, track_streak, streak_started_on, done, display_order, created_at, updated_at)
             VALUES (?1, 'Run', 'memo', 'daily', NULL, '2026-06-19', '2026-06-18', '09:30', '2026-06-18T00:00:00Z', 1, '2026-06-18', 1, 1000, '2026-06-18T00:00:00Z', '2026-06-18T00:00:00Z')",
            params![category_id],
        )
        .expect("insert todo");

        let records = V2ICloudSyncRepository::export_records(&conn).expect("records");
        let todo = records
            .iter()
            .find(|record| record.entity == "todo")
            .expect("todo record");

        assert_eq!(todo.payload["repeat_type"], "daily");
        assert_eq!(todo.payload["reminder_at"], "09:30");
        assert_eq!(todo.payload["archived_at"], "2026-06-18T00:00:00Z");
        assert_eq!(todo.payload["track_streak"], true);
        assert_eq!(todo.payload["streak_started_on"], "2026-06-18");
    }

    #[test]
    fn tombstones_export_once_after_sync() {
        let conn = setup_conn();
        let category_id: i64 = conn
            .query_row("SELECT id FROM v2_categories LIMIT 1", [], |row| row.get(0))
            .expect("category id");
        conn.execute(
            "INSERT INTO v2_todos
                (category_id, text, repeat_type, track_streak, done, display_order, created_at, updated_at)
             VALUES (?1, 'Run', 'none', 0, 0, 1000, '2026-06-18T00:00:00Z', '2026-06-18T00:00:00Z')",
            params![category_id],
        )
        .expect("insert todo");
        let item_id = conn.last_insert_rowid();

        V2ICloudSyncRepository::tombstone_item_tree(&conn, item_id).expect("tombstone item");

        let pending_records =
            V2ICloudSyncRepository::export_records(&conn).expect("pending records");
        assert!(pending_records
            .iter()
            .any(|record| record.entity == "todo" && record.deleted_at.is_some()));

        V2ICloudSyncRepository::mark_all_synced(&conn, "2026-06-18T10:00:00Z")
            .expect("mark synced");

        let synced_records = V2ICloudSyncRepository::export_records(&conn).expect("synced records");
        assert!(!synced_records
            .iter()
            .any(|record| record.entity == "todo" && record.deleted_at.is_some()));
    }

    #[test]
    fn applies_completion_log_with_larger_count() {
        let conn = setup_conn();
        let category_id: i64 = conn
            .query_row("SELECT id FROM v2_categories LIMIT 1", [], |row| row.get(0))
            .expect("category id");
        conn.execute(
            "INSERT INTO v2_todos
                (category_id, text, repeat_type, track_streak, done, display_order, created_at, updated_at)
             VALUES (?1, 'Run', 'none', 0, 0, 1000, '2026-06-18T00:00:00Z', '2026-06-18T00:00:00Z')",
            params![category_id],
        )
        .expect("insert todo");
        let item_id = conn.last_insert_rowid();
        V2ICloudSyncRepository::ensure_active_metadata(&conn).expect("metadata");
        let item_sync_id =
            V2ICloudSyncRepository::sync_id_for_local(&conn, V2ICloudSyncEntity::Todo, item_id)
                .expect("item sync id");

        let record = V2ICloudRecord {
            record_type: V2ICloudSyncEntity::CompletionLog
                .cloud_record_type()
                .to_string(),
            entity: V2ICloudSyncEntity::CompletionLog.as_str().to_string(),
            sync_id: format!("completion_log:{item_sync_id}:2026-06-18"),
            updated_at: "2026-06-18T10:00:00Z".to_string(),
            deleted_at: None,
            payload: json!({
                "item_sync_id": item_sync_id,
                "completed_on": "2026-06-18",
                "completed_count": 3,
                "created_at": "2026-06-18T09:00:00Z",
                "updated_at": "2026-06-18T10:00:00Z"
            }),
        };

        V2ICloudSyncRepository::apply_remote_records(&conn, &[record]).expect("apply");

        let count: i64 = conn
            .query_row(
                "SELECT completed_count FROM v2_completion_logs WHERE item_id = ?1 AND completed_on = '2026-06-18'",
                params![item_id],
                |row| row.get(0),
            )
            .expect("completion count");
        assert_eq!(count, 3);
    }
}
