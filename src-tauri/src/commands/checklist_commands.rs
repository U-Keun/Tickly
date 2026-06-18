use tauri::State;

use super::with_db;
use crate::models::{
    ChecklistArchivedItem, ChecklistCategory, ChecklistGraphData, ChecklistItemSearchResult,
    ChecklistRepeatType, ChecklistStreakHeatmap, ChecklistTag, ChecklistTagSummary,
    ChecklistTodoItem,
};
use crate::service::ChecklistService;
use crate::AppState;

#[tauri::command]
pub fn v2_get_categories(state: State<AppState>) -> Result<Vec<ChecklistCategory>, String> {
    with_db(&state, ChecklistService::get_categories)
}

#[tauri::command]
pub fn v2_create_category(
    name: String,
    state: State<AppState>,
) -> Result<ChecklistCategory, String> {
    with_db(&state, |db| ChecklistService::create_category(db, &name))
}

#[tauri::command]
pub fn v2_update_category(id: i64, name: String, state: State<AppState>) -> Result<(), String> {
    with_db(&state, |db| {
        ChecklistService::update_category(db, id, &name)
    })
}

#[tauri::command]
pub fn v2_delete_category(id: i64, state: State<AppState>) -> Result<(), String> {
    with_db(&state, |db| ChecklistService::delete_category(db, id))
}

#[tauri::command]
pub fn v2_reorder_categories(category_ids: Vec<i64>, state: State<AppState>) -> Result<(), String> {
    with_db(&state, |db| {
        ChecklistService::reorder_categories(db, &category_ids)
    })
}

#[tauri::command]
pub fn v2_get_items(
    category_id: i64,
    state: State<AppState>,
) -> Result<Vec<ChecklistTodoItem>, String> {
    with_db(&state, |db| ChecklistService::get_items(db, category_id))
}

#[tauri::command]
pub fn v2_get_active_reminder_items(
    state: State<AppState>,
) -> Result<Vec<ChecklistTodoItem>, String> {
    with_db(&state, ChecklistService::get_active_reminder_items)
}

#[tauri::command]
pub fn v2_get_tags(state: State<AppState>) -> Result<Vec<ChecklistTag>, String> {
    with_db(&state, ChecklistService::get_tags)
}

#[tauri::command]
pub fn v2_get_tag_summaries(state: State<AppState>) -> Result<Vec<ChecklistTagSummary>, String> {
    with_db(&state, ChecklistService::get_tag_summaries)
}

#[tauri::command]
pub fn v2_rename_tag(
    id: i64,
    name: String,
    state: State<AppState>,
) -> Result<ChecklistTag, String> {
    with_db(&state, |db| ChecklistService::rename_tag(db, id, &name))
}

#[tauri::command]
pub fn v2_delete_tag(id: i64, state: State<AppState>) -> Result<(), String> {
    with_db(&state, |db| ChecklistService::delete_tag(db, id))
}

#[tauri::command]
pub fn v2_search_items(
    query: String,
    limit: i64,
    state: State<AppState>,
) -> Result<Vec<ChecklistItemSearchResult>, String> {
    with_db(&state, |db| {
        ChecklistService::search_items(db, &query, limit)
    })
}

#[tauri::command]
pub fn v2_create_item(
    category_id: i64,
    text: String,
    tag_names: Option<Vec<String>>,
    state: State<AppState>,
) -> Result<ChecklistTodoItem, String> {
    with_db(&state, |db| {
        ChecklistService::create_item_with_tags(
            db,
            category_id,
            &text,
            tag_names.as_deref().unwrap_or(&[]),
        )
    })
}

#[tauri::command]
pub fn v2_update_item_text(id: i64, text: String, state: State<AppState>) -> Result<(), String> {
    with_db(&state, |db| {
        ChecklistService::update_item_text(db, id, &text)
    })
}

#[tauri::command]
pub fn v2_update_item_details(
    id: i64,
    text: String,
    memo: Option<String>,
    tag_names: Option<Vec<String>>,
    repeat_type: Option<String>,
    repeat_detail: Option<String>,
    reminder_at: Option<String>,
    track_streak: Option<bool>,
    state: State<AppState>,
) -> Result<ChecklistTodoItem, String> {
    let repeat = repeat_type
        .as_deref()
        .map(ChecklistRepeatType::from_str)
        .unwrap_or(ChecklistRepeatType::None);

    with_db(&state, |db| {
        ChecklistService::update_item_details(
            db,
            id,
            &text,
            memo.as_deref(),
            tag_names.as_deref().unwrap_or(&[]),
            &repeat,
            repeat_detail.as_deref(),
            reminder_at.as_deref(),
            track_streak,
        )
    })
}

#[tauri::command]
pub fn v2_toggle_item(id: i64, state: State<AppState>) -> Result<ChecklistTodoItem, String> {
    with_db(&state, |db| ChecklistService::toggle_item(db, id))
}

#[tauri::command]
pub fn v2_process_repeats(state: State<AppState>) -> Result<i64, String> {
    with_db(&state, ChecklistService::process_repeats)
}

#[tauri::command]
pub fn v2_archive_completed_items(category_id: i64, state: State<AppState>) -> Result<i64, String> {
    with_db(&state, |db| {
        ChecklistService::archive_completed_items(db, category_id)
    })
}

#[tauri::command]
pub fn v2_get_archived_items(state: State<AppState>) -> Result<Vec<ChecklistArchivedItem>, String> {
    with_db(&state, ChecklistService::get_archived_items)
}

#[tauri::command]
pub fn v2_restore_archived_item(
    id: i64,
    state: State<AppState>,
) -> Result<ChecklistTodoItem, String> {
    with_db(&state, |db| ChecklistService::restore_archived_item(db, id))
}

#[tauri::command]
pub fn v2_delete_archived_item(id: i64, state: State<AppState>) -> Result<(), String> {
    with_db(&state, |db| ChecklistService::delete_archived_item(db, id))
}

#[tauri::command]
pub fn v2_get_streak_heatmaps(
    state: State<AppState>,
) -> Result<Vec<ChecklistStreakHeatmap>, String> {
    with_db(&state, ChecklistService::get_streak_heatmaps)
}

#[tauri::command]
pub fn v2_get_graph_data(state: State<AppState>) -> Result<ChecklistGraphData, String> {
    with_db(&state, ChecklistService::get_graph_data)
}

#[tauri::command]
pub fn v2_delete_item(id: i64, state: State<AppState>) -> Result<(), String> {
    with_db(&state, |db| ChecklistService::delete_item(db, id))
}

#[tauri::command]
pub fn v2_reorder_items(
    category_id: i64,
    item_ids: Vec<i64>,
    state: State<AppState>,
) -> Result<(), String> {
    with_db(&state, |db| {
        ChecklistService::reorder_items(db, category_id, &item_ids)
    })
}
