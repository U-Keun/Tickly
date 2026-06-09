use tauri::State;

use super::with_db;
use crate::models::{V2Category, V2ItemSearchResult, V2TodoItem};
use crate::service::V2ChecklistService;
use crate::AppState;

#[tauri::command]
pub fn v2_get_categories(state: State<AppState>) -> Result<Vec<V2Category>, String> {
    with_db(&state, V2ChecklistService::get_categories)
}

#[tauri::command]
pub fn v2_create_category(name: String, state: State<AppState>) -> Result<V2Category, String> {
    with_db(&state, |db| V2ChecklistService::create_category(db, &name))
}

#[tauri::command]
pub fn v2_update_category(id: i64, name: String, state: State<AppState>) -> Result<(), String> {
    with_db(&state, |db| {
        V2ChecklistService::update_category(db, id, &name)
    })
}

#[tauri::command]
pub fn v2_delete_category(id: i64, state: State<AppState>) -> Result<(), String> {
    with_db(&state, |db| V2ChecklistService::delete_category(db, id))
}

#[tauri::command]
pub fn v2_reorder_categories(category_ids: Vec<i64>, state: State<AppState>) -> Result<(), String> {
    with_db(&state, |db| {
        V2ChecklistService::reorder_categories(db, &category_ids)
    })
}

#[tauri::command]
pub fn v2_get_items(category_id: i64, state: State<AppState>) -> Result<Vec<V2TodoItem>, String> {
    with_db(&state, |db| V2ChecklistService::get_items(db, category_id))
}

#[tauri::command]
pub fn v2_search_items(
    query: String,
    limit: i64,
    state: State<AppState>,
) -> Result<Vec<V2ItemSearchResult>, String> {
    with_db(&state, |db| {
        V2ChecklistService::search_items(db, &query, limit)
    })
}

#[tauri::command]
pub fn v2_create_item(
    category_id: i64,
    text: String,
    state: State<AppState>,
) -> Result<V2TodoItem, String> {
    with_db(&state, |db| {
        V2ChecklistService::create_item(db, category_id, &text)
    })
}

#[tauri::command]
pub fn v2_update_item_text(id: i64, text: String, state: State<AppState>) -> Result<(), String> {
    with_db(&state, |db| {
        V2ChecklistService::update_item_text(db, id, &text)
    })
}

#[tauri::command]
pub fn v2_toggle_item(id: i64, state: State<AppState>) -> Result<V2TodoItem, String> {
    with_db(&state, |db| V2ChecklistService::toggle_item(db, id))
}

#[tauri::command]
pub fn v2_delete_item(id: i64, state: State<AppState>) -> Result<(), String> {
    with_db(&state, |db| V2ChecklistService::delete_item(db, id))
}

#[tauri::command]
pub fn v2_reorder_items(
    category_id: i64,
    item_ids: Vec<i64>,
    state: State<AppState>,
) -> Result<(), String> {
    with_db(&state, |db| {
        V2ChecklistService::reorder_items(db, category_id, &item_ids)
    })
}
