use chrono::{Datelike, Local, NaiveDate, NaiveTime};
use rusqlite::Connection;

use crate::models::{RepeatType, TodoItem};
use crate::repository::{SettingsRepository, TodoRepository};
use crate::service::StreakService;

pub struct RepeatService;

/// Calculate the "logical date" based on reset time setting.
/// If current time is before the reset time, return yesterday.
/// Otherwise, return today.
pub fn get_logical_date(reset_time: &str) -> NaiveDate {
    let now = Local::now();
    let today = now.date_naive();

    // Parse "HH:MM" format
    let parts: Vec<&str> = reset_time.split(':').collect();
    let hour: u32 = parts.get(0).and_then(|s| s.parse().ok()).unwrap_or(0);
    let min: u32 = parts.get(1).and_then(|s| s.parse().ok()).unwrap_or(0);

    let reset_time_today = NaiveTime::from_hms_opt(hour, min, 0).unwrap_or(NaiveTime::MIN);
    let current_time = now.time();

    if current_time < reset_time_today {
        // Before reset time → still "yesterday" logically
        today - chrono::Duration::days(1)
    } else {
        today
    }
}

impl RepeatService {
    /// Calculate the next due date based on repeat type and detail
    pub fn calculate_next_due(
        repeat_type: &RepeatType,
        repeat_detail: Option<&str>,
        from_date: NaiveDate,
    ) -> Option<String> {
        match repeat_type {
            RepeatType::None => None,
            RepeatType::Daily => {
                let next = from_date + chrono::Duration::days(1);
                Some(next.format("%Y-%m-%d").to_string())
            }
            RepeatType::Weekly => {
                // repeat_detail is JSON array of weekdays [0-6], 0=Sunday
                let days: Vec<u32> = repeat_detail
                    .and_then(|s| serde_json::from_str(s).ok())
                    .unwrap_or_default();

                if days.is_empty() {
                    return None;
                }

                // Find next matching weekday
                let current_weekday = from_date.weekday().num_days_from_sunday();

                // Try to find a day in the current week after today
                for offset in 1..=7 {
                    let check_date = from_date + chrono::Duration::days(offset);
                    let check_weekday = check_date.weekday().num_days_from_sunday();
                    if days.contains(&check_weekday) {
                        return Some(check_date.format("%Y-%m-%d").to_string());
                    }
                }

                // Fallback: return the first day in the next week
                let first_day = days.iter().min().unwrap_or(&0);
                let days_until = (7 + *first_day as i64 - current_weekday as i64) % 7;
                let next = from_date
                    + chrono::Duration::days(if days_until == 0 { 7 } else { days_until });
                Some(next.format("%Y-%m-%d").to_string())
            }
            RepeatType::Monthly => {
                // repeat_detail is JSON array of days [1-31]
                let days: Vec<u32> = repeat_detail
                    .and_then(|s| serde_json::from_str(s).ok())
                    .unwrap_or_default();

                if days.is_empty() {
                    return None;
                }

                let current_day = from_date.day();
                let current_month = from_date.month();
                let current_year = from_date.year();

                // Try to find a day in the current month after today
                for &day in &days {
                    if day > current_day {
                        if let Some(next) =
                            NaiveDate::from_ymd_opt(current_year, current_month, day)
                        {
                            return Some(next.format("%Y-%m-%d").to_string());
                        }
                    }
                }

                // Move to next month
                let (next_year, next_month) = if current_month == 12 {
                    (current_year + 1, 1)
                } else {
                    (current_year, current_month + 1)
                };

                // Find the first valid day in the next month
                for &day in &days {
                    if let Some(next) = NaiveDate::from_ymd_opt(next_year, next_month, day) {
                        return Some(next.format("%Y-%m-%d").to_string());
                    }
                }

                // If no valid day found, try the smallest day
                let min_day = days.iter().min().unwrap_or(&1);
                if let Some(next) = NaiveDate::from_ymd_opt(next_year, next_month, *min_day) {
                    return Some(next.format("%Y-%m-%d").to_string());
                }

                None
            }
        }
    }

    /// Toggle an item and handle repeat logic
    /// Returns the updated item
    pub fn toggle_with_repeat(
        conn: &Connection,
        id: i64,
    ) -> Result<Option<TodoItem>, rusqlite::Error> {
        let item = TodoRepository::get_by_id(conn, id)?;

        let Some(mut item) = item else {
            return Ok(None);
        };

        if item.done {
            // Unchecking: just toggle done to false
            TodoRepository::set_done(conn, id, false, None, item.next_due_at.as_deref())?;
            item.done = false;
            // Remove completion from streak log if tracking
            if item.track_streak {
                let _ = StreakService::remove_completion(conn, id);
            }
        } else {
            Self::apply_completion(conn, id, &mut item)?;
        }

        Ok(Some(item))
    }

    fn apply_completion(
        conn: &Connection,
        id: i64,
        item: &mut TodoItem,
    ) -> Result<(), rusqlite::Error> {
        let today = Local::now().format("%Y-%m-%d").to_string();

        if item.repeat_type == RepeatType::None {
            // No repeat: mark as done
            TodoRepository::set_done(conn, id, true, Some(&today), None)?;
            item.done = true;
            item.last_completed_at = Some(today);
        } else {
            // Has repeat: calculate next due date and mark as done
            let today_date = Local::now().date_naive();
            let next_due = Self::calculate_next_due(
                &item.repeat_type,
                item.repeat_detail.as_deref(),
                today_date,
            );

            TodoRepository::set_done(conn, id, true, Some(&today), next_due.as_deref())?;
            item.done = true;
            item.last_completed_at = Some(today);
            item.next_due_at = next_due;
        }

        if item.track_streak {
            let _ = StreakService::log_completion(conn, id);
        }

        Ok(())
    }

    /// Process all repeating items and reactivate those whose due date has arrived
    /// Returns the number of items reactivated
    pub fn process_repeats(conn: &Connection) -> Result<i32, rusqlite::Error> {
        // Get reset time from settings (default: "00:00")
        let reset_time =
            SettingsRepository::get(conn, "reset_time")?.unwrap_or_else(|| "00:00".to_string());

        let logical_date = get_logical_date(&reset_time);
        let today = logical_date.format("%Y-%m-%d").to_string();

        let all_items = TodoRepository::get_all(conn)?;

        let mut reactivated = 0;

        for item in all_items {
            // Only process done items with a repeat type and a next_due_at
            if item.done && item.repeat_type != RepeatType::None {
                if let Some(ref next_due) = item.next_due_at {
                    // If today >= next_due_at, reactivate the item
                    if today >= *next_due {
                        TodoRepository::reactivate(conn, item.id)?;
                        reactivated += 1;
                    }
                }
            }
        }

        Ok(reactivated)
    }
}
