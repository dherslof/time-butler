/*
 * File: cli.rs
 * Description: The definition of the time container Day.
 * Author: dherslof
 * Created: 13-12-2024
 * License: MIT
 */

use chrono::{DateTime, Datelike, Local, NaiveDate, Utc};
use serde::{Deserialize, Serialize};
use std::fmt;
use uuid::Uuid;

const K_WORK_HOURS_DEFAULT: f32 = 8.0;
const K_NO_HOURS: f32 = 0.0;

/// Day struct to store time entries
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Day {
    /// Start time of the day
    starting_time: Option<DateTime<Local>>,
    /// End time of the day
    ending_time: Option<DateTime<Local>>,
    /// Hours logged
    hours: f32,
    /// extra_info of the work done
    extra_info: String,
    /// Timestamp of when the Day was created
    created: DateTime<Local>,
    /// Week number
    week: u32,
    /// Date of the day
    date: NaiveDate,
    // Flag to show if start_time is set
    start_time_set: bool,
    // Flag to show if ending_time is set
    ending_time_set: bool,
    // Flag to show if the day is closed
    closed: bool,
    // Time duration as paused
    hours_paused: f32,
    /// Unique ID for the day
    id: uuid::Uuid,
}

impl Day {
    /// Create a new Day
    pub fn new(extra_info: Option<String>) -> Self {
        Self {
            starting_time: None,
            ending_time: None,
            hours: K_NO_HOURS,
            extra_info: extra_info.unwrap_or_default(),
            created: Local::now(),
            week: Utc::now().iso_week().week(),
            date: Local::now().date_naive(),
            start_time_set: false,
            ending_time_set: false,
            closed: false,
            hours_paused: K_NO_HOURS,
            id: Uuid::new_v4(),
        }
    }

    /// Getter for `hours`
    pub fn hours(&self) -> f32 {
        self.hours
    }

    /// Getter for `extra_info`
    pub fn extra_info(&self) -> &str {
        &self.extra_info
    }

    /// Setter for `extra_info`
    pub fn set_extra_info(&mut self, info: String) {
        self.extra_info = info;
    }

    // /// Getter for `created`
    /* Unused, remove comment or remove function if not needed later
        pub fn created(&self) -> &DateTime<Local> {
            &self.created
        }
    */

    /// Getter for `starting_time`
    pub fn starting_time(&self) -> Option<&DateTime<Local>> {
        self.starting_time.as_ref()
    }

    /// Getter for `ending_time`
    pub fn ending_time(&self) -> Option<&DateTime<Local>> {
        self.ending_time.as_ref()
    }

    /// Getter for houres paused
    pub fn hours_paused(&self) -> f32 {
        self.hours_paused
    }

    // Getter for `id`
    pub fn id(&self) -> &uuid::Uuid {
        &self.id
    }

    /// Setter for `starting_time`
    pub fn set_starting_time(&mut self, t: Option<&DateTime<Local>>) {
        match t {
            Some(value) => self.starting_time = Some(*value),
            None => {
                self.starting_time = Some(Local::now());
                tracing::debug!("No starting time provided, setting current time");
            }
        }

        tracing::debug!(
            "Setting starting time for the day: {:?}",
            self.starting_time
        );
        self.start_time_set = true;

        if self.start_time_set && self.ending_time_set {
            self.closed = true;

            self.hours = self.calculate_hours();

            if self.hours < K_WORK_HOURS_DEFAULT {
                println!("WARNING: You have worked less than 8 hours today.");
            }
            self.update_date_and_week_();
        }
    }

    /// Setter for `ending_time`
    pub fn set_ending_time(&mut self, t: Option<&DateTime<Local>>) {
        match t {
            Some(value) => self.ending_time = Some(*value),
            None => {
                self.ending_time = Some(Local::now());
                tracing::debug!("No ending time provided, setting current time");
            }
        }

        tracing::debug!("Setting ending time for the day: {:?}", self.ending_time);
        self.ending_time_set = true;

        if self.start_time_set && self.ending_time_set {
            self.closed = true;

            self.hours = self.calculate_hours();

            if self.hours < K_WORK_HOURS_DEFAULT {
                println!("WARNING: You have worked less than 8 hours today.");
            }
        }
    }

    /// Set paused time
    pub fn set_paused_time(&mut self, paused: f32) {
        if self.start_time_set {
            self.hours_paused = paused;
        } else {
            tracing::warn!("Cannot set paused time before starting time is set. Provided paused time will be ignored");
        }

        // If used during modification, recalculate hours if both times are set
        if self.start_time_set && self.ending_time_set {
            self.hours = self.calculate_hours();
        }
    }

    /// Getter for `week`
    pub fn week(&self) -> u32 {
        self.week
    }

    /// Getter for `date`
    pub fn date(&self) -> NaiveDate {
        self.date
    }

    /// Getter for `month`
    pub fn month(&self) -> u32 {
        self.date.month()
    }

    /// Getter for `year`
    pub fn year(&self) -> i32 {
        self.date.year()
    }

    /// Getter for `start_time_set`
    pub fn start_time_set(&self) -> bool {
        self.start_time_set
    }

    /// Getter for `ending_time_set`
    pub fn ending_time_set(&self) -> bool {
        self.ending_time_set
    }

    /// Getter for `closed`
    pub fn closed(&self) -> bool {
        self.closed
    }

