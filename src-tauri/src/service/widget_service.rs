use std::collections::{HashMap, HashSet};
use std::fs;
use std::path::PathBuf;

use rusqlite::Connection;
use serde::Deserialize;
use tauri::{AppHandle, Manager};

use crate::models::{
    WidgetCategoryPendingItem, WidgetCategorySummary, WidgetSnapshot, WidgetTheme, WidgetTodoItem,
};
use crate::repository::{ChecklistRepository, SettingsRepository};
use crate::service::ChecklistService;

pub struct WidgetService;

const WIDGET_CACHE_PATH_KEY: &str = "widget_cache_path";
const WIDGET_APP_GROUP_ID_KEY: &str = "widget_app_group_id";
const THEME_SETTING_KEY: &str = "theme";
const DEFAULT_WIDGET_APP_GROUP_ID: &str = "group.com.u-keunsong.tickly";
const DEFAULT_WIDGET_CACHE_FILE: &str = "widget-cache.json";
const DEFAULT_WIDGET_ACTION_FILE: &str = "widget-actions.json";
const DEFAULT_WIDGET_ITEM_LIMIT: usize = 20;
const MAX_WIDGET_ITEM_LIMIT: usize = 100;

mod cache;
mod snapshot;
mod theme;

#[derive(Debug, Deserialize)]
struct WidgetToggleAction {
    #[serde(alias = "itemId")]
    item_id: i64,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct SavedThemeSetting {
    preset_id: Option<String>,
    custom_colors: Option<SavedThemeColors>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct SavedThemeColors {
    paper: Option<String>,
    canvas: Option<String>,
    stroke: Option<String>,
    ink: Option<String>,
    ink_muted: Option<String>,
    accent_sky: Option<String>,
    accent_sky_strong: Option<String>,
}

impl WidgetService {
    pub fn refresh_cache(
        conn: &Connection,
        app: &AppHandle,
        max_items: Option<usize>,
    ) -> Result<WidgetSnapshot, String> {
        let _ = ChecklistService::process_repeats(conn).map_err(|error| {
            log::error!("Failed to process repeats before widget refresh: {}", error);
            error
        });
        let snapshot = Self::get_snapshot(conn, max_items).map_err(|e| e.to_string())?;
        let cache_path = Self::resolve_cache_path(conn, app)?;

        if let Some(parent) = cache_path.parent() {
            fs::create_dir_all(parent).map_err(|e| e.to_string())?;
        }

        let snapshot_json = serde_json::to_string(&snapshot).map_err(|e| e.to_string())?;
        fs::write(&cache_path, snapshot_json).map_err(|e| e.to_string())?;
        Self::request_widget_reload();

        Ok(snapshot)
    }

    pub fn set_cache_path(conn: &Connection, path: &str) -> Result<(), rusqlite::Error> {
        SettingsRepository::set(conn, WIDGET_CACHE_PATH_KEY, path)
    }

    pub fn set_app_group_id(conn: &Connection, app_group_id: &str) -> Result<(), rusqlite::Error> {
        SettingsRepository::set(conn, WIDGET_APP_GROUP_ID_KEY, app_group_id)
    }

    pub fn get_cache_path(conn: &Connection, app: &AppHandle) -> Result<String, String> {
        Ok(Self::resolve_cache_path(conn, app)?
            .as_os_str()
            .to_string_lossy()
            .to_string())
    }

    pub fn get_app_group_id(conn: &Connection) -> Result<String, String> {
        let app_group_id = SettingsRepository::get(conn, WIDGET_APP_GROUP_ID_KEY)
            .map_err(|e| e.to_string())?
            .unwrap_or_else(|| DEFAULT_WIDGET_APP_GROUP_ID.to_string());

        Ok(app_group_id.trim().to_string())
    }

    pub fn toggle_item_and_refresh(
        conn: &Connection,
        app: &AppHandle,
        id: i64,
        max_items: Option<usize>,
    ) -> Result<WidgetSnapshot, String> {
        Self::toggle_active_item(conn, id)?;
        Self::refresh_cache(conn, app, max_items)
    }

    pub fn process_pending_actions(
        conn: &Connection,
        app: &AppHandle,
        max_items: Option<usize>,
    ) -> Result<usize, String> {
        let actions_path = Self::resolve_actions_path(conn, app)?;
        let actions = Self::read_pending_actions(&actions_path)?;

        if actions.is_empty() {
            return Ok(0);
        }

        let processed = Self::process_toggle_actions(conn, actions);

        Self::clear_pending_actions(&actions_path)?;
        Self::refresh_cache(conn, app, max_items)?;
        Ok(processed)
    }

    fn process_toggle_actions(conn: &Connection, actions: Vec<WidgetToggleAction>) -> usize {
        let mut processed = 0usize;
        let mut seen_item_ids = HashSet::new();
        for action in actions {
            if !seen_item_ids.insert(action.item_id) {
                continue;
            }

            match Self::toggle_active_item(conn, action.item_id) {
                Ok(true) => processed += 1,
                Ok(false) => {}
                Err(error) => {
                    log::error!(
                        "Failed to process widget action for item {}: {}",
                        action.item_id,
                        error
                    );
                }
            }
        }

        processed
    }

    fn toggle_active_item(conn: &Connection, id: i64) -> Result<bool, String> {
        let Some(item) =
            ChecklistRepository::get_item_by_id(conn, id).map_err(|error| error.to_string())?
        else {
            return Ok(false);
        };

        if item.archived_at.is_some() {
            return Ok(false);
        }

        ChecklistService::toggle_item(conn, id)?;
        Ok(true)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn setup_conn() -> Connection {
        let conn = Connection::open_in_memory().expect("in-memory db");
        conn.execute(
            "CREATE TABLE settings (
                key TEXT PRIMARY KEY,
                value TEXT NOT NULL
            )",
            [],
        )
        .expect("settings schema");
        ChecklistRepository::create_tables(&conn).expect("checklist schema");
        ChecklistRepository::ensure_default_category(&conn).expect("default category");
        conn
    }

    #[test]
    fn widget_snapshot_uses_active_categories_items_and_tags() {
        let conn = setup_conn();
        let home = ChecklistService::get_categories(&conn).unwrap()[0].clone();
        let work = ChecklistService::create_category(&conn, "Work").unwrap();
        let home_item = ChecklistService::create_item_with_tags(
            &conn,
            home.id,
            "Wallet",
            &["personal".to_string()],
        )
        .unwrap();
        let done_item =
            ChecklistService::create_item_with_tags(&conn, home.id, "Done", &[]).unwrap();
        let archived_item =
            ChecklistService::create_item_with_tags(&conn, work.id, "Archived", &[]).unwrap();

        ChecklistService::toggle_item(&conn, done_item.id).unwrap();
        ChecklistService::toggle_item(&conn, archived_item.id).unwrap();
        ChecklistService::archive_completed_items(&conn, work.id).unwrap();

        let snapshot = WidgetService::get_snapshot(&conn, None).unwrap();

        assert_eq!(snapshot.total_count, 2);
        assert_eq!(snapshot.pending_count, 1);
        assert!(snapshot.items.iter().any(|item| item.id == home_item.id));
        assert!(!snapshot
            .items
            .iter()
            .any(|item| item.id == archived_item.id));

        let home_summary = snapshot
            .categories
            .iter()
            .find(|category| category.category_id == Some(home.id))
            .expect("home summary");
        assert_eq!(home_summary.total_count, 2);
        assert_eq!(home_summary.pending_count, 1);
        assert_eq!(home_summary.pending_items[0].tags, vec!["personal"]);

        let work_summary = snapshot
            .categories
            .iter()
            .find(|category| category.category_id == Some(work.id))
            .expect("work summary");
        assert_eq!(work_summary.total_count, 0);
        assert_eq!(work_summary.pending_count, 0);
    }

    #[test]
    fn widget_actions_toggle_items_once_and_ignore_archived_items() {
        let conn = setup_conn();
        let home = ChecklistService::get_categories(&conn).unwrap()[0].clone();
        let active =
            ChecklistService::create_item_with_tags(&conn, home.id, "Active", &[]).unwrap();
        let archived =
            ChecklistService::create_item_with_tags(&conn, home.id, "Archived", &[]).unwrap();

        ChecklistService::toggle_item(&conn, archived.id).unwrap();
        ChecklistService::archive_completed_items(&conn, home.id).unwrap();

        let processed = WidgetService::process_toggle_actions(
            &conn,
            vec![
                WidgetToggleAction { item_id: active.id },
                WidgetToggleAction { item_id: active.id },
                WidgetToggleAction {
                    item_id: archived.id,
                },
            ],
        );
        let active_after = ChecklistRepository::get_item_by_id(&conn, active.id)
            .unwrap()
            .unwrap();
        let archived_after = ChecklistRepository::get_item_by_id(&conn, archived.id)
            .unwrap()
            .unwrap();

        assert_eq!(processed, 1);
        assert!(active_after.done);
        assert!(archived_after.archived_at.is_some());
    }
}
