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
        return false;
    }

    /// Overloaded function. Check if a day exists in the week based on NaiveDate
    pub fn exist(&self, date: &NaiveDate) -> bool {
        for e in &self.entries {
            if e.date() == *date {
                return true;
            }
        }
        return false;
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

            return true;
        } else {
            // Day not found
            return false;
        }
    }

    /// Return a copy of an Day
    pub fn get_day_copy(&self, date: &NaiveDate) -> Option<Day> {
        for d in &self.entries {
            if d.date() == *date {
                return Some(d.clone());
            }
        }
        return None;
    }

    /// Remove Day from the week
    pub fn remove_listed_day(&mut self, date: &NaiveDate) -> bool {
        if self.exist(date) {
            self.entries.retain(|d| d.date() != *date);
            return true;
        } else {
            return false;
        }
    }
}
