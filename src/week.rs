/*
 * File: week.rs
 * Description: The definition of Week struct. Contains the user added Days.
 * Author: dherslof
 * Created: 13-12-2024
 * License: MIT
 */

use crate::day::Day;
use chrono::NaiveDate;
use serde::{Deserialize, Serialize};

/// Represents a week with a number, ISO-style
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Week {
    /// Number
    number: u32,
    /// Week  entries
    entries: Vec<Day>,
    /// Year
    year: i32,
    /// Target hours
    target_hours: f32,
}

impl Week {
    /// Create a new week
    pub fn new(number: u32, year: i32, target_hours: f32) -> Self {
        Self {
            number,
            entries: Vec::new(),
            year,
            target_hours,
        }
    }

    /// Getter for `number`
    pub fn number(&self) -> u32 {
        self.number
    }

    /// Getter for `entries`
    pub fn entries(&self) -> &Vec<Day> {
        &self.entries
    }

    /// Mutable getter for `entries`
    pub fn entries_mut(&mut self) -> &mut Vec<Day> {
        &mut self.entries
    }

    pub fn year(&self) -> i32 {
        self.year
    }

    pub fn target_hours(&self) -> f32 {
        self.target_hours
    }

    // Uncomment if needed
    //  pub fn target_hours_reached(&self) -> bool {
    //      let total_hours: f32 = self.entries.iter().map(|d| d.hours()).sum();
    //      total_hours >= self.target_hours
    //  }

    /// Add a new entry to the project
    pub fn add_entry(&mut self, entry: Day) {
        self.entries.push(entry);
    }

    /// Check if a day exists in the week based on Day
    pub fn exists(&self, entry: &Day) -> bool {
        for e in &self.entries {
            if e.date() == entry.date() {
                return true;
            }
        }
        false
    }

    /// Overloaded function. Check if a day exists in the week based on NaiveDate
    pub fn exist(&self, date: &NaiveDate) -> bool {
        for e in &self.entries {
            if e.date() == *date {
                return true;
            }
        }
        false
    }

    /// Modify data for a specific day in the vector
    pub fn merge_day(&mut self, entry: &Day) -> bool {
        // Find the day by matching the date
        if let Some(day) = self.entries.iter_mut().find(|d| d.date() == entry.date()) {
            // Modify the data
            if day.closed() {
                tracing::warn!("Day already closed, unable to modify it");
                return false;
            }

            if !day.start_time_set() && entry.start_time_set() {
                tracing::debug!("Setting start time for the day");
                day.set_starting_time(entry.starting_time());
            }

            if !day.ending_time_set() && entry.ending_time_set() {
                tracing::debug!("Setting ending time for the day");
                day.set_ending_time(entry.ending_time());
            }

            if day.extra_info().is_empty() && !entry.extra_info().is_empty() {
                tracing::debug!("Setting extra info for the day");
                day.set_extra_info(entry.extra_info().to_string());
            }

            true
        } else {
            // Day not found
            false
        }
    }

    /// Return a copy of an Day
    pub fn get_day_copy(&self, date: &NaiveDate) -> Option<Day> {
        for d in &self.entries {
            if d.date() == *date {
                return Some(d.clone());
            }
        }
        None
    }

