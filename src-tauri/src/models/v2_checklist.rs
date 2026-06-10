use serde::{Deserialize, Serialize};

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
