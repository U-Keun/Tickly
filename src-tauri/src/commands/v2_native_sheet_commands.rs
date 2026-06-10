use serde::{Deserialize, Serialize};
use tauri::AppHandle;

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct V2NativeSheetRequest {
    pub token: String,
    pub kind: String,
    pub title: String,
    pub message: Option<String>,
    pub text: Option<V2NativeSheetTextRequest>,
    pub actions: Option<Vec<V2NativeSheetActionRequest>>,
    pub cancel_label: String,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct V2NativeSheetTextRequest {
    pub label: String,
    pub placeholder: String,
    pub initial_value: String,
    pub confirm_label: String,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct V2NativeSheetActionRequest {
    pub id: String,
    pub label: String,
    pub tone: Option<String>,
    pub disabled: Option<bool>,
}

#[tauri::command]
pub fn v2_show_native_sheet(request: V2NativeSheetRequest, app: AppHandle) -> Result<bool, String> {
    #[cfg(target_os = "ios")]
    {
        let request_json = serde_json::to_string(&request).map_err(|error| error.to_string())?;
        crate::ios_native_sheet::show_native_sheet(&app, &request_json)
    }

    #[cfg(not(target_os = "ios"))]
    {
        let _ = request;
        let _ = app;
        Ok(false)
    }
}
