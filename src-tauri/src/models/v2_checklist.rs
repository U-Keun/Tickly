use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub enum V2RepeatType {
    #[serde(rename = "none")]
    None,
    #[serde(rename = "daily")]
    Daily,
    #[serde(rename = "weekly")]
    Weekly,
    #[serde(rename = "monthly")]
    Monthly,
}

impl Default for V2RepeatType {
    fn default() -> Self {
        V2RepeatType::None
    }
}

impl V2RepeatType {
    pub fn from_str(value: &str) -> Self {
        match value {
            "daily" => V2RepeatType::Daily,
            "weekly" => V2RepeatType::Weekly,
            "monthly" => V2RepeatType::Monthly,
            _ => V2RepeatType::None,
        }
    }

    pub fn to_str(&self) -> &'static str {
        match self {
            V2RepeatType::None => "none",
            V2RepeatType::Daily => "daily",
            V2RepeatType::Weekly => "weekly",
            V2RepeatType::Monthly => "monthly",
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct V2Category {
    pub id: i64,
    pub name: String,
    pub display_order: i64,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct V2TodoItem {
    pub id: i64,
    pub category_id: i64,
    pub text: String,
    pub memo: Option<String>,
    pub tags: Vec<V2Tag>,
    pub repeat_type: V2RepeatType,
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
pub struct V2Tag {
    pub id: i64,
    pub name: String,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct V2ItemSearchResult {
    pub item: V2TodoItem,
    pub category: V2Category,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct V2ArchivedItem {
    pub item: V2TodoItem,
    pub category: V2Category,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct V2StreakLog {
    pub completed_on: String,
    pub completed_count: i64,
    pub combo_intensity: i64,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct V2StreakHeatmap {
    pub item: V2TodoItem,
    pub category: V2Category,
    pub logs: Vec<V2StreakLog>,
    pub combo_intensity: i64,
    pub total_days: i64,
    pub current_streak: i64,
    pub longest_streak: i64,
    pub current_streak_dates: Vec<String>,
    pub longest_streak_dates: Vec<String>,
}
