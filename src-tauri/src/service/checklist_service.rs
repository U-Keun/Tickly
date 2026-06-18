use chrono::{Datelike, Local, NaiveDate, NaiveTime};
use rusqlite::Connection;

use crate::models::{
    ChecklistArchivedItem, ChecklistCategory, ChecklistGraphData, ChecklistItemSearchResult,
    ChecklistRepeatType, ChecklistStreakHeatmap, ChecklistStreakLog, ChecklistTag,
    ChecklistTagSummary, ChecklistTodoItem,
};
use crate::repository::{ChecklistRepository, SettingsRepository};

pub struct ChecklistService;

enum ChecklistStreakCadence {
    Daily,
    Weekly([bool; 7]),
    Monthly([bool; 32]),
}

#[derive(Default)]
struct ChecklistStreakStats {
    current_streak: i64,
    longest_streak: i64,
    current_streak_dates: Vec<NaiveDate>,
    longest_streak_dates: Vec<NaiveDate>,
}

impl ChecklistStreakCadence {
    fn from_repeat(repeat_type: &ChecklistRepeatType, repeat_detail: Option<&str>) -> Self {
        match repeat_type {
            ChecklistRepeatType::None | ChecklistRepeatType::Daily => Self::Daily,
            ChecklistRepeatType::Weekly => {
                let mut weekdays = [false; 7];
                for value in
                    ChecklistService::parse_repeat_values(repeat_detail).unwrap_or_default()
                {
                    if value <= 6 {
                        weekdays[value as usize] = true;
                    }
                }

                if weekdays.iter().any(|enabled| *enabled) {
                    Self::Weekly(weekdays)
                } else {
                    Self::Daily
                }
            }
            ChecklistRepeatType::Monthly => {
                let mut month_days = [false; 32];
                for value in
                    ChecklistService::parse_repeat_values(repeat_detail).unwrap_or_default()
                {
                    if (1..=31).contains(&value) {
                        month_days[value as usize] = true;
                    }
                }

                if month_days.iter().skip(1).any(|enabled| *enabled) {
                    Self::Monthly(month_days)
                } else {
                    Self::Daily
                }
            }
        }
    }

    fn is_scheduled_on(&self, date: NaiveDate) -> bool {
        match self {
            Self::Daily => true,
            Self::Weekly(weekdays) => weekdays[date.weekday().num_days_from_sunday() as usize],
            Self::Monthly(month_days) => month_days[date.day() as usize],
        }
    }

    fn next_scheduled_after(&self, date: NaiveDate) -> NaiveDate {
        if let Self::Daily = self {
            return date + chrono::Duration::days(1);
        }

        let mut candidate = date + chrono::Duration::days(1);
        for _ in 0..400 {
            if self.is_scheduled_on(candidate) {
                return candidate;
            }
            candidate += chrono::Duration::days(1);
        }

        date + chrono::Duration::days(1)
    }
}

impl ChecklistService {
    pub fn get_categories(conn: &Connection) -> Result<Vec<ChecklistCategory>, String> {
        ChecklistRepository::ensure_default_category(conn).map_err(|error| error.to_string())?;
        ChecklistRepository::get_categories(conn).map_err(|error| error.to_string())
    }

    pub fn create_category(conn: &Connection, name: &str) -> Result<ChecklistCategory, String> {
        let trimmed_name = Self::trim_required(name, "Category name")?;
        ChecklistRepository::create_category(conn, trimmed_name).map_err(|error| error.to_string())
    }

    pub fn update_category(conn: &Connection, id: i64, name: &str) -> Result<(), String> {
        let trimmed_name = Self::trim_required(name, "Category name")?;
        ChecklistRepository::update_category(conn, id, trimmed_name)
            .map_err(|error| error.to_string())
    }

    pub fn delete_category(conn: &Connection, id: i64) -> Result<(), String> {
        let category_count =
            ChecklistRepository::count_categories(conn).map_err(|error| error.to_string())?;
        if category_count <= 1 {
            return Err("At least one category is required.".to_string());
        }

        ChecklistRepository::delete_category(conn, id).map_err(|error| error.to_string())
    }

    pub fn reorder_categories(conn: &Connection, category_ids: &[i64]) -> Result<(), String> {
        ChecklistRepository::reorder_categories(conn, category_ids)
            .map_err(|error| error.to_string())
    }

    pub fn get_items(
        conn: &Connection,
        category_id: i64,
    ) -> Result<Vec<ChecklistTodoItem>, String> {
        Self::require_category(conn, category_id)?;
        ChecklistRepository::get_items(conn, category_id).map_err(|error| error.to_string())
    }

    pub fn get_active_reminder_items(conn: &Connection) -> Result<Vec<ChecklistTodoItem>, String> {
        ChecklistRepository::get_active_reminder_items(conn).map_err(|error| error.to_string())
    }

    pub fn get_tags(conn: &Connection) -> Result<Vec<ChecklistTag>, String> {
        ChecklistRepository::get_tags(conn).map_err(|error| error.to_string())
    }

    pub fn get_tag_summaries(conn: &Connection) -> Result<Vec<ChecklistTagSummary>, String> {
        ChecklistRepository::get_tag_summaries(conn).map_err(|error| error.to_string())
    }

