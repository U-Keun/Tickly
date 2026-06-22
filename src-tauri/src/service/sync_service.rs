use rusqlite::Connection;

use crate::models::{ChecklistSyncRecord, ChecklistSyncStatus};
use crate::repository::SyncRepository;

pub struct SyncService;

impl SyncService {
    pub fn get_status(conn: &Connection) -> Result<ChecklistSyncStatus, String> {
        let enabled = SyncRepository::get_state(conn, "enabled")
            .map_err(|error| error.to_string())?
            .as_deref()
            == Some("true");
        let last_synced_at =
            SyncRepository::get_state(conn, "last_synced_at").map_err(|error| error.to_string())?;
        let last_error = SyncRepository::get_state(conn, "last_error")
            .map_err(|error| error.to_string())?
            .filter(|value| !value.trim().is_empty());

        Ok(ChecklistSyncStatus {
            enabled,
            last_synced_at,
            last_error,
        })
    }

    pub fn set_enabled(conn: &Connection, enabled: bool) -> Result<ChecklistSyncStatus, String> {
        if enabled {
            SyncRepository::ensure_baseline(conn).map_err(|error| error.to_string())?;
        }
        SyncRepository::set_state(conn, "enabled", if enabled { "true" } else { "false" })
            .map_err(|error| error.to_string())?;
        Self::get_status(conn)
    }

    pub fn export_records(conn: &Connection) -> Result<Vec<ChecklistSyncRecord>, String> {
        SyncRepository::ensure_baseline(conn).map_err(|error| error.to_string())?;
        SyncRepository::export_records(conn).map_err(|error| error.to_string())
    }

    pub fn apply_remote_records(
        conn: &Connection,
        records: &[ChecklistSyncRecord],
    ) -> Result<i64, String> {
        SyncRepository::apply_remote_records(conn, records).map_err(|error| error.to_string())
    }

    pub fn mark_records_synced(conn: &Connection, sync_ids: &[String]) -> Result<(), String> {
        SyncRepository::mark_records_synced(conn, sync_ids).map_err(|error| error.to_string())
    }

    pub fn set_last_error(conn: &Connection, error: Option<&str>) -> Result<(), String> {
        SyncRepository::set_state(conn, "last_error", error.unwrap_or(""))
            .map_err(|source| source.to_string())
    }
}
