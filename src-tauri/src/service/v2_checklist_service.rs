use rusqlite::Connection;

use crate::models::{V2Category, V2ItemSearchResult, V2Tag, V2TodoItem};
use crate::repository::V2ChecklistRepository;

pub struct V2ChecklistService;

impl V2ChecklistService {
    pub fn get_categories(conn: &Connection) -> Result<Vec<V2Category>, String> {
        V2ChecklistRepository::ensure_default_category(conn).map_err(|error| error.to_string())?;
        V2ChecklistRepository::get_categories(conn).map_err(|error| error.to_string())
    }

    pub fn create_category(conn: &Connection, name: &str) -> Result<V2Category, String> {
        let trimmed_name = Self::trim_required(name, "Category name")?;
        V2ChecklistRepository::create_category(conn, trimmed_name)
            .map_err(|error| error.to_string())
    }

    pub fn update_category(conn: &Connection, id: i64, name: &str) -> Result<(), String> {
        let trimmed_name = Self::trim_required(name, "Category name")?;
        V2ChecklistRepository::update_category(conn, id, trimmed_name)
            .map_err(|error| error.to_string())
    }

    pub fn delete_category(conn: &Connection, id: i64) -> Result<(), String> {
        let category_count =
            V2ChecklistRepository::count_categories(conn).map_err(|error| error.to_string())?;
        if category_count <= 1 {
            return Err("At least one category is required.".to_string());
        }

        V2ChecklistRepository::delete_category(conn, id).map_err(|error| error.to_string())
    }

    pub fn reorder_categories(conn: &Connection, category_ids: &[i64]) -> Result<(), String> {
        V2ChecklistRepository::reorder_categories(conn, category_ids)
            .map_err(|error| error.to_string())
    }

    pub fn get_items(conn: &Connection, category_id: i64) -> Result<Vec<V2TodoItem>, String> {
        Self::require_category(conn, category_id)?;
        V2ChecklistRepository::get_items(conn, category_id).map_err(|error| error.to_string())
    }

    pub fn get_tags(conn: &Connection) -> Result<Vec<V2Tag>, String> {
        V2ChecklistRepository::get_tags(conn).map_err(|error| error.to_string())
    }

    pub fn search_items(
        conn: &Connection,
        query: &str,
        limit: i64,
    ) -> Result<Vec<V2ItemSearchResult>, String> {
        let trimmed_query = query.trim();
        if trimmed_query.is_empty() || limit <= 0 {
            return Ok(Vec::new());
        }

        let safe_limit = limit.min(50);
        V2ChecklistRepository::search_items(conn, trimmed_query, safe_limit)
            .map_err(|error| error.to_string())
    }

    pub fn create_item_with_tags(
        conn: &Connection,
        category_id: i64,
        text: &str,
        tag_names: &[String],
    ) -> Result<V2TodoItem, String> {
        Self::require_category(conn, category_id)?;
        let trimmed_text = Self::trim_required(text, "Item text")?;
        let normalized_tags = Self::normalize_tag_names(tag_names)?;
        V2ChecklistRepository::create_item(conn, category_id, trimmed_text, &normalized_tags)
            .map_err(|error| error.to_string())
    }

    pub fn update_item_text(conn: &Connection, id: i64, text: &str) -> Result<(), String> {
        let trimmed_text = Self::trim_required(text, "Item text")?;
        V2ChecklistRepository::update_item_text(conn, id, trimmed_text)
            .map_err(|error| error.to_string())
    }

    pub fn update_item_details(
        conn: &Connection,
        id: i64,
        text: &str,
        memo: Option<&str>,
        tag_names: &[String],
    ) -> Result<V2TodoItem, String> {
        let trimmed_text = Self::trim_required(text, "Item text")?;
        let normalized_memo = memo.and_then(|value| {
            let trimmed = value.trim();
            if trimmed.is_empty() {
                None
            } else {
                Some(trimmed)
            }
        });
        let normalized_tags = Self::normalize_tag_names(tag_names)?;

        V2ChecklistRepository::update_item_details(
            conn,
            id,
            trimmed_text,
            normalized_memo,
            &normalized_tags,
        )
        .map_err(|error| error.to_string())
    }

    pub fn toggle_item(conn: &Connection, id: i64) -> Result<V2TodoItem, String> {
        let Some(item) =
            V2ChecklistRepository::get_item_by_id(conn, id).map_err(|error| error.to_string())?
        else {
            return Err("Item not found.".to_string());
        };

        V2ChecklistRepository::set_item_done(conn, id, !item.done)
            .map_err(|error| error.to_string())?;

        V2ChecklistRepository::get_item_by_id(conn, id)
            .map_err(|error| error.to_string())?
            .ok_or_else(|| "Item not found after toggle.".to_string())
    }