    /// Calculate the hours worked
    fn calculate_hours(&self) -> f32 {
        //TODO: Refactor this function to not cut minutes so hard
        let duration = match (self.ending_time, self.starting_time) {
            (Some(end), Some(start)) => end - start,
            _ => {
                tracing::warn!("Cannot compute duration as one of the times is None.");
                chrono::Duration::zero()
            }
        };

        tracing::debug!("Calculated duration: {:?}", duration);

        // Get the duration as minutes, and then convert to hours on order to not round away the minutes
        let worked_hours = duration.num_minutes() as f32 / 60.0;

        let net_hours = worked_hours - self.hours_paused;

        // Prevent negative hours
        if net_hours < 0.0 {
            tracing::warn!("Worked hours (net hours) calculated as negative, [paused time = {}h]. Paused hours will not be considered", self.hours_paused);
            tracing::debug!("Setting worked hours to: {}", worked_hours);
            worked_hours
        } else {
            net_hours
        }
    }

    fn update_date_and_week_(&mut self) {
        // Will be used when modifying a day, to update the date and week if the starting time is modified.
        if let Some(starting_time) = self.starting_time {
            let updated_date = starting_time.date_naive();
            let updated_week = starting_time.iso_week().week();
            tracing::debug!(
                "Updating date and week based on starting time. New date: {}, New week: {}",
                updated_date,
                updated_week
            );
            if self.date != updated_date {
                self.date = updated_date;
            }
            if self.week != updated_week {
                self.week = updated_week;
            }
        } else {
            tracing::debug!("Cannot update date and week as starting time is not set.");
        }
    }
}

/// Implement Display for Day, intended for printing (debugging) purposes
impl fmt::Display for Day {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Created: {}\nstarting_time - {:?}\nend_time: {:?}\nhours - {}",
            self.created, self.starting_time, self.ending_time, self.hours
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::{Duration, Local, TimeZone};

    #[test]
    fn test_day_creation_defaults() {
        let day = Day::new(None);

        assert_eq!(day.hours(), 0.0);
        assert_eq!(day.extra_info(), "");
        assert!(day.starting_time().is_none());
        assert!(day.ending_time().is_none());
        assert!(!day.start_time_set());
        assert!(!day.ending_time_set());
        assert!(!day.closed());
        assert_eq!(day.hours_paused(), 0.0);
    }

    #[test]
    fn test_day_creation_with_extra_info() {
        let day = Day::new(Some("Client work".to_string()));
        assert_eq!(day.extra_info(), "Client work");
    }

    #[test]
    fn test_day_set_extra_info() {
        let mut day = Day::new(None);
        day.set_extra_info("Updated".to_string());
        assert_eq!(day.extra_info(), "Updated");
    }

    #[test]
    fn test_day_set_starting_time_updates_flags_and_value() {
        let mut day = Day::new(None);
        let start = Local
            .with_ymd_and_hms(2026, 1, 15, 9, 0, 0)
            .single()
            .unwrap();

        day.set_starting_time(Some(&start));

        assert!(day.start_time_set());
        assert_eq!(day.starting_time(), Some(&start));
        assert!(!day.closed());
    }

    #[test]
    fn test_day_set_ending_time_updates_flags_and_value() {
        let mut day = Day::new(None);
        let end = Local
            .with_ymd_and_hms(2026, 1, 15, 17, 0, 0)
            .single()
            .unwrap();

        day.set_ending_time(Some(&end));

        assert!(day.ending_time_set());
        assert_eq!(day.ending_time(), Some(&end));
        assert!(!day.closed());
    }

    #[test]
    fn test_day_closes_and_calculates_hours_when_both_times_are_set() {
        let mut day = Day::new(None);
        let start = Local
            .with_ymd_and_hms(2026, 1, 15, 9, 0, 0)
            .single()
            .unwrap();
        let end = start + Duration::hours(8);

        day.set_starting_time(Some(&start));
        day.set_ending_time(Some(&end));

        assert!(day.closed());
        assert_eq!(day.hours(), 8.0);
    }

    #[test]
    fn test_day_set_paused_time_before_start_is_ignored() {
        let mut day = Day::new(None);

        day.set_paused_time(2.0);

        assert_eq!(day.hours_paused(), 0.0);
    }

    #[test]
    fn test_day_set_paused_time_after_close_recalculates_hours() {
        let mut day = Day::new(None);
        let start = Local
            .with_ymd_and_hms(2026, 1, 15, 9, 0, 0)
            .single()
            .unwrap();
        let end = start + Duration::hours(9);

        day.set_starting_time(Some(&start));
        day.set_ending_time(Some(&end));
        day.set_paused_time(1.0);

        assert_eq!(day.hours_paused(), 1.0);
        assert_eq!(day.hours(), 8.0);
    }

    #[test]
    fn test_day_paused_time_greater_than_worked_hours_falls_back_to_worked_hours() {
        let mut day = Day::new(None);
        let start = Local
            .with_ymd_and_hms(2026, 1, 15, 9, 0, 0)
            .single()
            .unwrap();
        let end = start + Duration::hours(1);

        day.set_starting_time(Some(&start));
        day.set_ending_time(Some(&end));
        day.set_paused_time(2.0);

        assert_eq!(day.hours(), 1.0);
    }

    #[test]
    fn test_day_updates_date_and_week_when_starting_time_is_set_last() {
        let mut day = Day::new(None);
        let start = Local
            .with_ymd_and_hms(2026, 1, 16, 9, 0, 0)
            .single()
            .unwrap();
        let end = start + Duration::hours(8);

        day.set_ending_time(Some(&end));
        day.set_starting_time(Some(&start));

        assert_eq!(day.date(), start.date_naive());
        assert_eq!(day.week(), start.iso_week().week());
    }

    #[test]
    fn test_day_display_contains_key_fields() {
        let day = Day::new(Some("Display test".to_string()));
        let rendered = format!("{}", day);

        assert!(rendered.contains("Created:"));
        assert!(rendered.contains("starting_time"));
        assert!(rendered.contains("hours -"));
    }
}
