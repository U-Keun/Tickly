use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub enum ChecklistRepeatType {
    #[serde(rename = "none")]
    None,
    #[serde(rename = "daily")]
    Daily,
    #[serde(rename = "weekly")]
    Weekly,
    #[serde(rename = "monthly")]
    Monthly,
}

impl Default for ChecklistRepeatType {
    fn default() -> Self {
        ChecklistRepeatType::None
    }
}

impl ChecklistRepeatType {
    pub fn from_str(value: &str) -> Self {
        match value {
            "daily" => ChecklistRepeatType::Daily,
            "weekly" => ChecklistRepeatType::Weekly,
            "monthly" => ChecklistRepeatType::Monthly,
            _ => ChecklistRepeatType::None,
        }
    }

    pub fn to_str(&self) -> &'static str {
        match self {
            ChecklistRepeatType::None => "none",
            ChecklistRepeatType::Daily => "daily",
            ChecklistRepeatType::Weekly => "weekly",
            ChecklistRepeatType::Monthly => "monthly",
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct ChecklistCategory {
    pub id: i64,
    pub name: String,
    pub display_order: i64,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct ChecklistTodoItem {
    pub id: i64,
    pub category_id: i64,
    pub text: String,
    pub memo: Option<String>,
    pub tags: Vec<ChecklistTag>,
    pub repeat_type: ChecklistRepeatType,
    pub repeat_detail: Option<String>,
    pub next_due_at: Option<String>,
    pub last_completed_at: Option<String>,
    pub reminder_at: Option<String>,
    pub archived_at: Option<String>,
    pub track_streak: bool,
    pub streak_started_on: Option<String>,
    pub done: bool,
    pub display_order: i64,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct ChecklistTag {
    pub id: i64,
    pub name: String,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct ChecklistTagSummary {
    pub tag: ChecklistTag,
    pub item_count: i64,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct ChecklistItemSearchResult {
    pub item: ChecklistTodoItem,
    pub category: ChecklistCategory,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct ChecklistArchivedItem {
    pub item: ChecklistTodoItem,
    pub category: ChecklistCategory,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct ChecklistGraphTagEdge {
    pub tag_id: i64,
    pub item_id: i64,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct ChecklistGraphData {
    pub categories: Vec<ChecklistCategory>,
    pub items: Vec<ChecklistTodoItem>,
    pub tags: Vec<ChecklistTag>,
    pub tag_edges: Vec<ChecklistGraphTagEdge>,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct ChecklistStreakLog {
    pub completed_on: String,
    pub completed_count: i64,
    pub combo_intensity: i64,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct ChecklistStreakHeatmap {
    pub item: ChecklistTodoItem,
    pub category: ChecklistCategory,
    pub logs: Vec<ChecklistStreakLog>,
    pub combo_intensity: i64,
    pub total_days: i64,
    pub current_streak: i64,
    pub longest_streak: i64,
    pub current_streak_dates: Vec<String>,
    pub longest_streak_dates: Vec<String>,
}