    pub fn delete_item(conn: &Connection, id: i64) -> Result<(), String> {
        V2ChecklistRepository::delete_item(conn, id).map_err(|error| error.to_string())
    }

    pub fn reorder_items(
        conn: &Connection,
        category_id: i64,
        item_ids: &[i64],
    ) -> Result<(), String> {
        Self::require_category(conn, category_id)?;
        V2ChecklistRepository::reorder_items(conn, category_id, item_ids)
            .map_err(|error| error.to_string())
    }

    fn trim_required<'a>(value: &'a str, label: &str) -> Result<&'a str, String> {
        let trimmed = value.trim();
        if trimmed.is_empty() {
            Err(format!("{label} cannot be empty."))
        } else {
            Ok(trimmed)
        }
    }

    fn require_category(conn: &Connection, category_id: i64) -> Result<(), String> {
        match V2ChecklistRepository::get_category_by_id(conn, category_id)
            .map_err(|error| error.to_string())?
        {
            Some(_) => Ok(()),
            None => Err("Category not found.".to_string()),
        }
    }

    fn normalize_tag_names(tag_names: &[String]) -> Result<Vec<String>, String> {
        let mut normalized = Vec::new();
        let mut seen = std::collections::HashSet::new();

        for raw_name in tag_names {
            let trimmed = raw_name.trim().trim_start_matches('#').trim();
            if trimmed.is_empty() {
                continue;
            }

            if !trimmed.chars().all(|character| {
                character.is_alphanumeric() || character == '_' || character == '-'
            }) {
                return Err("Tag names can only contain letters, numbers, _, and -.".to_string());
            }

            let key = trimmed.to_lowercase();
            if seen.insert(key) {
                normalized.push(trimmed.to_string());
            }
        }

        Ok(normalized)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn setup_conn() -> Connection {
        let conn = Connection::open_in_memory().expect("in-memory db");
        V2ChecklistRepository::create_tables(&conn).expect("v2 schema");
        V2ChecklistRepository::ensure_default_category(&conn).expect("default category");
        conn
    }

    #[test]
    fn rejects_empty_category_names() {
        let conn = setup_conn();
        let error = V2ChecklistService::create_category(&conn, "  ").unwrap_err();

        assert!(error.contains("Category name"));
    }

    #[test]
    fn prevents_deleting_last_category() {
        let conn = setup_conn();
        let home = V2ChecklistService::get_categories(&conn).unwrap()[0].clone();
        let error = V2ChecklistService::delete_category(&conn, home.id).unwrap_err();

        assert!(error.contains("At least one category"));
    }

    #[test]
    fn creates_and_toggles_item() {
        let conn = setup_conn();
        let category_id = V2ChecklistService::get_categories(&conn).unwrap()[0].id;
        let item = V2ChecklistService::create_item_with_tags(&conn, category_id, "  Wallet  ", &[])
            .unwrap();

        assert_eq!(item.text, "Wallet");
        assert_eq!(item.memo, None);
        assert!(!item.done);

        let toggled = V2ChecklistService::toggle_item(&conn, item.id).unwrap();
        assert!(toggled.done);
    }

    #[test]
    fn rejects_items_for_missing_categories() {
        let conn = setup_conn();
        let error =
            V2ChecklistService::create_item_with_tags(&conn, 999, "Umbrella", &[]).unwrap_err();

        assert!(error.contains("Category not found"));
    }

    #[test]
    fn search_returns_empty_for_blank_query() {
        let conn = setup_conn();
        let results = V2ChecklistService::search_items(&conn, "  ", 8).unwrap();

        assert!(results.is_empty());
    }

    #[test]
    fn search_returns_matching_items_with_categories() {
        let conn = setup_conn();
        let home = V2ChecklistService::get_categories(&conn).unwrap()[0].clone();
        let travel = V2ChecklistService::create_category(&conn, "Travel").unwrap();
        V2ChecklistService::create_item_with_tags(&conn, home.id, "Work wallet", &[]).unwrap();
        V2ChecklistService::create_item_with_tags(&conn, travel.id, "Travel wallet", &[]).unwrap();

        let results = V2ChecklistService::search_items(&conn, "wallet", 8).unwrap();

        assert_eq!(results.len(), 2);
        assert_eq!(results[0].category.name, "Home");
        assert_eq!(results[0].item.text, "Work wallet");
        assert_eq!(results[1].category.name, "Travel");
        assert_eq!(results[1].item.text, "Travel wallet");
    }

    #[test]
    fn updates_item_details_and_normalizes_blank_memo() {
        let conn = setup_conn();
        let category_id = V2ChecklistService::get_categories(&conn).unwrap()[0].id;
        let item =
            V2ChecklistService::create_item_with_tags(&conn, category_id, "Wallet", &[]).unwrap();

        V2ChecklistService::update_item_details(
            &conn,
            item.id,
            "  Wallet and keys  ",
            Some("  front pocket  "),
            &[],
        )
        .unwrap();
        let updated = V2ChecklistRepository::get_item_by_id(&conn, item.id)
            .unwrap()
            .unwrap();

        assert_eq!(updated.text, "Wallet and keys");
        assert_eq!(updated.memo.as_deref(), Some("front pocket"));

        V2ChecklistService::update_item_details(&conn, item.id, "Wallet", Some("  "), &[]).unwrap();
        let cleared = V2ChecklistRepository::get_item_by_id(&conn, item.id)
            .unwrap()
            .unwrap();

        assert_eq!(cleared.memo, None);
    }

    #[test]
    fn search_matches_item_memo() {
        let conn = setup_conn();
        let category_id = V2ChecklistService::get_categories(&conn).unwrap()[0].id;
        let item =
            V2ChecklistService::create_item_with_tags(&conn, category_id, "Passport", &[]).unwrap();
        V2ChecklistService::update_item_details(
            &conn,
            item.id,
            "Passport",
            Some("Keep it in the blue wallet pouch."),
            &[],
        )
        .unwrap();

        let results = V2ChecklistService::search_items(&conn, "blue", 8).unwrap();

        assert_eq!(results.len(), 1);
        assert_eq!(results[0].item.id, item.id);
        assert_eq!(
            results[0].item.memo.as_deref(),
            Some("Keep it in the blue wallet pouch.")
        );
    }

    #[test]
    fn search_orders_pending_before_done_within_category() {
        let conn = setup_conn();
        let category_id = V2ChecklistService::get_categories(&conn).unwrap()[0].id;
        let done =
            V2ChecklistService::create_item_with_tags(&conn, category_id, "Charge cable", &[])
                .unwrap();
        let pending =
            V2ChecklistService::create_item_with_tags(&conn, category_id, "Charge battery", &[])
                .unwrap();
        V2ChecklistService::toggle_item(&conn, done.id).unwrap();

        let results = V2ChecklistService::search_items(&conn, "Charge", 8).unwrap();

        assert_eq!(results[0].item.id, pending.id);
        assert_eq!(results[1].item.id, done.id);
    }

    #[test]
    fn search_respects_limit() {
        let conn = setup_conn();
        let category_id = V2ChecklistService::get_categories(&conn).unwrap()[0].id;
        V2ChecklistService::create_item_with_tags(&conn, category_id, "Wallet", &[]).unwrap();
        V2ChecklistService::create_item_with_tags(&conn, category_id, "Wallet backup", &[])
            .unwrap();

        let results = V2ChecklistService::search_items(&conn, "Wallet", 1).unwrap();

        assert_eq!(results.len(), 1);
    }

    #[test]
    fn create_item_with_tags_normalizes_duplicates_and_hashes() {
        let conn = setup_conn();
        let category_id = V2ChecklistService::get_categories(&conn).unwrap()[0].id;
        let item = V2ChecklistService::create_item_with_tags(
            &conn,
            category_id,
            "Read",
            &[
                "#Church".to_string(),
                "church".to_string(),
                "morning".to_string(),
                " ".to_string(),
            ],
        )
        .unwrap();

        assert_eq!(item.tags.len(), 2);
        assert_eq!(item.tags[0].name, "Church");
        assert_eq!(item.tags[1].name, "morning");
    }

    #[test]
    fn rejects_invalid_tag_names() {
        let conn = setup_conn();
        let category_id = V2ChecklistService::get_categories(&conn).unwrap()[0].id;
        let error = V2ChecklistService::create_item_with_tags(
            &conn,
            category_id,
            "Read",
            &["bad!".to_string()],
        )
        .unwrap_err();

        assert!(error.contains("Tag names"));
    }

    #[test]
    fn search_matches_item_tags() {
        let conn = setup_conn();
        let category_id = V2ChecklistService::get_categories(&conn).unwrap()[0].id;
        let item = V2ChecklistService::create_item_with_tags(
            &conn,
            category_id,
            "Read",
            &["church".to_string()],
        )
        .unwrap();

        let results = V2ChecklistService::search_items(&conn, "church", 8).unwrap();

        assert_eq!(results.len(), 1);
        assert_eq!(results[0].item.id, item.id);
        assert_eq!(results[0].item.tags[0].name, "church");
    }
}
