use super::*;

impl WidgetService {
    pub(super) fn resolve_cache_path(
        conn: &Connection,
        app: &AppHandle,
    ) -> Result<PathBuf, String> {
        let custom_path = SettingsRepository::get(conn, WIDGET_CACHE_PATH_KEY)
            .map_err(|e| e.to_string())?
            .unwrap_or_default();
        let custom_path = custom_path.trim();

        if !custom_path.is_empty() {
            return Ok(PathBuf::from(custom_path));
        }

        if let Some(app_group_path) = Self::resolve_app_group_cache_path(conn)? {
            return Ok(app_group_path);
        }

        let app_data_dir = app.path().app_data_dir().map_err(|e| e.to_string())?;
        Ok(app_data_dir.join(DEFAULT_WIDGET_CACHE_FILE))
    }

    pub(super) fn resolve_actions_path(
        conn: &Connection,
        app: &AppHandle,
    ) -> Result<PathBuf, String> {
        let cache_path = Self::resolve_cache_path(conn, app)?;
        if let Some(parent) = cache_path.parent() {
            return Ok(parent.join(DEFAULT_WIDGET_ACTION_FILE));
        }
        Ok(cache_path.with_file_name(DEFAULT_WIDGET_ACTION_FILE))
    }

    fn resolve_app_group_cache_path(conn: &Connection) -> Result<Option<PathBuf>, String> {
        let app_group_id = Self::get_app_group_id(conn)?;

        if app_group_id.is_empty() {
            return Ok(None);
        }

        #[cfg(target_os = "ios")]
        {
            return Ok(resolve_ios_app_group_cache_path(&app_group_id));
        }

        #[cfg(not(target_os = "ios"))]
        {
            let _ = app_group_id;
            Ok(None)
        }
    }

    pub(super) fn normalize_limit(max_items: Option<usize>) -> usize {
        max_items
            .unwrap_or(DEFAULT_WIDGET_ITEM_LIMIT)
            .clamp(1, MAX_WIDGET_ITEM_LIMIT)
    }

    pub(super) fn category_sort_order(
        category_id: Option<i64>,
        category_order_map: &HashMap<i64, i64>,
    ) -> i64 {
        category_id
            .and_then(|id| category_order_map.get(&id).copied())
            .unwrap_or(i64::MAX)
    }

    pub(super) fn read_pending_actions(
        actions_path: &PathBuf,
    ) -> Result<Vec<WidgetToggleAction>, String> {
        if !actions_path.exists() {
            return Ok(Vec::new());
        }

        let content = fs::read_to_string(actions_path).map_err(|e| e.to_string())?;
        if content.trim().is_empty() {
            return Ok(Vec::new());
        }

        match serde_json::from_str::<Vec<WidgetToggleAction>>(&content) {
            Ok(actions) => Ok(actions),
            Err(error) => {
                log::error!("Failed to decode widget action queue: {}", error);
                let _ = fs::write(actions_path, "[]");
                Ok(Vec::new())
            }
        }
    }

    pub(super) fn clear_pending_actions(actions_path: &PathBuf) -> Result<(), String> {
        if let Some(parent) = actions_path.parent() {
            fs::create_dir_all(parent).map_err(|e| e.to_string())?;
        }
        fs::write(actions_path, "[]").map_err(|e| e.to_string())
    }

    pub(super) fn request_widget_reload() {
        #[cfg(target_os = "ios")]
        {
            request_ios_widget_reload();
        }
    }
}

#[cfg(target_os = "ios")]
fn resolve_ios_app_group_cache_path(app_group_id: &str) -> Option<PathBuf> {
    use objc2_foundation::{NSFileManager, NSString};

    let group_identifier = NSString::from_str(app_group_id);
    let file_manager = NSFileManager::defaultManager();
    let container_url =
        file_manager.containerURLForSecurityApplicationGroupIdentifier(&group_identifier)?;
    let container_path = container_url.path()?;
    Some(PathBuf::from(container_path.to_string()).join(DEFAULT_WIDGET_CACHE_FILE))
}

#[cfg(target_os = "ios")]
fn request_ios_widget_reload() {
    use objc2::msg_send;
    use objc2::runtime::{AnyClass, AnyObject};
    use std::ffi::CString;

    let class_name = match CString::new("WidgetCenter") {
        Ok(name) => name,
        Err(error) => {
            log::error!("Failed to build WidgetCenter class name: {}", error);
            return;
        }
    };

    let Some(widget_center_class) = AnyClass::get(class_name.as_c_str()) else {
        log::warn!("WidgetCenter class not found while reloading widget timelines");
        return;
    };

    let shared_center_ptr: *mut AnyObject = unsafe { msg_send![widget_center_class, sharedCenter] };
    let Some(shared_center) = (unsafe { shared_center_ptr.as_ref() }) else {
        log::warn!("WidgetCenter.sharedCenter returned null");
        return;
    };

    unsafe {
        let _: () = msg_send![shared_center, reloadAllTimelines];
    }
}
