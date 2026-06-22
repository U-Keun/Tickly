use rusqlite::Connection;
use tauri::State;

use crate::AppState;

mod checklist_commands;
mod native_dock_commands;
mod native_sheet_commands;
mod settings_commands;
mod sync_commands;
mod widget_commands;

pub(super) fn with_db<T, E, F>(state: &State<'_, AppState>, action: F) -> Result<T, String>
where
    E: ToString,
    F: FnOnce(&Connection) -> Result<T, E>,
{
    let db = state.db.lock().map_err(|e| e.to_string())?;
    action(&db).map_err(|e| e.to_string())
}

pub use checklist_commands::*;
pub use native_dock_commands::*;
pub use native_sheet_commands::*;
pub use settings_commands::*;
pub use sync_commands::*;
pub use widget_commands::*;
