/*
 * File: backup.rs
 * Description: Functionality related to backup of files and data
 * Author: dherslof
 * Created: 07-10-2025
 * License: MIT
 */

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::fs;
use std::io::Write;
use std::path::Path;

const STATE_FILE: &str = "backup_state.bin";

#[derive(Serialize, Deserialize, Debug)]
struct BackupState {
    last_backup: DateTime<Utc>,
}

/// The backup organizer struct
pub struct BackupOrganizer {
    backup_dir: String,
    project_data_file: String,
    week_data_file: String,
    storage_dir: String,
}

impl BackupOrganizer {
    pub fn new(
        backup_dir: &str,
        project_data_file: &str,
        week_data_file: &str,
        storage_dir: &str,
    ) -> Self {
        Self {
            backup_dir: backup_dir.to_string(),
            project_data_file: project_data_file.to_string(),
            week_data_file: week_data_file.to_string(),
            storage_dir: storage_dir.to_string(),
        }
    }

    /// Main API for backing up data
    pub fn backup_data(
        &self,
        backup_enabled: bool,
        override_existing: bool,
        duration_days: u32,
        update_state: Option<bool>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        if !backup_enabled {
            tracing::debug!("Backups not enabled. No backups will be created");
            return Ok(());
        }

        tracing::debug!("Backups enabled, loading state");
        let mut do_backup = false;
        match self.load_state(duration_days) {
            Ok(do_new_backup) => {
                do_backup = do_new_backup;
            }
            Err(e) => {
                // Check if it's a NotFound error
                if let Some(io_err) = e.downcast_ref::<std::io::Error>() {
                    if io_err.kind() == std::io::ErrorKind::NotFound {
                        // Handle missing state file specifically
                        tracing::warn!("State file not found. Assuming no backup done previously.");
                        do_backup = true;
                    } else {
                        // Assume error and return
                        tracing::error!("Failed to load backup state: {}", e);
                        return Err(e);
                    }
                }
            }
        }

        if !do_backup {
            tracing::debug!("Backup interval has not passed. No backup will be done");
            return Ok(());
        }

        tracing::info!("Backup interval has passed. Starting to back up files");

        // Ensure backup directory exists before proceeding
        self.init_backup_directory()?;

        // Do the actual backups
        self.do_backup(override_existing)?;

        let update_new_state = update_state.unwrap_or(true);
        if update_new_state {
            self.save_state()?;
        }

        return Ok(());
    }

    /// Create the backup directory if it does not exist
    fn init_backup_directory(&self) -> Result<(), Box<dyn std::error::Error>> {
        let path = Path::new(&self.backup_dir);
        if !path.exists() {
            tracing::debug!(
                "Backup directory: {} do not exists. Creating it.",
                self.backup_dir
            );
            fs::create_dir_all(path)?;
        }
        Ok(())
    }

    /// Checks if a file exists at the given path
    fn file_exists(path: &String) -> bool {
        Path::new(path).is_file()
    }

    /// Get existing backups, in order to remove if wanted
    fn get_existing_backup_files(&self, backup_dir: &str, backup_file: &str) -> Vec<String> {
        let mut matches = Vec::new();
        if let Ok(entries) = std::fs::read_dir(backup_dir) {
            for entry in entries.flatten() {
                if let Some(file_name) = entry.file_name().to_str() {
                    if file_name.contains(backup_file) {
                        matches.push(file_name.to_string());
                    }
                }
            }
        }
        matches
    }

    /// Load last known backup state
    fn load_state(&self, duration_days: u32) -> Result<bool, Box<dyn std::error::Error>> {
        let state_file = format!("{}/{}", self.storage_dir, STATE_FILE);
        if !Self::file_exists(&state_file) {
            return Err(Box::new(std::io::Error::new(
                std::io::ErrorKind::NotFound,
                format!("State file does not exist: {}", state_file),
            )));
        }

        // Read and deserialize the state file
        let data = std::fs::read(&state_file)?;
        let backup_state: BackupState = bincode::deserialize(&data)?;

        // Calculate the duration since last backup
        let now = Utc::now();
        let duration = now.signed_duration_since(backup_state.last_backup);

        if duration.num_days() >= duration_days as i64 {
            Ok(true)
        } else {
            Ok(false)
        }
    }

    /// Save current backup state
    fn save_state(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let state_file_path = format!("{}/{}", self.storage_dir, STATE_FILE);
        let backup_state = BackupState {
            last_backup: Utc::now(),
        };

        // Serialize the BackupState struct to binary using bincode
        let encoded: Vec<u8> = bincode::serialize(&backup_state)?;

        let mut file = fs::File::create(&state_file_path)?;
        file.write_all(&encoded)?;

        tracing::debug!("Backup state saved to {}", state_file_path);
        Ok(true)
    }

    /// Do the actual backup
    fn do_backup(&self, override_existing: bool) -> Result<(), Box<dyn std::error::Error>> {
        let current_date = Utc::now();

        let prj_data_backup_file = format!(
            "{}/prj_data_{}.bin",
            self.backup_dir,
            current_date.format("%Y%m%d")
        );

        let existing_prj_files = self.get_existing_backup_files(&self.backup_dir, "prj_data");
        if !existing_prj_files.is_empty() {
            tracing::debug!("Found existing backup files: {:?}", existing_prj_files);
            if override_existing {
                tracing::info!("Overriding existing backup files");
                for file_name in existing_prj_files {
                    let full_path = format!("{}/{}", self.backup_dir, file_name);
                    if let Err(e) = std::fs::remove_file(&full_path) {
                        tracing::error!("Failed to remove file {}: {}", full_path, e);
                    }
                }
            }
        } else {
            tracing::debug!("No existing backup file found.");
        }

        fs::copy(&self.project_data_file, &prj_data_backup_file)?;
        tracing::info!("Backed up project data to {}", prj_data_backup_file);

        let week_data_backup_file = format!(
            "{}/week_data_{}.bin",
            self.backup_dir,
            current_date.format("%Y%m%d")
        );

        let existing_week_files = self.get_existing_backup_files(&self.backup_dir, "prj_data");
        if !existing_week_files.is_empty() {
            tracing::debug!("Found existing backup files: {:?}", existing_week_files);
            if override_existing {
                tracing::info!("Overriding existing backup files");
                for file_name in existing_week_files {
                    let full_path = format!("{}/{}", self.backup_dir, file_name);
                    if let Err(e) = std::fs::remove_file(&full_path) {
                        tracing::error!("Failed to remove file {}: {}", full_path, e);
                    }
                }
            }
        } else {
            tracing::debug!("No existing backup file found.");
        }

        fs::copy(&self.week_data_file, &week_data_backup_file)?;
        tracing::info!("Backed up week data to {}", week_data_backup_file);

        Ok(())
    }
}