    pub fn rename_tag(conn: &Connection, id: i64, name: &str) -> Result<ChecklistTag, String> {
        let normalized_tags = Self::normalize_tag_names(&[name.to_string()])?;
        let normalized_name = normalized_tags
            .first()
            .ok_or_else(|| "Tag name cannot be empty.".to_string())?;

        ChecklistRepository::rename_tag(conn, id, normalized_name)
            .map_err(|error| error.to_string())
    }

    pub fn delete_tag(conn: &Connection, id: i64) -> Result<(), String> {
        ChecklistRepository::delete_tag(conn, id).map_err(|error| error.to_string())
    }

    pub fn search_items(
        conn: &Connection,
        query: &str,
        limit: i64,
    ) -> Result<Vec<ChecklistItemSearchResult>, String> {
        let trimmed_query = query.trim();
        if trimmed_query.is_empty() || limit <= 0 {
            return Ok(Vec::new());
        }

        let safe_limit = limit.min(50);
        ChecklistRepository::search_items(conn, trimmed_query, safe_limit)
            .map_err(|error| error.to_string())
    }

    pub fn create_item_with_tags(
        conn: &Connection,
        category_id: i64,
        text: &str,
        tag_names: &[String],
    ) -> Result<ChecklistTodoItem, String> {
        Self::require_category(conn, category_id)?;
        let trimmed_text = Self::trim_required(text, "Item text")?;
        let normalized_tags = Self::normalize_tag_names(tag_names)?;
        ChecklistRepository::create_item(conn, category_id, trimmed_text, &normalized_tags)
            .map_err(|error| error.to_string())
    }

    pub fn update_item_text(conn: &Connection, id: i64, text: &str) -> Result<(), String> {
        let trimmed_text = Self::trim_required(text, "Item text")?;
        ChecklistRepository::update_item_text(conn, id, trimmed_text)
            .map_err(|error| error.to_string())
    }

    pub fn update_item_details(
        conn: &Connection,
        id: i64,
        text: &str,
        memo: Option<&str>,
        tag_names: &[String],
        repeat_type: &ChecklistRepeatType,
        repeat_detail: Option<&str>,
        reminder_at: Option<&str>,
        track_streak: Option<bool>,
    ) -> Result<ChecklistTodoItem, String> {
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
        let normalized_repeat_detail = Self::normalize_repeat_detail(repeat_type, repeat_detail)?;
        let normalized_reminder_at = Self::normalize_reminder_at(reminder_at)?;
        let Some(existing_item) =
            ChecklistRepository::get_item_by_id(conn, id).map_err(|error| error.to_string())?
        else {
            return Err("Item not found.".to_string());
        };
        let logical_date = Self::logical_date(conn)?;
        let requested_track_streak = track_streak.unwrap_or(existing_item.track_streak);
        let next_track_streak = *repeat_type != ChecklistRepeatType::None && requested_track_streak;
        let next_streak_started_on = if next_track_streak {
            existing_item
                .streak_started_on
                .clone()
                .or_else(|| Some(logical_date.format("%Y-%m-%d").to_string()))
        } else {
            None
        };
        let next_due_at = if existing_item.done && *repeat_type != ChecklistRepeatType::None {
            Self::calculate_next_due(
                repeat_type,
                normalized_repeat_detail.as_deref(),
                logical_date,
            )
        } else {
            None
        };

        ChecklistRepository::update_item_details(
            conn,
            id,
            trimmed_text,
            normalized_memo,
            &normalized_tags,
            repeat_type,
            normalized_repeat_detail.as_deref(),
            next_due_at.as_deref(),
            normalized_reminder_at.as_deref(),
            next_track_streak,
            next_streak_started_on.as_deref(),
        )
        .map_err(|error| error.to_string())
    }

    pub fn toggle_item(conn: &Connection, id: i64) -> Result<ChecklistTodoItem, String> {
        let Some(item) =
            ChecklistRepository::get_item_by_id(conn, id).map_err(|error| error.to_string())?
        else {
            return Err("Item not found.".to_string());
        };

        let logical_date = Self::logical_date(conn)?;
        let completed_on = logical_date.format("%Y-%m-%d").to_string();

        if item.done {
            ChecklistRepository::restore_item(conn, id, &completed_on)
                .map_err(|error| error.to_string())?;
        } else {
            let next_due_at = Self::calculate_next_due(
                &item.repeat_type,
                item.repeat_detail.as_deref(),
                logical_date,
            );
            ChecklistRepository::complete_item(conn, id, &completed_on, next_due_at.as_deref())
                .map_err(|error| error.to_string())?;
        }

        ChecklistRepository::get_item_by_id(conn, id)
            .map_err(|error| error.to_string())?
            .ok_or_else(|| "Item not found after toggle.".to_string())
    }

    pub fn process_repeats(conn: &Connection) -> Result<i64, String> {
        let logical_date = Self::logical_date(conn)?;
        let logical_date_text = logical_date.format("%Y-%m-%d").to_string();
        ChecklistRepository::reactivate_due_repeats(conn, &logical_date_text)
            .map_err(|error| error.to_string())
    }

