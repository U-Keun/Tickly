use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct ChecklistSyncRecord {
    pub entity_type: String,
    pub sync_id: String,
    pub updated_at: String,
    pub deleted_at: Option<String>,
    pub payload: Value,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct ChecklistSyncStatus {
    pub enabled: bool,
    pub last_synced_at: Option<String>,
    pub last_error: Option<String>,
}
