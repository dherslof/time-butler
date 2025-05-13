/*
 * File: cli.rs
 * Description: The definition of the Project struct
 * Author: dherslof
 * Created: 13-12-2024
 * License: MIT
 */

use serde::{Deserialize, Serialize};
use std::fmt;
use uuid::Uuid;

use crate::entry::Entry;

/// Represents a project with a name and description
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Project {
    /// Project name
    name: String,
    /// Project description
    description: Option<String>,
    /// Project entries
    entries: Vec<Entry>,
    /// Project ID
    id: uuid::Uuid,
}

/// Implementation for Project functionality
impl Project {
    /// Create a new project
    pub fn new(name: String, description: Option<String>) -> Self {
        Self {
            name,
            description,
            entries: Vec::new(),
            id: Uuid::new_v4(),
        }
    }

    /// Getter for `name`
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Getter for `description`
    pub fn description(&self) -> Option<&str> {
        self.description.as_deref()
    }

    /// Getter for `entries`
    pub fn entries(&self) -> &Vec<Entry> {
        &self.entries
    }

    /// Add a new entry to the project
    pub fn add_entry(&mut self, entry: Entry) {
        self.entries.push(entry);
    }

    /// Getter for `id`
    pub fn id(&self) -> &Uuid {
        &self.id
    }

    /// Remove an entry from the project
    pub fn remove_listed_entry(&mut self, entry_id: &Uuid) -> bool {
        if self.entry_exists(entry_id) {
            self.entries.retain(|e| e.id() != entry_id);
            return true;
        } else {
            return false;
        }
    }

    /// Check if entry exists in the project
    pub fn entry_exists(&self, entry_id: &Uuid) -> bool {
        for e in &self.entries {
            if e.id() == entry_id {
                return true;
            }
        }
        return false;
    }

    /// Return a copy of an entry
    pub fn get_entry_copy(&self, entry_id: &Uuid) -> Option<Entry> {
        for e in &self.entries {
            if e.id() == entry_id {
                return Some(e.clone());
            }
        }
        return None;
    }
}

/// Implement Display for Project
impl fmt::Display for Project {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Name: {}\nDescription: - {:?}\n Entries: {}\n, ID: {}\n",
            self.name,
            self.description(),
            self.entries.len(),
            self.id
        )
    }
}
