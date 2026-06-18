use tauri::{AppHandle, State};

use crate::models::{V2ICloudNativeResult, V2ICloudSyncResult, V2ICloudSyncStatus};
use crate::service::V2ICloudSyncService;
use crate::AppState;

#[tauri::command]
pub fn v2_get_icloud_sync_status(
    state: State<AppState>,
    app: AppHandle,
) -> Result<V2ICloudSyncStatus, String> {
    let availability =
        perform_native_request(&app, V2ICloudSyncService::build_availability_request());
    let db = state.db.lock().map_err(|error| error.to_string())?;
    V2ICloudSyncService::get_status(&db, Some(availability))
}

#[tauri::command]
pub fn v2_set_icloud_sync_enabled(
    enabled: bool,
    state: State<AppState>,
    app: AppHandle,
) -> Result<V2ICloudSyncStatus, String> {
    {
        let db = state.db.lock().map_err(|error| error.to_string())?;
        V2ICloudSyncService::set_enabled(&db, enabled)?;
    }

    v2_get_icloud_sync_status(state, app)
}

#[tauri::command]
pub fn v2_trigger_icloud_sync(
    state: State<AppState>,
    app: AppHandle,
) -> Result<V2ICloudSyncResult, String> {
    let (request, local_record_count) = {
        let db = state.db.lock().map_err(|error| error.to_string())?;
        if !V2ICloudSyncService::is_enabled(&db)? {
            return Ok(V2ICloudSyncResult {
                available: true,
                status: "disabled".to_string(),
                pushed: 0,
                pulled: 0,
                last_synced_at: None,
                error: None,
            });
        }
        let request = V2ICloudSyncService::build_sync_request(&db)?;
        let local_record_count = V2ICloudSyncService::local_record_count(&request);
        (request, local_record_count)
    };

    let native_result = perform_native_request(&app, request);
    let db = state.db.lock().map_err(|error| error.to_string())?;
    V2ICloudSyncService::apply_sync_result(&db, native_result, local_record_count)
}

fn perform_native_request(
    app: &AppHandle,
    request: crate::models::V2ICloudNativeRequest,
) -> V2ICloudNativeResult {
    let Ok(request_json) = V2ICloudSyncService::encode_native_request(&request) else {
        return V2ICloudSyncService::native_unavailable_result();
    };

    #[cfg(target_os = "ios")]
    {
        match crate::ios_icloud_sync::perform_icloud_sync(app, &request_json) {
            Ok(Some(result_json)) => V2ICloudSyncService::parse_native_result(&result_json)
                .unwrap_or_else(|error| V2ICloudNativeResult {
                    available: false,
                    status: "error".to_string(),
                    error: Some(error),
                    records: Vec::new(),
                    synced_at: None,
                }),
            Ok(None) => V2ICloudSyncService::native_unavailable_result(),
            Err(error) => V2ICloudNativeResult {
                available: false,
                status: "error".to_string(),
                error: Some(error),
                records: Vec::new(),
                synced_at: None,
            },
        }
    }

    #[cfg(not(target_os = "ios"))]
    {
        let _ = app;
        let _ = request_json;
        V2ICloudSyncService::native_unavailable_result()
    }
}
