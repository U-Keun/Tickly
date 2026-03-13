use tauri::State;

use super::with_db;
use crate::models::{HeatmapData, TrackedItem};
use crate::service::StreakService;
use crate::AppState;

#[tauri::command]
pub fn get_tracked_items(state: State<AppState>) -> Result<Vec<TrackedItem>, String> {
    with_db(&state, |db| StreakService::get_tracked_items(db))
}

#[tauri::command]
pub fn get_item_heatmap_data(
    item_id: i64,
    state: State<AppState>,
) -> Result<Option<HeatmapData>, String> {
    with_db(&state, |db| {
        StreakService::get_item_heatmap_data(db, item_id)
    })
}

#[tauri::command]
pub fn update_track_streak(
    id: i64,
    track_streak: bool,
    state: State<AppState>,
) -> Result<(), String> {
    with_db(&state, |db| {
        StreakService::update_track_streak(db, id, track_streak)
    })
}