    pub fn archive_completed_items(conn: &Connection, category_id: i64) -> Result<i64, String> {
        Self::require_category(conn, category_id)?;
        ChecklistRepository::archive_completed_items(conn, category_id)
            .map_err(|error| error.to_string())
    }

    pub fn get_archived_items(conn: &Connection) -> Result<Vec<ChecklistArchivedItem>, String> {
        ChecklistRepository::get_archived_items(conn).map_err(|error| error.to_string())
    }

    pub fn restore_archived_item(conn: &Connection, id: i64) -> Result<ChecklistTodoItem, String> {
        ChecklistRepository::restore_archived_item(conn, id).map_err(|error| error.to_string())
    }

    pub fn delete_archived_item(conn: &Connection, id: i64) -> Result<(), String> {
        ChecklistRepository::delete_archived_item(conn, id).map_err(|error| error.to_string())
    }

    pub fn get_streak_heatmaps(conn: &Connection) -> Result<Vec<ChecklistStreakHeatmap>, String> {
        let logical_date = Self::logical_date(conn)?;
        let oldest_heatmap_date = logical_date - chrono::Duration::days(364);
        let tracked_items =
            ChecklistRepository::get_streak_items(conn).map_err(|error| error.to_string())?;

        let mut heatmaps = Vec::new();
        for tracked in tracked_items {
            let started_on = tracked
                .item
                .streak_started_on
                .as_deref()
                .and_then(|value| NaiveDate::parse_from_str(value, "%Y-%m-%d").ok())
                .unwrap_or(logical_date);
            let all_logs = ChecklistRepository::get_completion_logs_for_item(
                conn,
                tracked.item.id,
                &started_on.format("%Y-%m-%d").to_string(),
            )
            .map_err(|error| error.to_string())?;
            let cadence = ChecklistStreakCadence::from_repeat(
                &tracked.item.repeat_type,
                tracked.item.repeat_detail.as_deref(),
            );
            let completion_dates = Self::scheduled_completion_dates(&all_logs, &cadence);
            let streak_segments = Self::build_streak_segments(&completion_dates, &cadence);
            let streak_stats = Self::calculate_streaks(&streak_segments, &cadence, logical_date);
            let combo_intensity = Self::build_combo_intensity(&streak_segments);
            let logs = Self::heatmap_logs_for_recent_days(
                &all_logs,
                &combo_intensity,
                oldest_heatmap_date.max(started_on),
            );

            heatmaps.push(ChecklistStreakHeatmap {
                item: tracked.item,
                category: tracked.category,
                combo_intensity: logs
                    .iter()
                    .map(|log| log.combo_intensity)
                    .max()
                    .unwrap_or_default(),
                total_days: completion_dates.len() as i64,
                current_streak: streak_stats.current_streak,
                longest_streak: streak_stats.longest_streak,
                current_streak_dates: Self::format_dates(&streak_stats.current_streak_dates),
                longest_streak_dates: Self::format_dates(&streak_stats.longest_streak_dates),
                logs,
            });
        }

        Ok(heatmaps)
    }

    pub fn get_graph_data(conn: &Connection) -> Result<ChecklistGraphData, String> {
        ChecklistRepository::get_graph_data(conn).map_err(|error| error.to_string())
    }

    pub fn delete_item(conn: &Connection, id: i64) -> Result<(), String> {
        ChecklistRepository::delete_item(conn, id).map_err(|error| error.to_string())
    }

    pub fn reorder_items(
        conn: &Connection,
        category_id: i64,
        item_ids: &[i64],
    ) -> Result<(), String> {
        Self::require_category(conn, category_id)?;
        ChecklistRepository::reorder_items(conn, category_id, item_ids)
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
        match ChecklistRepository::get_category_by_id(conn, category_id)
            .map_err(|error| error.to_string())?
        {
            Some(_) => Ok(()),
            None => Err("Category not found.".to_string()),
        }
    }

    fn logical_date(conn: &Connection) -> Result<NaiveDate, String> {
        let reset_time =
            SettingsRepository::get(conn, "reset_time").map_err(|error| error.to_string())?;
        Ok(Self::logical_date_from_reset_time(
            reset_time.as_deref().unwrap_or("00:00"),
        ))
    }

    fn logical_date_from_reset_time(reset_time: &str) -> NaiveDate {
        let now = Local::now();
        let today = now.date_naive();
        let parts: Vec<&str> = reset_time.split(':').collect();
        let hour: u32 = parts
            .first()
            .and_then(|value| value.parse().ok())
            .unwrap_or(0);
        let minute: u32 = parts
            .get(1)
            .and_then(|value| value.parse().ok())
            .unwrap_or(0);
        let reset_time_today = NaiveTime::from_hms_opt(hour, minute, 0).unwrap_or(NaiveTime::MIN);

        if now.time() < reset_time_today {
            today - chrono::Duration::days(1)
        } else {
            today
        }
    }

