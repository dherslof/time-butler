/*
 * File: storage_handler.rs
 * Description: The storage handler for the application. Handles storage and retrieval of data
 * Author: dherslof
 * Created: 13-12-2024
 * License: MIT
 */

use std::env;
use std::fs;
use std::io;
use std::io::Read;
use std::io::Write;
use std::path::Path;

use bincode;

use crate::project::Project;
use crate::week::Week;

/// The storage handler struct
pub struct StorageHandler {
    /// Path to the project storage file
    default_project_data_file_path: String,
    /// Path to the week storage file
    default_week_data_file_path: String,
    /// Storage directory
    default_storage_dir: String,
    /// Report directory
    default_report_dir: String,
}

/// Implementation for StorageHandler functionality
impl StorageHandler {
    pub fn new() -> Self {
        Self {
            default_project_data_file_path: "/prj_data.bin".to_string(),
            default_week_data_file_path: "/week_data.bin".to_string(),
            default_storage_dir: "/.app_storage".to_string(),
            default_report_dir: "/generated_reports".to_string(),
        }
    }

    /// Load projects from storage
    pub fn load_projects(&self) -> Option<Vec<Project>> {
        // Load projects from storage
        let cwd = match env::current_dir() {
            Ok(cwd) => cwd,
            Err(e) => {
                tracing::error!("Error getting current directory: {}", e);
                return None;
            }
        };
        let prj_file = format!(
            "{}{}{}",
            cwd.display(),
            self.default_storage_dir,
            self.default_project_data_file_path
        );

        tracing::debug!("Loading projects from file: {}", prj_file);
        if !fs::metadata(&prj_file).is_ok() {
            tracing::error!(
                "File {} does not exist - OK if running for first time",
                prj_file
            );
            return None;
        }

        let mut file = fs::File::open(prj_file)
            .map_err(|e| io::Error::new(io::ErrorKind::Other, format!("Error opening file: {}", e)))
            .ok()?;

        let mut buffer = Vec::new();
        file.read_to_end(&mut buffer).ok()?;

        let projects = match bincode::deserialize(&buffer) {
            Ok(projects) => projects,
            Err(e) => {
                tracing::error!("Error deserializing data: {}", e);
                return None;
            }
        };

        return Some(projects);
    }

    /// Store projects to storage
    pub fn store_projects(&self, projects: Vec<Project>) -> io::Result<()> {
        // Store projects to storage
        let cwd = match env::current_dir() {
            Ok(cwd) => cwd,
            Err(e) => {
                tracing::error!("Error getting current directory: {}", e);
                return Err(e);
            }
        };

        let storage_dir = format!("{}{}", cwd.display(), self.default_storage_dir);
        if !fs::metadata(&storage_dir).is_ok() {
            tracing::debug!("Storage directory does not exist, creating it");
            self.create_storage_dir()?;
        }

        let prj_file = format!(
            "{}{}{}",
            cwd.display(),
            self.default_storage_dir,
            self.default_project_data_file_path
        );

        tracing::debug!("Storing projects to file: {}", prj_file);

        let serialized_data = bincode::serialize(&projects).map_err(|e| {
            io::Error::new(io::ErrorKind::Other, format!("Serialization error: {}", e))
        })?;
        let mut file = fs::File::create(prj_file)?;
        file.write_all(&serialized_data)?;

        Ok(())
    }

    /// Create the storage directory if not existing
    fn create_storage_dir(&self) -> io::Result<()> {
        let cwd = match env::current_dir() {
            Ok(cwd) => cwd,
            Err(e) => {
                tracing::error!("Error getting current directory: {}", e);
                return Err(e);
            }
        };
        let storage_dir = format!("{}{}", cwd.display(), self.default_storage_dir);

        tracing::debug!("Creating storage directory: {}", storage_dir);
        fs::create_dir_all(storage_dir)?;

        Ok(())
    }

    /// Create the report directory if not existing
    pub fn create_report_dir(&self) -> io::Result<()> {
        let cwd = match env::current_dir() {
            Ok(cwd) => cwd,
            Err(e) => {
                tracing::error!("Error getting current directory: {}", e);
                return Err(e);
            }
        };
        let report_dir = format!("{}{}", cwd.display(), self.default_report_dir);

        if Path::new(&report_dir).exists() && Path::new(&report_dir).is_dir() {
            tracing::debug!(
                "Report directory already exists: {}, no need to create new",
                report_dir
            );
            return Ok(());
        }

        tracing::debug!("Creating report directory: {}", report_dir);
        fs::create_dir_all(report_dir)?;

        Ok(())
    }

    /// Load weeks from storage
    pub fn load_weeks(&self) -> Option<Vec<Week>> {
        // Load projects from storage
        let cwd = match env::current_dir() {
            Ok(cwd) => cwd,
            Err(e) => {
                tracing::error!("Error getting current directory: {}", e);
                return None;
            }
        };
        let week_file = format!(
            "{}{}{}",
            cwd.display(),
            self.default_storage_dir,
            self.default_week_data_file_path
        );

        tracing::debug!("Loading weeks from file: {}", week_file);
        if !fs::metadata(&week_file).is_ok() {
            tracing::error!("File {} does not exist", week_file);
            return None;
        }

        let mut file = fs::File::open(week_file)
            .map_err(|e| io::Error::new(io::ErrorKind::Other, format!("Error opening file: {}", e)))
            .ok()?;

        let mut buffer = Vec::new();
        file.read_to_end(&mut buffer).ok()?;

        let weeks = match bincode::deserialize(&buffer) {
            Ok(weeks) => weeks,
            Err(e) => {
                tracing::error!("Error deserializing data: {}", e);
                return None;
            }
        };

        return Some(weeks);
    }

    /// Store weeks to storage
    pub fn store_weeks(&self, weeks: Vec<Week>) -> io::Result<()> {
        // Store weeks to storage
        let cwd = match env::current_dir() {
            Ok(cwd) => cwd,
            Err(e) => {
                tracing::error!("Error getting current directory: {}", e);
                return Err(e);
            }
        };

        let storage_dir = format!("{}{}", cwd.display(), self.default_storage_dir);
        if !fs::metadata(&storage_dir).is_ok() {
            tracing::debug!("Storage directory does not exist, creating it");
            self.create_storage_dir()?;
        }

        let week_file = format!(
            "{}{}{}",
            cwd.display(),
            self.default_storage_dir,
            self.default_week_data_file_path
        );

        tracing::debug!("Storing projects to file: {}", week_file);

        let serialized_data = bincode::serialize(&weeks).map_err(|e| {
            io::Error::new(io::ErrorKind::Other, format!("Serialization error: {}", e))
        })?;
        let mut file = fs::File::create(week_file)?;
        file.write_all(&serialized_data)?;

        Ok(())
    }
}
