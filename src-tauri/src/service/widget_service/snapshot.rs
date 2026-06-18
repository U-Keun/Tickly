use std::collections::HashMap;

use super::*;

impl WidgetService {
    pub fn get_snapshot(
        conn: &Connection,
        max_items: Option<usize>,
    ) -> Result<WidgetSnapshot, rusqlite::Error> {
        ChecklistRepository::ensure_default_category(conn)?;
        let categories = ChecklistRepository::get_categories(conn)?;
        let category_name_map: HashMap<i64, String> = categories
            .iter()
            .map(|cat| (cat.id, cat.name.clone()))
            .collect();
        let category_order_map: HashMap<i64, i64> = categories
            .iter()
            .map(|cat| (cat.id, cat.display_order))
            .collect();
        let mut todos = Vec::new();
        let mut category_counts: HashMap<i64, (usize, usize)> = HashMap::new();

        for category in &categories {
            let category_items = ChecklistRepository::get_items(conn, category.id)?;
            let entry = category_counts.entry(category.id).or_insert((0, 0));
            entry.0 = category_items.len();
            entry.1 = category_items.iter().filter(|item| !item.done).count();
            todos.extend(category_items);
        }

        todos.sort_by(|a, b| {
            Self::category_sort_order(Some(a.category_id), &category_order_map)
                .cmp(&Self::category_sort_order(
                    Some(b.category_id),
                    &category_order_map,
                ))
                .then(a.done.cmp(&b.done))
                .then(a.display_order.cmp(&b.display_order))
                .then(a.id.cmp(&b.id))
        });

        let mut pending_item_ids_map: HashMap<i64, Vec<i64>> = HashMap::new();
        let mut pending_items_map: HashMap<i64, Vec<WidgetCategoryPendingItem>> = HashMap::new();
        for todo in &todos {
            if !todo.done {
                let tags = todo.tags.iter().map(|tag| tag.name.clone()).collect();
                pending_item_ids_map
                    .entry(todo.category_id)
                    .or_default()
                    .push(todo.id);
                pending_items_map.entry(todo.category_id).or_default().push(
                    WidgetCategoryPendingItem {
                        id: todo.id,
                        text: todo.text.clone(),
                        display_order: todo.display_order,
                        tags,
                    },
                );
            }
        }

        let limit = Self::normalize_limit(max_items);
        let total_count = todos.len();
        let pending_count = todos.iter().filter(|item| !item.done).count();
        let items = todos
            .into_iter()
            .take(limit)
            .map(|item| WidgetTodoItem {
                id: item.id,
                text: item.text,
                done: item.done,
                category_id: Some(item.category_id),
                category_name: category_name_map.get(&item.category_id).cloned(),
                display_order: item.display_order,
                reminder_at: item.reminder_at,
                updated_at: Some(item.updated_at),
            })
            .collect();
        let mut widget_categories: Vec<WidgetCategorySummary> = categories
            .iter()
            .map(|category| {
                let (total_count, pending_count) =
                    category_counts.get(&category.id).copied().unwrap_or((0, 0));
                let pending_item_ids = pending_item_ids_map
                    .get(&category.id)
                    .cloned()
                    .unwrap_or_default();
                let pending_items = pending_items_map
                    .get(&category.id)
                    .cloned()
                    .unwrap_or_default();

                WidgetCategorySummary {
                    category_id: Some(category.id),
                    category_name: category.name.clone(),
                    total_count,
                    pending_count,
                    first_pending_item_id: pending_item_ids.first().copied(),
                    pending_item_ids,
                    pending_items,
                }
            })
            .collect();

        widget_categories.sort_by(|a, b| {
            Self::category_sort_order(a.category_id, &category_order_map)
                .cmp(&Self::category_sort_order(
                    b.category_id,
                    &category_order_map,
                ))
                .then(a.category_name.cmp(&b.category_name))
        });

        let theme = Self::resolve_widget_theme(conn);

        Ok(WidgetSnapshot {
            generated_at: chrono::Utc::now().format("%Y-%m-%dT%H:%M:%SZ").to_string(),
            total_count,
            pending_count,
            items,
            categories: widget_categories,
            theme,
        })
    }
}