    fn calculate_next_due(
        repeat_type: &ChecklistRepeatType,
        repeat_detail: Option<&str>,
        from_date: NaiveDate,
    ) -> Option<String> {
        match repeat_type {
            ChecklistRepeatType::None => None,
            ChecklistRepeatType::Daily => Some(
                (from_date + chrono::Duration::days(1))
                    .format("%Y-%m-%d")
                    .to_string(),
            ),
            ChecklistRepeatType::Weekly => {
                let days = Self::parse_repeat_values(repeat_detail)?;
                for offset in 1..=7 {
                    let check_date = from_date + chrono::Duration::days(offset);
                    let check_weekday = check_date.weekday().num_days_from_sunday();
                    if days.contains(&check_weekday) {
                        return Some(check_date.format("%Y-%m-%d").to_string());
                    }
                }
                None
            }
            ChecklistRepeatType::Monthly => {
                let days = Self::parse_repeat_values(repeat_detail)?;
                let current_day = from_date.day();
                let current_month = from_date.month();
                let current_year = from_date.year();

                for &day in &days {
                    if day > current_day {
                        if let Some(next) =
                            NaiveDate::from_ymd_opt(current_year, current_month, day)
                        {
                            return Some(next.format("%Y-%m-%d").to_string());
                        }
                    }
                }

                let (next_year, next_month) = if current_month == 12 {
                    (current_year + 1, 1)
                } else {
                    (current_year, current_month + 1)
                };

                for &day in &days {
                    if let Some(next) = NaiveDate::from_ymd_opt(next_year, next_month, day) {
                        return Some(next.format("%Y-%m-%d").to_string());
                    }
                }

                None
            }
        }
    }

    fn normalize_repeat_detail(
        repeat_type: &ChecklistRepeatType,
        repeat_detail: Option<&str>,
    ) -> Result<Option<String>, String> {
        match repeat_type {
            ChecklistRepeatType::None | ChecklistRepeatType::Daily => Ok(None),
            ChecklistRepeatType::Weekly => {
                let values = Self::parse_and_validate_repeat_values(
                    repeat_detail,
                    0,
                    6,
                    "Weekly repeat days",
                )?;
                serde_json::to_string(&values)
                    .map(Some)
                    .map_err(|error| error.to_string())
            }
            ChecklistRepeatType::Monthly => {
                let values = Self::parse_and_validate_repeat_values(
                    repeat_detail,
                    1,
                    31,
                    "Monthly repeat dates",
                )?;
                serde_json::to_string(&values)
                    .map(Some)
                    .map_err(|error| error.to_string())
            }
        }
    }

    fn normalize_reminder_at(reminder_at: Option<&str>) -> Result<Option<String>, String> {
        let Some(value) = reminder_at else {
            return Ok(None);
        };
        let trimmed = value.trim();
        if trimmed.is_empty() {
            return Ok(None);
        }

        let parts: Vec<&str> = trimmed.split(':').collect();
        if parts.len() != 2 || parts[0].len() != 2 || parts[1].len() != 2 {
            return Err("Reminder time must use HH:MM format.".to_string());
        }

        let hour = parts[0]
            .parse::<u32>()
            .map_err(|_| "Reminder time must use HH:MM format.".to_string())?;
        let minute = parts[1]
            .parse::<u32>()
            .map_err(|_| "Reminder time must use HH:MM format.".to_string())?;

        if hour > 23 || minute > 59 {
            return Err("Reminder time must use HH:MM format.".to_string());
        }

        Ok(Some(format!("{hour:02}:{minute:02}")))
    }

    fn parse_repeat_values(repeat_detail: Option<&str>) -> Option<Vec<u32>> {
        repeat_detail.and_then(|value| serde_json::from_str::<Vec<u32>>(value).ok())
    }

    fn scheduled_completion_dates(
        logs: &[ChecklistStreakLog],
        cadence: &ChecklistStreakCadence,
    ) -> Vec<NaiveDate> {
        logs.iter()
            .filter_map(|log| NaiveDate::parse_from_str(&log.completed_on, "%Y-%m-%d").ok())
            .filter(|date| cadence.is_scheduled_on(*date))
            .collect()
    }

    fn build_streak_segments(
        completion_dates: &[NaiveDate],
        cadence: &ChecklistStreakCadence,
    ) -> Vec<Vec<NaiveDate>> {
        if completion_dates.is_empty() {
            return Vec::new();
        }

        let mut segments = Vec::new();
        let mut current_segment = vec![completion_dates[0]];
        let mut previous_date = completion_dates[0];

        for date in completion_dates.iter().copied().skip(1) {
            let expected_next = cadence.next_scheduled_after(previous_date);
            if date == expected_next {
                current_segment.push(date);
            } else {
                segments.push(current_segment);
                current_segment = vec![date];
            }
            previous_date = date;
        }

        segments.push(current_segment);
        segments
    }

    fn calculate_streaks(
        streak_segments: &[Vec<NaiveDate>],
        cadence: &ChecklistStreakCadence,
        today: NaiveDate,
    ) -> ChecklistStreakStats {
        if streak_segments.is_empty() {
            return ChecklistStreakStats::default();
        }

        let mut longest_segment = &streak_segments[0];
        for segment in streak_segments.iter().skip(1) {
            if segment.len() >= longest_segment.len() {
                longest_segment = segment;
            }
        }

        let current_segment = streak_segments.last().unwrap_or(longest_segment);
        let last_completion = *current_segment.last().unwrap_or(&today);
        let next_expected = cadence.next_scheduled_after(last_completion);
        let current_streak_dates = if next_expected >= today {
            current_segment.clone()
        } else {
            Vec::new()
        };

        ChecklistStreakStats {
            current_streak: current_streak_dates.len() as i64,
            longest_streak: longest_segment.len() as i64,
            current_streak_dates,
            longest_streak_dates: longest_segment.clone(),
        }
    }

