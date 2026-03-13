use tauri::{AppHandle, State};

use super::with_db;
use crate::models::WidgetSnapshot;
use crate::service::WidgetService;
use crate::AppState;

#[tauri::command]
pub fn get_widget_snapshot(
    max_items: Option<i64>,
    state: State<AppState>,
) -> Result<WidgetSnapshot, String> {
    with_db(&state, |db| {
        WidgetService::get_snapshot(db, normalize_limit(max_items))
    })
}

#[tauri::command]
pub fn refresh_widget_cache(
    max_items: Option<i64>,
    app: AppHandle,
    state: State<AppState>,
) -> Result<WidgetSnapshot, String> {
    with_db(&state, |db| {
        WidgetService::refresh_cache(db, &app, normalize_limit(max_items))
    })
}

#[tauri::command]
pub fn toggle_item_from_widget(
    id: i64,
    max_items: Option<i64>,
    app: AppHandle,
    state: State<AppState>,
) -> Result<WidgetSnapshot, String> {
    with_db(&state, |db| {
        WidgetService::toggle_item_and_refresh(db, &app, id, normalize_limit(max_items))
    })
}

#[tauri::command]
pub fn process_widget_actions(
    max_items: Option<i64>,
    app: AppHandle,
    state: State<AppState>,
) -> Result<i64, String> {
    with_db(&state, |db| {
        WidgetService::process_pending_actions(db, &app, normalize_limit(max_items))
            .map(|processed| processed as i64)
    })
}

#[tauri::command]
pub fn set_widget_cache_path(path: String, state: State<AppState>) -> Result<(), String> {
    with_db(&state, |db| WidgetService::set_cache_path(db, &path))
}

#[tauri::command]
pub fn set_widget_app_group_id(app_group_id: String, state: State<AppState>) -> Result<(), String> {
    with_db(&state, |db| {
        WidgetService::set_app_group_id(db, &app_group_id)
    })
}

#[tauri::command]
pub fn get_widget_cache_path(app: AppHandle, state: State<AppState>) -> Result<String, String> {
    with_db(&state, |db| WidgetService::get_cache_path(db, &app))
}

#[tauri::command]
pub fn get_widget_app_group_id(state: State<AppState>) -> Result<String, String> {
    with_db(&state, |db| WidgetService::get_app_group_id(db))
}

fn normalize_limit(max_items: Option<i64>) -> Option<usize> {
    max_items.and_then(|value| {
        if value > 0 {
            Some(value as usize)
        } else {
            None
        }
    })
}
