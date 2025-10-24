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
}

impl Day {
    /// Create a new Day
    pub fn new(extra_info: Option<String>) -> Self {
        Self {
            starting_time: None,
            ending_time: None,
            hours: K_NO_HOURS,
            extra_info: extra_info.unwrap_or_else(|| "".to_string()),
            created: Local::now(),
            week: Utc::now().iso_week().week(),
            date: Local::now().date_naive(),
            start_time_set: false,
            ending_time_set: false,
            closed: false,
            hours_paused: K_NO_HOURS,
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

    /// Getter for `created`
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

    /// Setter for `starting_time`
    pub fn set_starting_time(&mut self, t: Option<&DateTime<Local>>) {
        match t {
            Some(value) => self.starting_time = Some(value.clone()),
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
        }
    }

    /// Setter for `ending_time`
    pub fn set_ending_time(&mut self, t: Option<&DateTime<Local>>) {
        match t {
            Some(value) => self.ending_time = Some(value.clone()),
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