    fn build_combo_intensity(streak_segments: &[Vec<NaiveDate>]) -> Vec<(String, i64)> {
        let mut intensities = Vec::new();

        for segment in streak_segments {
            for (index, date) in segment.iter().enumerate() {
                intensities.push((
                    date.format("%Y-%m-%d").to_string(),
                    Self::combo_level_for_length(index + 1),
                ));
            }
        }

        intensities
    }

    fn combo_level_for_length(length: usize) -> i64 {
        match length {
            0 => 0,
            1 => 1,
            2 => 2,
            3 => 3,
            4 => 4,
            5..=6 => 5,
            7..=9 => 6,
            10..=13 => 7,
            14..=18 => 8,
            19..=25 => 9,
            _ => 10,
        }
    }

    fn heatmap_logs_for_recent_days(
        logs: &[ChecklistStreakLog],
        combo_intensity: &[(String, i64)],
        oldest_date: NaiveDate,
    ) -> Vec<ChecklistStreakLog> {
        logs.iter()
            .filter(|log| {
                NaiveDate::parse_from_str(&log.completed_on, "%Y-%m-%d")
                    .map(|date| date >= oldest_date)
                    .unwrap_or(false)
            })
            .map(|log| {
                let level = combo_intensity
                    .iter()
                    .find(|(completed_on, _)| completed_on == &log.completed_on)
                    .map(|(_, level)| *level)
                    .unwrap_or_default();
                ChecklistStreakLog {
                    completed_on: log.completed_on.clone(),
                    completed_count: log.completed_count,
                    combo_intensity: level,
                }
            })
            .collect()
    }

    fn format_dates(dates: &[NaiveDate]) -> Vec<String> {
        dates
            .iter()
            .map(|date| date.format("%Y-%m-%d").to_string())
            .collect()
    }

