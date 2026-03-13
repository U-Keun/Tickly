use tauri::State;

use super::with_db;
use crate::models::{Tag, TodoItem};
use crate::service::TagService;
use crate::AppState;

#[tauri::command]
pub fn get_all_tags(state: State<AppState>) -> Result<Vec<Tag>, String> {
    with_db(&state, |db| TagService::get_all_tags(db))
}

#[tauri::command]
pub fn create_tag(name: String, state: State<AppState>) -> Result<Tag, String> {
    with_db(&state, |db| TagService::create_tag(db, &name))
}

#[tauri::command]
pub fn delete_tag(id: i64, state: State<AppState>) -> Result<(), String> {
    with_db(&state, |db| TagService::delete_tag(db, id))
}

#[tauri::command]
pub fn add_tag_to_item(
    item_id: i64,
    tag_name: String,
    state: State<AppState>,
) -> Result<Tag, String> {
    with_db(&state, |db| {
        TagService::add_tag_to_item(db, item_id, &tag_name)
    })
}

#[tauri::command]
pub fn remove_tag_from_item(
    item_id: i64,
    tag_id: i64,
    state: State<AppState>,
) -> Result<(), String> {
    with_db(&state, |db| {
        TagService::remove_tag_from_item(db, item_id, tag_id)
    })
}

#[tauri::command]
pub fn get_tags_for_item(item_id: i64, state: State<AppState>) -> Result<Vec<Tag>, String> {
    with_db(&state, |db| TagService::get_tags_for_item(db, item_id))
}

#[tauri::command]
pub fn get_items_by_tag(tag_id: i64, state: State<AppState>) -> Result<Vec<TodoItem>, String> {
    with_db(&state, |db| TagService::get_items_by_tag(db, tag_id))
}
