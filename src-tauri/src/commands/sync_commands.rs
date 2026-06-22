use serde::{Deserialize, Serialize};
use tauri::{AppHandle, State};

use super::with_db;
use crate::models::{ChecklistSyncRecord, ChecklistSyncStatus};
use crate::service::SyncService;
use crate::AppState;

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ICloudSyncExchangeRequest {
    pub token: String,
    pub records: Vec<ChecklistSyncRecord>,
}

#[tauri::command]
pub fn checklist_sync_get_status(state: State<AppState>) -> Result<ChecklistSyncStatus, String> {
    with_db(&state, SyncService::get_status)
}

#[tauri::command]
pub fn checklist_sync_set_enabled(
    enabled: bool,
    state: State<AppState>,
) -> Result<ChecklistSyncStatus, String> {
    with_db(&state, |db| SyncService::set_enabled(db, enabled))
}

#[tauri::command]
pub fn checklist_sync_export_records(
    state: State<AppState>,
) -> Result<Vec<ChecklistSyncRecord>, String> {
    with_db(&state, SyncService::export_records)
}

#[tauri::command]
pub fn checklist_sync_apply_remote_records(
    records: Vec<ChecklistSyncRecord>,
    state: State<AppState>,
) -> Result<i64, String> {
    with_db(&state, |db| SyncService::apply_remote_records(db, &records))
}

#[tauri::command]
pub fn checklist_sync_mark_records_synced(
    sync_ids: Vec<String>,
    state: State<AppState>,
) -> Result<(), String> {
    with_db(&state, |db| SyncService::mark_records_synced(db, &sync_ids))
}

#[tauri::command]
pub fn checklist_sync_set_last_error(
    error: Option<String>,
    state: State<AppState>,
) -> Result<(), String> {
    with_db(&state, |db| {
        SyncService::set_last_error(db, error.as_deref())
    })
}

#[tauri::command]
pub fn checklist_icloud_exchange(
    request: ICloudSyncExchangeRequest,
    app: AppHandle,
) -> Result<bool, String> {
    #[cfg(target_os = "ios")]
    {
        let request_json = serde_json::to_string(&request).map_err(|error| error.to_string())?;
        crate::ios_icloud_sync::exchange_icloud_sync(&app, &request_json)
    }

    #[cfg(not(target_os = "ios"))]
    {
        let _ = request;
        let _ = app;
        Ok(false)
    }
}