    fn parse_and_validate_repeat_values(
        repeat_detail: Option<&str>,
        min: u32,
        max: u32,
        label: &str,
    ) -> Result<Vec<u32>, String> {
        let values: Vec<u32> = repeat_detail
            .and_then(|value| serde_json::from_str(value).ok())
            .unwrap_or_default();

        let mut normalized = Vec::new();
        let mut seen = std::collections::HashSet::new();
        for value in values {
            if value < min || value > max {
                return Err(format!("{label} contain an invalid value."));
            }
            if seen.insert(value) {
                normalized.push(value);
            }
        }
        normalized.sort_unstable();

        if normalized.is_empty() {
            return Err(format!("{label} cannot be empty."));
        }

        Ok(normalized)
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

    fn date(year: i32, month: u32, day: u32) -> NaiveDate {
        NaiveDate::from_ymd_opt(year, month, day).expect("valid date")
    }

    fn completion_count(conn: &Connection, item_id: i64, completed_on: &str) -> i64 {
        conn.query_row(
            "SELECT COALESCE(completed_count, 0)
             FROM checklist_completion_logs
             WHERE item_id = ?1 AND completed_on = ?2",
            rusqlite::params![item_id, completed_on],
            |row| row.get(0),
        )
        .unwrap_or(0)
    }

    fn insert_completion_log(conn: &Connection, item_id: i64, completed_on: &str) {
        conn.execute(
            "INSERT INTO checklist_completion_logs
                (item_id, completed_on, completed_count, created_at, updated_at)
             VALUES (?1, ?2, 1, '2026-06-17T00:00:00Z', '2026-06-17T00:00:00Z')",
            rusqlite::params![item_id, completed_on],
        )
        .unwrap();
    }

    #[test]
    fn rejects_empty_category_names() {
        let conn = setup_conn();
        let error = ChecklistService::create_category(&conn, "  ").unwrap_err();

        assert!(error.contains("Category name"));
    }

    #[test]
    fn prevents_deleting_last_category() {
        let conn = setup_conn();
        let home = ChecklistService::get_categories(&conn).unwrap()[0].clone();
        let error = ChecklistService::delete_category(&conn, home.id).unwrap_err();

        assert!(error.contains("At least one category"));
    }

    #[test]
    fn creates_and_toggles_item() {
        let conn = setup_conn();
        let category_id = ChecklistService::get_categories(&conn).unwrap()[0].id;
        let item =
            ChecklistService::create_item_with_tags(&conn, category_id, "  Wallet  ", &[]).unwrap();

        assert_eq!(item.text, "Wallet");
        assert_eq!(item.memo, None);
        assert!(!item.track_streak);
        assert_eq!(item.streak_started_on, None);
        assert!(!item.done);

        let toggled = ChecklistService::toggle_item(&conn, item.id).unwrap();
        assert!(toggled.done);
    }

    #[test]
    fn enables_and_disables_streak_from_logical_date() {
        let conn = setup_conn();
        let category_id = ChecklistService::get_categories(&conn).unwrap()[0].id;
        let item =
            ChecklistService::create_item_with_tags(&conn, category_id, "Walk", &[]).unwrap();
        let logical_date = ChecklistService::logical_date(&conn)
            .unwrap()
            .format("%Y-%m-%d")
            .to_string();

        let enabled = ChecklistService::update_item_details(
            &conn,
            item.id,
            "Walk",
            None,
            &[],
            &ChecklistRepeatType::Daily,
            None,
            None,
            Some(true),
        )
        .unwrap();

        assert!(enabled.track_streak);
        assert_eq!(
            enabled.streak_started_on.as_deref(),
            Some(logical_date.as_str())
        );

        let disabled = ChecklistService::update_item_details(
            &conn,
            item.id,
            "Walk",
            None,
            &[],
            &ChecklistRepeatType::Daily,
            None,
            None,
            Some(false),
        )
        .unwrap();

        assert!(!disabled.track_streak);
        assert_eq!(disabled.streak_started_on, None);
    }

    #[test]
    fn prevents_streak_without_repeat() {
        let conn = setup_conn();
        let category_id = ChecklistService::get_categories(&conn).unwrap()[0].id;
        let item =
            ChecklistService::create_item_with_tags(&conn, category_id, "Walk", &[]).unwrap();

        let updated = ChecklistService::update_item_details(
            &conn,
            item.id,
            "Walk",
            None,
            &[],
            &ChecklistRepeatType::None,
            None,
            None,
            Some(true),
        )
        .unwrap();

        assert!(!updated.track_streak);
        assert_eq!(updated.streak_started_on, None);
    }

    #[test]
    fn clears_streak_when_repeat_is_removed() {
        let conn = setup_conn();
        let category_id = ChecklistService::get_categories(&conn).unwrap()[0].id;
        let item =
            ChecklistService::create_item_with_tags(&conn, category_id, "Walk", &[]).unwrap();

        let enabled = ChecklistService::update_item_details(
            &conn,
            item.id,
            "Walk",
            None,
            &[],
            &ChecklistRepeatType::Daily,
            None,
            None,
            Some(true),
        )
        .unwrap();
        assert!(enabled.track_streak);

        let disabled = ChecklistService::update_item_details(
            &conn,
            item.id,
            "Walk",
            None,
            &[],
            &ChecklistRepeatType::None,
            None,
            None,
            None,
        )
        .unwrap();

        assert!(!disabled.track_streak);
        assert_eq!(disabled.streak_started_on, None);
    }

    #[test]
    fn streak_heatmap_ignores_logs_before_streak_started_on() {
        let conn = setup_conn();
        let category_id = ChecklistService::get_categories(&conn).unwrap()[0].id;
        let item =
            ChecklistService::create_item_with_tags(&conn, category_id, "Read", &[]).unwrap();
        conn.execute(
            "UPDATE checklist_todos
             SET repeat_type = 'daily', track_streak = 1, streak_started_on = '2026-06-01'
             WHERE id = ?1",
            rusqlite::params![item.id],
        )
        .unwrap();
        insert_completion_log(&conn, item.id, "2026-05-31");
        insert_completion_log(&conn, item.id, "2026-06-01");
        insert_completion_log(&conn, item.id, "2026-06-02");

        let heatmaps = ChecklistService::get_streak_heatmaps(&conn).unwrap();

        assert_eq!(heatmaps.len(), 1);
        assert_eq!(heatmaps[0].logs.len(), 2);
        assert_eq!(heatmaps[0].logs[0].completed_on, "2026-06-01");
        assert_eq!(heatmaps[0].total_days, 2);
        assert!(heatmaps[0].longest_streak >= 2);
    }

    #[test]
    fn streak_heatmap_excludes_non_repeating_tracked_items() {
        let conn = setup_conn();
        let category_id = ChecklistService::get_categories(&conn).unwrap()[0].id;
        let item =
            ChecklistService::create_item_with_tags(&conn, category_id, "Read", &[]).unwrap();
        conn.execute(
            "UPDATE checklist_todos
             SET repeat_type = 'none', track_streak = 1, streak_started_on = '2026-06-01'
             WHERE id = ?1",
            rusqlite::params![item.id],
        )
        .unwrap();
        insert_completion_log(&conn, item.id, "2026-06-01");

        let heatmaps = ChecklistService::get_streak_heatmaps(&conn).unwrap();

        assert!(heatmaps.is_empty());
    }

    #[test]
    fn weekly_streak_counts_scheduled_slots_only() {
        let logs = vec![
            ChecklistStreakLog {
                completed_on: "2026-03-02".to_string(),
                completed_count: 1,
                combo_intensity: 0,
            },
            ChecklistStreakLog {
                completed_on: "2026-03-04".to_string(),
                completed_count: 1,
                combo_intensity: 0,
            },
        ];
        let cadence =
            ChecklistStreakCadence::from_repeat(&ChecklistRepeatType::Weekly, Some("[1,3,5]"));
        let completion_dates = ChecklistService::scheduled_completion_dates(&logs, &cadence);
        let streak_segments = ChecklistService::build_streak_segments(&completion_dates, &cadence);
        let stats =
            ChecklistService::calculate_streaks(&streak_segments, &cadence, date(2026, 3, 6));

        assert_eq!((stats.current_streak, stats.longest_streak), (2, 2));
    }

    #[test]
    fn calculates_next_due_for_daily_weekly_and_monthly() {
        assert_eq!(
            ChecklistService::calculate_next_due(
                &ChecklistRepeatType::Daily,
                None,
                date(2026, 6, 16)
            ),
            Some("2026-06-17".to_string())
        );
        assert_eq!(
            ChecklistService::calculate_next_due(
                &ChecklistRepeatType::Weekly,
                Some("[4]"),
                date(2026, 6, 16),
            ),
            Some("2026-06-18".to_string())
        );
        assert_eq!(
            ChecklistService::calculate_next_due(
                &ChecklistRepeatType::Weekly,
                Some("[2]"),
                date(2026, 6, 16),
            ),
            Some("2026-06-23".to_string())
        );
        assert_eq!(
            ChecklistService::calculate_next_due(
                &ChecklistRepeatType::Monthly,
                Some("[20]"),
                date(2026, 6, 16),
            ),
            Some("2026-06-20".to_string())
        );
    }

    #[test]
    fn rejects_empty_weekly_repeat_detail() {
        let conn = setup_conn();
        let category_id = ChecklistService::get_categories(&conn).unwrap()[0].id;
        let item = ChecklistService::create_item_with_tags(&conn, category_id, "Water plants", &[])
            .unwrap();

        let error = ChecklistService::update_item_details(
            &conn,
            item.id,
            "Water plants",
            None,
            &[],
            &ChecklistRepeatType::Weekly,
            Some("[]"),
            None,
            None,
        )
        .unwrap_err();

        assert!(error.contains("Weekly repeat days"));
    }

    #[test]
    fn completing_repeat_item_sets_due_date_and_completion_log() {
        let conn = setup_conn();
        let category_id = ChecklistService::get_categories(&conn).unwrap()[0].id;
        let item =
            ChecklistService::create_item_with_tags(&conn, category_id, "Stretch", &[]).unwrap();

        ChecklistService::update_item_details(
            &conn,
            item.id,
            "Stretch",
            None,
            &[],
            &ChecklistRepeatType::Daily,
            None,
            None,
            None,
        )
        .unwrap();
        let completed = ChecklistService::toggle_item(&conn, item.id).unwrap();

        assert!(completed.done);
        assert_eq!(completed.repeat_type, ChecklistRepeatType::Daily);
        assert!(completed.next_due_at.is_some());
        let completed_on = completed.last_completed_at.as_deref().unwrap();
        assert_eq!(completion_count(&conn, item.id, completed_on), 1);
    }

    #[test]
    fn restoring_completed_item_removes_today_completion_log() {
        let conn = setup_conn();
        let category_id = ChecklistService::get_categories(&conn).unwrap()[0].id;
        let item =
            ChecklistService::create_item_with_tags(&conn, category_id, "Journal", &[]).unwrap();
        let completed = ChecklistService::toggle_item(&conn, item.id).unwrap();
        let completed_on = completed.last_completed_at.clone().unwrap();

        let restored = ChecklistService::toggle_item(&conn, item.id).unwrap();

        assert!(!restored.done);
        assert_eq!(restored.last_completed_at, None);
        assert_eq!(restored.next_due_at, None);
        assert_eq!(completion_count(&conn, item.id, &completed_on), 0);
    }

    #[test]
    fn process_repeats_reactivates_due_repeat_items() {
        let conn = setup_conn();
        let category_id = ChecklistService::get_categories(&conn).unwrap()[0].id;
        let item =
            ChecklistService::create_item_with_tags(&conn, category_id, "Vitamins", &[]).unwrap();

        ChecklistService::update_item_details(
            &conn,
            item.id,
            "Vitamins",
            None,
            &[],
            &ChecklistRepeatType::Daily,
            None,
            Some("08:15"),
            None,
        )
        .unwrap();
        ChecklistService::toggle_item(&conn, item.id).unwrap();
        conn.execute(
            "UPDATE checklist_todos SET next_due_at = '2000-01-01' WHERE id = ?1",
            rusqlite::params![item.id],
        )
        .unwrap();

        let reactivated = ChecklistService::process_repeats(&conn).unwrap();
        let next_item = ChecklistRepository::get_item_by_id(&conn, item.id)
            .unwrap()
            .unwrap();

        assert_eq!(reactivated, 1);
        assert!(!next_item.done);
        assert_eq!(next_item.next_due_at, None);
        assert!(next_item.last_completed_at.is_some());
        assert_eq!(next_item.reminder_at.as_deref(), Some("08:15"));
    }

    #[test]
    fn rejects_items_for_missing_categories() {
        let conn = setup_conn();
        let error =
            ChecklistService::create_item_with_tags(&conn, 999, "Umbrella", &[]).unwrap_err();

        assert!(error.contains("Category not found"));
    }

    #[test]
    fn search_returns_empty_for_blank_query() {
        let conn = setup_conn();
        let results = ChecklistService::search_items(&conn, "  ", 8).unwrap();

        assert!(results.is_empty());
    }

    #[test]
    fn search_returns_matching_items_with_categories() {
        let conn = setup_conn();
        let home = ChecklistService::get_categories(&conn).unwrap()[0].clone();
        let travel = ChecklistService::create_category(&conn, "Travel").unwrap();
        ChecklistService::create_item_with_tags(&conn, home.id, "Work wallet", &[]).unwrap();
        ChecklistService::create_item_with_tags(&conn, travel.id, "Travel wallet", &[]).unwrap();

        let results = ChecklistService::search_items(&conn, "wallet", 8).unwrap();

        assert_eq!(results.len(), 2);
        assert_eq!(results[0].category.name, "Home");
        assert_eq!(results[0].item.text, "Work wallet");
        assert_eq!(results[1].category.name, "Travel");
        assert_eq!(results[1].item.text, "Travel wallet");
    }

    #[test]
    fn updates_item_details_and_normalizes_blank_memo() {
        let conn = setup_conn();
        let category_id = ChecklistService::get_categories(&conn).unwrap()[0].id;
        let item =
            ChecklistService::create_item_with_tags(&conn, category_id, "Wallet", &[]).unwrap();

        ChecklistService::update_item_details(
            &conn,
            item.id,
            "  Wallet and keys  ",
            Some("  front pocket  "),
            &[],
            &ChecklistRepeatType::None,
            None,
            Some("09:30"),
            None,
        )
        .unwrap();
        let updated = ChecklistRepository::get_item_by_id(&conn, item.id)
            .unwrap()
            .unwrap();

        assert_eq!(updated.text, "Wallet and keys");
        assert_eq!(updated.memo.as_deref(), Some("front pocket"));
        assert_eq!(updated.reminder_at.as_deref(), Some("09:30"));

        ChecklistService::update_item_details(
            &conn,
            item.id,
            "Wallet",
            Some("  "),
            &[],
            &ChecklistRepeatType::None,
            None,
            Some("  "),
            None,
        )
        .unwrap();
        let cleared = ChecklistRepository::get_item_by_id(&conn, item.id)
            .unwrap()
            .unwrap();

        assert_eq!(cleared.memo, None);
        assert_eq!(cleared.reminder_at, None);
    }

    #[test]
    fn rejects_invalid_reminder_time() {
        let conn = setup_conn();
        let category_id = ChecklistService::get_categories(&conn).unwrap()[0].id;
        let item =
            ChecklistService::create_item_with_tags(&conn, category_id, "Medicine", &[]).unwrap();

        let error = ChecklistService::update_item_details(
            &conn,
            item.id,
            "Medicine",
            None,
            &[],
            &ChecklistRepeatType::None,
            None,
            Some("25:99"),
            None,
        )
        .unwrap_err();

        assert!(error.contains("Reminder time"));
    }

    #[test]
    fn search_matches_item_memo() {
        let conn = setup_conn();
        let category_id = ChecklistService::get_categories(&conn).unwrap()[0].id;
        let item =
            ChecklistService::create_item_with_tags(&conn, category_id, "Passport", &[]).unwrap();
        ChecklistService::update_item_details(
            &conn,
            item.id,
            "Passport",
            Some("Keep it in the blue wallet pouch."),
            &[],
            &ChecklistRepeatType::None,
            None,
            None,
            None,
        )
        .unwrap();

        let results = ChecklistService::search_items(&conn, "blue", 8).unwrap();

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
        let category_id = ChecklistService::get_categories(&conn).unwrap()[0].id;
        let done = ChecklistService::create_item_with_tags(&conn, category_id, "Charge cable", &[])
            .unwrap();
        let pending =
            ChecklistService::create_item_with_tags(&conn, category_id, "Charge battery", &[])
                .unwrap();
        ChecklistService::toggle_item(&conn, done.id).unwrap();

        let results = ChecklistService::search_items(&conn, "Charge", 8).unwrap();

        assert_eq!(results[0].item.id, pending.id);
        assert_eq!(results[1].item.id, done.id);
    }

    #[test]
    fn search_respects_limit() {
        let conn = setup_conn();
        let category_id = ChecklistService::get_categories(&conn).unwrap()[0].id;
        ChecklistService::create_item_with_tags(&conn, category_id, "Wallet", &[]).unwrap();
        ChecklistService::create_item_with_tags(&conn, category_id, "Wallet backup", &[]).unwrap();

        let results = ChecklistService::search_items(&conn, "Wallet", 1).unwrap();

        assert_eq!(results.len(), 1);
    }

    #[test]
    fn create_item_with_tags_normalizes_duplicates_and_hashes() {
        let conn = setup_conn();
        let category_id = ChecklistService::get_categories(&conn).unwrap()[0].id;
        let item = ChecklistService::create_item_with_tags(
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
        let category_id = ChecklistService::get_categories(&conn).unwrap()[0].id;
        let error = ChecklistService::create_item_with_tags(
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
        let category_id = ChecklistService::get_categories(&conn).unwrap()[0].id;
        let item = ChecklistService::create_item_with_tags(
            &conn,
            category_id,
            "Read",
            &["church".to_string()],
        )
        .unwrap();

        let results = ChecklistService::search_items(&conn, "church", 8).unwrap();

        assert_eq!(results.len(), 1);
        assert_eq!(results[0].item.id, item.id);
        assert_eq!(results[0].item.tags[0].name, "church");
    }
}
