use tauri::{AppHandle, State};

use super::with_db;
use crate::models::{RepeatType, TodoItem};
use crate::service::{RepeatService, ResetService, TodoService, WidgetService};
use crate::AppState;

#[tauri::command]
pub fn add_item(
    text: String,
    category_id: Option<i64>,
    repeat_type: Option<String>,
    repeat_detail: Option<String>,
    track_streak: Option<bool>,
    reminder_at: Option<String>,
    state: State<AppState>,
) -> Result<TodoItem, String> {
    let repeat = repeat_type
        .map(|s| RepeatType::from_str(&s))
        .unwrap_or(RepeatType::None);
    with_db(&state, |db| {
        TodoService::create_item(
            db,
            &text,
            category_id,
            &repeat,
            repeat_detail.as_deref(),
            track_streak.unwrap_or(false),
            reminder_at.as_deref(),
        )
    })
}

#[tauri::command]
pub fn update_item_reminder(
    id: i64,
    reminder_at: Option<String>,
    state: State<AppState>,
) -> Result<(), String> {
    with_db(&state, |db| {
        TodoService::update_reminder(db, id, reminder_at.as_deref())
    })
}

#[tauri::command]
pub fn update_item_linked_app(
    id: i64,
    linked_app: Option<String>,
    state: State<AppState>,
) -> Result<(), String> {
    with_db(&state, |db| {
        TodoService::update_linked_app(db, id, linked_app.as_deref())
    })
}

#[tauri::command]
pub fn get_items(
    category_id: Option<i64>,
    app: AppHandle,
    state: State<AppState>,
) -> Result<Vec<TodoItem>, String> {
    with_db(&state, |db| {
        if let Err(error) = WidgetService::process_pending_actions(db, &app, None) {
            log::error!(
                "Failed to process widget actions before get_items: {}",
                error
            );
        }
        TodoService::get_items(db, category_id)
    })
}

#[tauri::command]
pub fn toggle_item(id: i64, state: State<AppState>) -> Result<Option<TodoItem>, String> {
    with_db(&state, |db| TodoService::toggle_item(db, id))
}

#[tauri::command]
pub fn delete_item(id: i64, state: State<AppState>) -> Result<(), String> {
    with_db(&state, |db| TodoService::delete_item(db, id))
}

#[tauri::command]
pub fn edit_item(id: i64, text: String, state: State<AppState>) -> Result<(), String> {
    with_db(&state, |db| TodoService::update_text(db, id, &text))
}

#[tauri::command]
pub fn update_item_memo(
    id: i64,
    memo: Option<String>,
    state: State<AppState>,
) -> Result<(), String> {
    with_db(&state, |db| {
        TodoService::update_memo(db, id, memo.as_deref())
    })
}

#[tauri::command]
pub fn update_item_repeat(
    id: i64,
    repeat_type: String,
    repeat_detail: Option<String>,
    state: State<AppState>,
) -> Result<(), String> {
    let repeat = RepeatType::from_str(&repeat_type);
    with_db(&state, |db| {
        TodoService::update_repeat(db, id, &repeat, repeat_detail.as_deref())
    })
}

#[tauri::command]
pub fn reorder_items(item_ids: Vec<i64>, state: State<AppState>) -> Result<(), String> {
    with_db(&state, |db| TodoService::reorder_items(db, &item_ids))
}

#[tauri::command]
pub fn reset_all_items(category_id: Option<i64>, state: State<AppState>) -> Result<(), String> {
    with_db(&state, |db| ResetService::reset_items(db, category_id))
}

#[tauri::command]
pub fn check_and_auto_reset(state: State<AppState>) -> Result<bool, String> {
    with_db(&state, |db| ResetService::check_and_auto_reset(db))
}

#[tauri::command]
pub fn process_repeats(state: State<AppState>) -> Result<i32, String> {
    with_db(&state, |db| RepeatService::process_repeats(db))
}
