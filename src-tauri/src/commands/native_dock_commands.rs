use serde::{Deserialize, Serialize};
use tauri::AppHandle;

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct NativeDockRequest {
    pub visible: bool,
    pub streak_label: String,
    pub graph_label: String,
    pub archive_label: String,
    pub settings_label: String,
    pub streak_enabled: bool,
    pub graph_enabled: bool,
    pub archive_enabled: bool,
    pub settings_enabled: bool,
}

#[tauri::command]
pub fn v2_configure_native_dock(
    request: NativeDockRequest,
    app: AppHandle,
) -> Result<bool, String> {
    #[cfg(target_os = "ios")]
    {
        let request_json = serde_json::to_string(&request).map_err(|error| error.to_string())?;
        crate::ios_native_dock::configure_native_dock(&app, &request_json)
    }

    #[cfg(not(target_os = "ios"))]
    {
        let _ = request;
        let _ = app;
        Ok(false)
    }
}
