use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum V2ICloudSyncEntity {
    Category,
    Todo,
    Tag,
    TodoTag,
    CompletionLog,
}

impl V2ICloudSyncEntity {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Category => "category",
            Self::Todo => "todo",
            Self::Tag => "tag",
            Self::TodoTag => "todo_tag",
            Self::CompletionLog => "completion_log",
        }
    }

    pub fn cloud_record_type(&self) -> &'static str {
        match self {
            Self::Category => "TicklyV2Category",
            Self::Todo => "TicklyV2Todo",
            Self::Tag => "TicklyV2Tag",
            Self::TodoTag => "TicklyV2TodoTag",
            Self::CompletionLog => "TicklyV2CompletionLog",
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct V2ICloudRecord {
    pub record_type: String,
    pub entity: String,
    pub sync_id: String,
    pub updated_at: String,
    pub deleted_at: Option<String>,
    pub payload: Value,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct V2ICloudNativeRequest {
    pub action: String,
    pub records: Vec<V2ICloudRecord>,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct V2ICloudNativeResult {
    pub available: bool,
    pub status: String,
    pub error: Option<String>,
    pub records: Vec<V2ICloudRecord>,
    pub synced_at: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct V2ICloudSyncStatus {
    pub enabled: bool,
    pub available: bool,
    pub status: String,
    pub last_synced_at: Option<String>,
    pub error: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct V2ICloudSyncResult {
    pub available: bool,
    pub status: String,
    pub pushed: i64,
    pub pulled: i64,
    pub last_synced_at: Option<String>,
    pub error: Option<String>,
}
