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
    hours: f32,
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
    pub fn new(hours: f32, description: Option<String>) -> Self {
        Self {
            hours,
            description,
            created: Local::now(),
            id: Uuid::new_v4(),
        }
    }

    /// Getter for `hours`
    pub fn hours(&self) -> f32 {
        self.hours
    }

    /// Getter for `description`
    pub fn description(&self) -> &str {
        let return_value = match &self.description {
            Some(value) => value.as_str(),
            None => "",
        };

        return_value
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_entry_hours() {
        let e = Entry::new(7.5, Some("Wrote tests".to_string()));
        assert_eq!(e.hours(), 7.5);
    }

    #[test]
    fn test_entry_description_none() {
        let e = Entry::new(4.0, None);
        assert_eq!(e.description(), "");
    }

    #[test]
    fn test_entry_description_some() {
        let e = Entry::new(4.0, Some("Worked on project".to_string()));
        assert_eq!(e.description(), "Worked on project");
    }

    #[test]
    fn test_entry_created() {
        let e = Entry::new(3.0, None);
        assert!(e.created() <= &Local::now());
    }

    #[test]
    fn test_entry_id() {
        let e1 = Entry::new(2.0, None);
        let e2 = Entry::new(2.0, None);
        assert_ne!(e1.id(), e2.id());
    }

    #[test]
    fn test_entry_display_contains_core_fields() {
        let e = Entry::new(6.0, Some("Pair programming".to_string()));
        let rendered = format!("{}", e);

        assert!(rendered.contains("Pair programming"));
        assert!(rendered.contains("hours - 6"));
        assert!(rendered.contains("Created:"));
    }

    #[test]
    fn test_entry_clone_keeps_values() {
        let e = Entry::new(8.0, Some("Feature work".to_string()));
        let c = e.clone();

        assert_eq!(c.hours(), e.hours());
        assert_eq!(c.description(), e.description());
        assert_eq!(c.created(), e.created());
        assert_eq!(c.id(), e.id());
    }

    #[test]
    fn test_entry_serde_round_trip() {
        let e = Entry::new(5.5, Some("Bug fixes".to_string()));

        let json = serde_json::to_string(&e).unwrap();
        let parsed: Entry = serde_json::from_str(&json).unwrap();

        assert_eq!(parsed.hours(), e.hours());
        assert_eq!(parsed.description(), e.description());
        assert_eq!(parsed.created(), e.created());
        assert_eq!(parsed.id(), e.id());
    }

    #[test]
    fn test_entry_hours_zero_is_supported() {
        let e = Entry::new(0.0, None);
        assert_eq!(e.hours(), 0.0);
    }

    #[test]
    fn test_entry_hours_negative_is_currently_allowed() {
        let e = Entry::new(-1.25, Some("Adjustment".to_string()));
        assert_eq!(e.hours(), -1.25);
    }
}
