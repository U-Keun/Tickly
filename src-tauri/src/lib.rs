mod commands;
#[cfg(target_os = "ios")]
mod ios_fullscreen;
#[cfg(target_os = "ios")]
mod ios_native_dock;
#[cfg(target_os = "ios")]
mod ios_native_sheet;
mod models;
mod repository;
mod service;

use commands::*;
use repository::init_database;
use rusqlite::Connection;
use service::WidgetService;
use std::sync::Mutex;
use tauri::Manager;

pub struct AppState {
    pub db: Mutex<Connection>,
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    #[allow(unused_mut)]
    let mut builder = tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_os::init())
        .plugin(tauri_plugin_deep_link::init())
        .plugin(tauri_plugin_notification::init());

    builder
        .setup(|app| {
            let conn = init_database(app.handle())?;
            if let Err(error) = WidgetService::process_pending_actions(&conn, app.handle(), None) {
                log::error!("Failed to process widget actions on app startup: {}", error);
            }

            app.manage(AppState {
                db: Mutex::new(conn),
            });

            #[cfg(target_os = "ios")]
            ios_fullscreen::configure_ios_fullscreen_viewport(app);

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            // Widget commands
            get_widget_snapshot,
            refresh_widget_cache,
            toggle_item_from_widget,
            process_widget_actions,
            set_widget_cache_path,
            get_widget_cache_path,
            set_widget_app_group_id,
            get_widget_app_group_id,
            // Settings commands
            get_setting,
            set_setting,
            // Checklist commands
            v2_get_categories,
            v2_create_category,
            v2_update_category,
            v2_delete_category,
            v2_reorder_categories,
            v2_get_items,
            v2_get_active_reminder_items,
            v2_get_tags,
            v2_get_tag_summaries,
            v2_rename_tag,
            v2_delete_tag,
            v2_search_items,
            v2_create_item,
            v2_update_item_text,
            v2_update_item_details,
            v2_toggle_item,
            v2_process_repeats,
            v2_archive_completed_items,
            v2_get_archived_items,
            v2_restore_archived_item,
            v2_delete_archived_item,
            v2_get_streak_heatmaps,
            v2_get_graph_data,
            v2_delete_item,
            v2_reorder_items,
            v2_configure_native_dock,
            v2_show_native_sheet
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
