use rusqlite::Connection;

use crate::models::Category;
use crate::repository::{CategoryRepository, CompletionLogRepository};

pub struct CategoryService;

impl CategoryService {
    pub fn get_all(conn: &Connection) -> Result<Vec<Category>, rusqlite::Error> {
        CategoryRepository::get_all(conn)
    }

    pub fn create(conn: &Connection, name: &str) -> Result<Category, rusqlite::Error> {
        CategoryRepository::create(conn, name)
    }

    pub fn update(conn: &Connection, id: i64, name: &str) -> Result<(), rusqlite::Error> {
        CategoryRepository::update(conn, id, name)
    }

    pub fn delete(conn: &Connection, id: i64) -> Result<(), rusqlite::Error> {
        // Check if category has been synced (has sync_id)
        if let Some(category) = CategoryRepository::get_by_id(conn, id)? {
            CompletionLogRepository::delete_for_category(conn, id)?;
            if category.sync_id.is_some() {
                // Category was synced - mark as deleted, also mark all todos in this category
                // First mark all todos in this category as deleted
                conn.execute(
                    "UPDATE todos SET sync_status = 'deleted', updated_at = datetime('now') WHERE category_id = ?1 AND sync_id IS NOT NULL",
                    rusqlite::params![id],
                )?;
                // Delete todos that were never synced
                conn.execute(
                    "DELETE FROM todos WHERE category_id = ?1 AND sync_id IS NULL",
                    rusqlite::params![id],
                )?;
                // Mark category as deleted
                CategoryRepository::mark_deleted(conn, id)
            } else {
                // Category was never synced - delete immediately
                CategoryRepository::delete(conn, id)
            }
        } else {
            Ok(())
        }
    }

    pub fn reorder(conn: &Connection, category_ids: &[i64]) -> Result<(), rusqlite::Error> {
        CategoryRepository::reorder(conn, category_ids)
    }
}
