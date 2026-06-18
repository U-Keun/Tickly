use rusqlite::Connection;

use crate::models::{
    V2ICloudNativeRequest, V2ICloudNativeResult, V2ICloudSyncResult, V2ICloudSyncStatus,
};
use crate::repository::{SettingsRepository, V2ICloudSyncRepository};

pub struct V2ICloudSyncService;

impl V2ICloudSyncService {
    const ENABLED_KEY: &'static str = "v2_icloud_sync_enabled";
    const LAST_SYNCED_AT_KEY: &'static str = "v2_icloud_last_synced_at";
    const LAST_STATUS_KEY: &'static str = "v2_icloud_last_status";
    const LAST_ERROR_KEY: &'static str = "v2_icloud_last_error";

    pub fn get_status(
        conn: &Connection,
        availability: Option<V2ICloudNativeResult>,
    ) -> Result<V2ICloudSyncStatus, String> {
        let enabled = Self::is_enabled(conn)?;
        let last_synced_at = SettingsRepository::get(conn, Self::LAST_SYNCED_AT_KEY)
            .map_err(|error| error.to_string())?
            .or_else(|| V2ICloudSyncRepository::last_synced_at(conn).ok().flatten());
        let stored_status = SettingsRepository::get(conn, Self::LAST_STATUS_KEY)
            .map_err(|error| error.to_string())?
            .unwrap_or_else(|| "idle".to_string());
        let stored_error = SettingsRepository::get(conn, Self::LAST_ERROR_KEY)
            .map_err(|error| error.to_string())?;

        Ok(match availability {
            Some(native) => V2ICloudSyncStatus {
                enabled,
                available: native.available,
                status: if native.available {
                    stored_status
                } else {
                    native.status
                },
                last_synced_at,
                error: native.error.or(stored_error),
            },
            None => V2ICloudSyncStatus {
                enabled,
                available: false,
                status: "unavailable".to_string(),
                last_synced_at,
                error: stored_error,
            },
        })
    }

    pub fn set_enabled(conn: &Connection, enabled: bool) -> Result<(), String> {
        SettingsRepository::set(
            conn,
            Self::ENABLED_KEY,
            if enabled { "true" } else { "false" },
        )
        .map_err(|error| error.to_string())
    }

    pub fn is_enabled(conn: &Connection) -> Result<bool, String> {
        Ok(SettingsRepository::get(conn, Self::ENABLED_KEY)
            .map_err(|error| error.to_string())?
            .as_deref()
            == Some("true"))
    }

    pub fn build_availability_request() -> V2ICloudNativeRequest {
        V2ICloudNativeRequest {
            action: "availability".to_string(),
            records: Vec::new(),
        }
    }

    pub fn build_sync_request(conn: &Connection) -> Result<V2ICloudNativeRequest, String> {
        let records =
            V2ICloudSyncRepository::export_records(conn).map_err(|error| error.to_string())?;
        Ok(V2ICloudNativeRequest {
            action: "sync".to_string(),
            records,
        })
    }

    pub fn apply_sync_result(
        conn: &Connection,
        native_result: V2ICloudNativeResult,
        local_record_count: usize,
    ) -> Result<V2ICloudSyncResult, String> {
        if !native_result.available {
            Self::save_status(
                conn,
                &native_result.status,
                native_result.error.as_deref(),
                None,
            )?;
            return Ok(V2ICloudSyncResult {
                available: false,
                status: native_result.status,
                pushed: 0,
                pulled: 0,
                last_synced_at: None,
                error: native_result.error,
            });
        }

        let pulled = V2ICloudSyncRepository::apply_remote_records(conn, &native_result.records)
            .map_err(|error| error.to_string())?;
        let synced_at = native_result
            .synced_at
            .clone()
            .unwrap_or_else(Self::now_iso);
        V2ICloudSyncRepository::mark_all_synced(conn, &synced_at)
            .map_err(|error| error.to_string())?;
        Self::save_status(conn, "synced", None, Some(&synced_at))?;

        Ok(V2ICloudSyncResult {
            available: true,
            status: "synced".to_string(),
            pushed: local_record_count as i64,
            pulled,
            last_synced_at: Some(synced_at),
            error: None,
        })
    }

    pub fn native_unavailable_result() -> V2ICloudNativeResult {
        V2ICloudNativeResult {
            available: false,
            status: "unavailable".to_string(),
            error: Some("iCloud sync is only available in the iOS app.".to_string()),
            records: Vec::new(),
            synced_at: None,
        }
    }

    #[cfg_attr(not(target_os = "ios"), allow(dead_code))]
    pub fn parse_native_result(value: &str) -> Result<V2ICloudNativeResult, String> {
        serde_json::from_str(value).map_err(|error| error.to_string())
    }

    pub fn encode_native_request(request: &V2ICloudNativeRequest) -> Result<String, String> {
        serde_json::to_string(request).map_err(|error| error.to_string())
    }

    pub fn local_record_count(request: &V2ICloudNativeRequest) -> usize {
        request.records.len()
    }

    fn save_status(
        conn: &Connection,
        status: &str,
        error: Option<&str>,
        synced_at: Option<&str>,
    ) -> Result<(), String> {
        SettingsRepository::set(conn, Self::LAST_STATUS_KEY, status)
            .map_err(|error| error.to_string())?;
        if let Some(value) = error {
            SettingsRepository::set(conn, Self::LAST_ERROR_KEY, value)
                .map_err(|error| error.to_string())?;
        } else {
            SettingsRepository::delete(conn, Self::LAST_ERROR_KEY)
                .map_err(|error| error.to_string())?;
        }
        if let Some(value) = synced_at {
            SettingsRepository::set(conn, Self::LAST_SYNCED_AT_KEY, value)
                .map_err(|error| error.to_string())?;
        }
        Ok(())
    }

    fn now_iso() -> String {
        chrono::Utc::now().format("%Y-%m-%dT%H:%M:%SZ").to_string()
    }
}