    /// Remove Day from the week
    pub fn remove_listed_day(&mut self, date: &NaiveDate) -> bool {
        if self.exist(date) {
            self.entries.retain(|d| d.date() != *date);
            true
        } else {
            false
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::{Duration, Local, TimeZone};

    #[test]
    fn test_week_creation() {
        let week = Week::new(1, 2026, 40.0);
        assert_eq!(week.number(), 1);
        assert_eq!(week.year(), 2026);
        assert_eq!(week.target_hours(), 40.0);
        assert!(week.entries().is_empty());
    }

    #[test]
    fn test_week_add_entry() {
        let mut week = Week::new(1, 2026, 40.0);
        let day = Day::new(None);
        week.add_entry(day.clone());
        assert_eq!(week.entries().len(), 1);
        assert_eq!(week.entries()[0].date(), day.date());
    }

    #[test]
    fn test_week_exists() {
        let mut week = Week::new(1, 2026, 40.0);
        let day = Day::new(None);
        week.add_entry(day.clone());
        assert!(week.exists(&day));
        assert!(week.exist(&day.date()));
    }

    #[test]
    fn test_week_exists_false_when_not_added() {
        let week = Week::new(1, 2026, 40.0);
        let day = Day::new(None);

        assert!(!week.exists(&day));
    }

    #[test]
    fn test_week_exist_false_for_empty_week() {
        let week = Week::new(1, 2026, 40.0);
        let date = NaiveDate::from_ymd_opt(2026, 1, 1).unwrap();

        assert!(!week.exist(&date));
    }

    #[test]
    fn test_week_merge_day() {
        let mut week = Week::new(1, 2026, 40.0);
        let mut day = Day::new(None);
        day.set_extra_info("Initial info".to_string());
        week.add_entry(day.clone());

        let mut modified_day = day.clone();
        modified_day.set_extra_info("Modified info".to_string());

        assert!(week.merge_day(&modified_day));
        assert_eq!(week.entries()[0].extra_info(), "Initial info");
    }

    #[test]
    fn test_week_remove_listed_day() {
        let mut week = Week::new(1, 2026, 40.0);
        let day = Day::new(None);
        week.add_entry(day.clone());

        assert!(week.remove_listed_day(&day.date()));
        assert!(!week.exist(&day.date()));
    }

    #[test]
    fn test_week_remove_listed_day_nonexistent_returns_false() {
        let mut week = Week::new(1, 2026, 40.0);
        let day = Day::new(None);
        week.add_entry(day);

        let missing_date = NaiveDate::from_ymd_opt(2026, 1, 1).unwrap();
        assert!(!week.remove_listed_day(&missing_date));
        assert_eq!(week.entries().len(), 1);
    }

    #[test]
    fn test_week_get_day_copy() {
        let mut week = Week::new(1, 2026, 40.0);
        let mut day = Day::new(None);
        day.set_extra_info("Keep this info".to_string());
        week.add_entry(day.clone());

        let day_copy = week.get_day_copy(&day.date());
        assert!(day_copy.is_some());
        let day_copy = day_copy.unwrap();
        assert_eq!(day_copy.date(), day.date());
        assert_eq!(day_copy.extra_info(), "Keep this info");
    }

    #[test]
    fn test_week_get_day_copy_nonexistent() {
        let week = Week::new(1, 2026, 40.0);
        let day_copy = week.get_day_copy(&NaiveDate::from_ymd_opt(2026, 1, 1).unwrap());
        assert!(day_copy.is_none());
    }

    #[test]
    fn test_week_merge_day_nonexistent() {
        let mut week = Week::new(1, 2026, 40.0);
        let day = Day::new(None);
        assert!(!week.merge_day(&day));
        assert!(week.entries().is_empty());
    }

    #[test]
    fn test_week_merge_day_closed() {
        let mut week = Week::new(1, 2026, 40.0);
        let mut day = Day::new(None);
        day.set_extra_info("Initial info".to_string());
        let start = chrono::Local::now();
        let end = start + chrono::Duration::hours(8);
        day.set_starting_time(Some(&start));
        day.set_ending_time(Some(&end));
        week.add_entry(day.clone());

        let mut modified_day = day.clone();
        modified_day.set_extra_info("Modified info".to_string());

        assert!(!week.merge_day(&modified_day));
        assert_eq!(week.entries()[0].extra_info(), "Initial info");
    }

    #[test]
    fn test_week_entries_mut_can_push_day() {
        let mut week = Week::new(1, 2026, 40.0);
        let day = Day::new(None);

        week.entries_mut().push(day);

        assert_eq!(week.entries().len(), 1);
    }

    #[test]
    fn test_week_merge_day_sets_start_end_and_extra_info() {
        let mut week = Week::new(1, 2026, 40.0);
        let day = Day::new(None);
        week.add_entry(day.clone());

        let start = Local
            .with_ymd_and_hms(2026, 1, 2, 9, 0, 0)
            .single()
            .unwrap();
        let end = start + Duration::hours(8);

        let mut modified_day = day.clone();
        modified_day.set_starting_time(Some(&start));
        modified_day.set_ending_time(Some(&end));
        modified_day.set_extra_info("Merged info".to_string());

        assert!(week.merge_day(&modified_day));

        let merged = &week.entries()[0];
        assert!(merged.start_time_set());
        assert!(merged.ending_time_set());
        assert_eq!(merged.extra_info(), "Merged info");
    }
}
