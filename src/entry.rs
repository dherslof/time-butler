/*
 * File: entry.rs
 * Description: The definition of the Entry struct. Contains the time entries for projects
 * Author: dherslof
 * Created: 13-12-2024
 * License: MIT
 */

use chrono::{DateTime, Local};
use serde::{Deserialize, Serialize};
use std::fmt;
use uuid::Uuid;

/// Entry struct to store time entries
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Entry {
    /// Hours logged
    hours: u32,
    /// Description of the work done
    description: Option<String>,
    /// Timestamp of when the entry was created
    created: DateTime<Local>,
    /// Unique ID for the entry
    id: uuid::Uuid,
}

/// Implementation for Entry functionality
impl Entry {
    /// Create a new Entry
    pub fn new(hours: u32, description: Option<String>) -> Self {
        Self {
            hours,
            description,
            created: Local::now(),
            id: Uuid::new_v4(),
        }
    }

    /// Getter for `hours`
    pub fn hours(&self) -> u32 {
        self.hours
    }

    /// Getter for `description`
    pub fn description(&self) -> &str {
        let return_value = match &self.description {
            Some(value) => value.as_str(),
            None => "",
        };

        return return_value;
    }

    /// Getter for `created`
    pub fn created(&self) -> &DateTime<Local> {
        &self.created
    }

    /// Getter for `id`
    pub fn id(&self) -> &uuid::Uuid {
        &self.id
    }
}

/// Implement Display for Entry
impl fmt::Display for Entry {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Description: {:?}\nhours - {}\n (Created: {})",
            self.description, self.hours, self.created
        )
    }
}
