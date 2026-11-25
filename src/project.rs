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
            true
        } else {
            false
        }
    }

    /// Check if entry exists in the project
    pub fn entry_exists(&self, entry_id: &Uuid) -> bool {
        for e in &self.entries {
            if e.id() == entry_id {
                return true;
            }
        }
        false
    }

    /// Return a copy of an entry
    pub fn get_entry_copy(&self, entry_id: &Uuid) -> Option<Entry> {
        for e in &self.entries {
            if e.id() == entry_id {
                return Some(e.clone());
            }
        }
        None
    }

    /// Update the project name
    pub fn update_name(&mut self, new_name: String) {
        self.name = new_name;
    }

    /// Update the project description
    pub fn update_description(&mut self, new_description: String) {
        self.description = Some(new_description);
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_project_creation_defaults() {
        let project = Project::new("Time Butler".to_string(), None);

        assert_eq!(project.name(), "Time Butler");
        assert_eq!(project.description(), None);
        assert!(project.entries().is_empty());
    }

    #[test]
    fn test_project_creation_with_description() {
        let project = Project::new("Time Butler".to_string(), Some("Tracker".to_string()));

        assert_eq!(project.description(), Some("Tracker"));
    }

    #[test]
    fn test_project_id_is_unique() {
        let p1 = Project::new("A".to_string(), None);
        let p2 = Project::new("B".to_string(), None);

        assert_ne!(p1.id(), p2.id());
    }

    #[test]
    fn test_project_add_entry() {
        let mut project = Project::new("Time Butler".to_string(), None);
        let entry = Entry::new(2.5, Some("Planning".to_string()));

        project.add_entry(entry.clone());

        assert_eq!(project.entries().len(), 1);
        assert_eq!(project.entries()[0].id(), entry.id());
    }

    #[test]
    fn test_project_entry_exists_true_after_add() {
        let mut project = Project::new("Time Butler".to_string(), None);
        let entry = Entry::new(1.0, None);
        let id = *entry.id();
        project.add_entry(entry);

        assert!(project.entry_exists(&id));
    }

    #[test]
    fn test_project_entry_exists_false_when_missing() {
        let project = Project::new("Time Butler".to_string(), None);
        let missing_id = Uuid::new_v4();

        assert!(!project.entry_exists(&missing_id));
    }

    #[test]
    fn test_project_get_entry_copy_returns_some_for_existing() {
        let mut project = Project::new("Time Butler".to_string(), None);
        let entry = Entry::new(3.0, Some("Implementation".to_string()));
        let id = *entry.id();
        project.add_entry(entry.clone());

        let copy = project.get_entry_copy(&id);

        assert!(copy.is_some());
        let copy = copy.unwrap();
        assert_eq!(copy.id(), entry.id());
        assert_eq!(copy.hours(), entry.hours());
        assert_eq!(copy.description(), entry.description());
    }

    #[test]
    fn test_project_get_entry_copy_returns_none_for_missing() {
        let project = Project::new("Time Butler".to_string(), None);
        let missing_id = Uuid::new_v4();

        assert!(project.get_entry_copy(&missing_id).is_none());
    }

    #[test]
    fn test_project_remove_listed_entry_success() {
        let mut project = Project::new("Time Butler".to_string(), None);
        let entry = Entry::new(4.0, Some("Refactor".to_string()));
        let id = *entry.id();
        project.add_entry(entry);

        assert!(project.remove_listed_entry(&id));
        assert!(!project.entry_exists(&id));
        assert!(project.entries().is_empty());
    }

    #[test]
    fn test_project_remove_listed_entry_nonexistent_returns_false() {
        let mut project = Project::new("Time Butler".to_string(), None);
        project.add_entry(Entry::new(1.0, None));
        let missing_id = Uuid::new_v4();

        assert!(!project.remove_listed_entry(&missing_id));
        assert_eq!(project.entries().len(), 1);
    }

    #[test]
    fn test_project_update_name() {
        let mut project = Project::new("Old Name".to_string(), None);

        project.update_name("New Name".to_string());

        assert_eq!(project.name(), "New Name");
    }

    #[test]
    fn test_project_update_description() {
        let mut project = Project::new("Time Butler".to_string(), None);

        project.update_description("Updated description".to_string());

        assert_eq!(project.description(), Some("Updated description"));
    }

    #[test]
    fn test_project_display_contains_key_fields() {
        let mut project = Project::new("My Project".to_string(), Some("Desc".to_string()));
        project.add_entry(Entry::new(1.5, Some("Task".to_string())));

        let rendered = format!("{}", project);

        assert!(rendered.contains("Name: My Project"));
        assert!(rendered.contains("Description:"));
        assert!(rendered.contains("Entries: 1"));
        assert!(rendered.contains("ID:"));
    }
}
