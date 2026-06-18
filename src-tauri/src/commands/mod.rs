use rusqlite::Connection;
use tauri::State;

use crate::AppState;

mod auth_commands;
mod category_commands;
mod graph_commands;
mod realtime_commands;
mod settings_commands;
mod streak_commands;
mod sync_commands;
mod tag_commands;
mod todo_commands;
mod v2_checklist_commands;
mod v2_icloud_sync_commands;
mod v2_native_dock_commands;
mod v2_native_sheet_commands;
mod widget_commands;

pub(super) fn with_db<T, E, F>(state: &State<'_, AppState>, action: F) -> Result<T, String>
where
    E: ToString,
    F: FnOnce(&Connection) -> Result<T, E>,
{
    let db = state.db.lock().map_err(|e| e.to_string())?;
    action(&db).map_err(|e| e.to_string())
}

pub use auth_commands::*;
pub use category_commands::*;
pub use graph_commands::*;
pub use realtime_commands::*;
pub use settings_commands::*;
pub use streak_commands::*;
pub use sync_commands::*;
pub use tag_commands::*;
pub use todo_commands::*;
pub use v2_checklist_commands::*;
pub use v2_icloud_sync_commands::*;
pub use v2_native_dock_commands::*;
pub use v2_native_sheet_commands::*;
pub use widget_commands::*;
