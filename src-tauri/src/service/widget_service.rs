use std::collections::{HashMap, HashSet};
use std::fs;
use std::path::PathBuf;

use rusqlite::Connection;
use serde::Deserialize;
use tauri::{AppHandle, Manager};

use crate::models::{
    WidgetCategoryPendingItem, WidgetCategorySummary, WidgetSnapshot, WidgetTheme, WidgetTodoItem,
};
use crate::repository::{
    CategoryRepository, SettingsRepository, TodoRepository, TodoTagRepository,
};

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
        crate::service::RepeatService::complete_with_repeat(conn, id).map_err(|e| e.to_string())?;
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

        let mut processed = 0usize;
        let mut seen_item_ids = HashSet::new();
        for action in actions {
            if !seen_item_ids.insert(action.item_id) {
                continue;
            }

            match crate::service::RepeatService::complete_with_repeat(conn, action.item_id) {
                Ok(Some(_)) => processed += 1,
                Ok(None) => {}
                Err(error) => {
                    log::error!(
                        "Failed to process widget action for item {}: {}",
                        action.item_id,
                        error
                    );
                }
            }
        }

        Self::clear_pending_actions(&actions_path)?;
        Self::refresh_cache(conn, app, max_items)?;
        Ok(processed)
    }
}
