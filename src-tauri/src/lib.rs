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
            checklist_get_categories,
            checklist_create_category,
            checklist_update_category,
            checklist_delete_category,
            checklist_reorder_categories,
            checklist_get_items,
            checklist_get_active_reminder_items,
            checklist_get_tags,
            checklist_get_tag_summaries,
            checklist_rename_tag,
            checklist_delete_tag,
            checklist_search_items,
            checklist_create_item,
            checklist_update_item_text,
            checklist_update_item_details,
            checklist_toggle_item,
            checklist_process_repeats,
            checklist_archive_completed_items,
            checklist_get_archived_items,
            checklist_restore_archived_item,
            checklist_delete_archived_item,
            checklist_get_streak_heatmaps,
            checklist_get_graph_data,
            checklist_delete_item,
            checklist_reorder_items,
            configure_native_dock,
            show_native_sheet
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
