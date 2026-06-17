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
    pub form: Option<V2NativeSheetFormRequest>,
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
pub struct V2NativeSheetFormRequest {
    pub fields: Vec<V2NativeSheetFormFieldRequest>,
    pub confirm_label: String,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct V2NativeSheetFormFieldRequest {
    pub id: String,
    pub kind: String,
    pub label: String,
    pub placeholder: String,
    pub initial_value: String,
    pub clear_label: Option<String>,
    pub initial_tags: Option<Vec<String>>,
    pub initial_repeat_detail: Option<Vec<i32>>,
    pub repeat_labels: Option<V2NativeSheetRepeatLabels>,
    pub suggestions: Option<Vec<String>>,
    pub required: Option<bool>,
    pub requires_repeat: Option<bool>,
    pub disabled_message: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct V2NativeSheetRepeatLabels {
    pub none: String,
    pub daily: String,
    pub weekly: String,
    pub monthly: String,
    pub weekly_detail: String,
    pub monthly_detail: String,
    pub weekdays: Vec<String>,
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
