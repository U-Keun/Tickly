use tauri::State;

use super::with_db;
use crate::models::{SyncResult, SyncStatusInfo};
use crate::service::{AuthService, SyncService};
use crate::AppState;

#[tauri::command]
pub fn trigger_sync(state: State<'_, AppState>) -> Result<SyncResult, String> {
    let client = state.supabase.as_ref().ok_or("Supabase not configured")?;
    with_db(&state, |conn| {
        // Get session
        let session = AuthService::get_current_session(conn)?.ok_or("Not logged in")?;

        // Check if token needs refresh (we'll skip refresh for now in sync context)
        // A more robust implementation would handle token refresh here
        if AuthService::is_session_expired(&session) {
            return Err("Session expired. Please log in again.".to_string());
        }

        // Perform sync
        SyncService::sync_all_blocking(conn, client, &session.access_token, &session.user_id)
    })
}

#[tauri::command]
pub fn get_sync_status(state: State<'_, AppState>) -> Result<SyncStatusInfo, String> {
    with_db(&state, |conn| {
        let is_enabled = SyncService::is_sync_enabled(conn)?;
        let pending_count = SyncService::get_pending_count(conn)?;
        let last_synced_at = SyncService::get_last_synced_at(conn)?;

        // Check if logged in
        let is_online = match AuthService::get_current_session(conn)? {
            Some(session) => !AuthService::is_session_expired(&session),
            None => false,
        };

        Ok::<SyncStatusInfo, String>(SyncStatusInfo {
            is_enabled,
            is_syncing: false, // This would need to be tracked elsewhere for real-time status
            is_online,
            pending_count,
            last_synced_at,
        })
    })
}

#[tauri::command]
pub fn get_pending_count(state: State<'_, AppState>) -> Result<usize, String> {
    with_db(&state, |conn| SyncService::get_pending_count(conn))
}

#[tauri::command]
pub fn set_sync_enabled(state: State<'_, AppState>, enabled: bool) -> Result<(), String> {
    with_db(&state, |conn| SyncService::set_sync_enabled(conn, enabled))
}

#[tauri::command]
pub fn is_sync_enabled(state: State<'_, AppState>) -> Result<bool, String> {
    with_db(&state, |conn| SyncService::is_sync_enabled(conn))
}

#[tauri::command]
pub fn force_pull(state: State<'_, AppState>) -> Result<(), String> {
    with_db(&state, |conn| {
        // Delete all local data to force re-pull from server
        // Order matters due to foreign key constraints
        conn.execute("DELETE FROM todo_tags", [])
            .map_err(|e| e.to_string())?;
        conn.execute("DELETE FROM completion_logs", [])
            .map_err(|e| e.to_string())?;
        conn.execute("DELETE FROM todos", [])
            .map_err(|e| e.to_string())?;
        conn.execute("DELETE FROM tags", [])
            .map_err(|e| e.to_string())?;
        conn.execute("DELETE FROM categories", [])
            .map_err(|e| e.to_string())?;

        Ok::<(), String>(())
    })
}
