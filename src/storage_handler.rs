/*
 * File: storage_handler.rs
 * Description: The storage handler for the application. Handles storage and retrieval of data
 * Author: dherslof
 * Created: 13-12-2024
 * License: MIT
 */

use std::fs;
use std::io;
use std::io::Read;
use std::io::Write;
use std::path::Path;

use bincode;

use crate::backup_organizer::BackupOrganizer;
use crate::config::AppConfiguration;
use crate::project::Project;
use crate::week::Week;

// Constants for base paths
const BASE_PATH: &str = ".local/time-butler";
const STORAGE_DIR: &str = ".app_storage";
const REPORT_DIR: &str = ".generated_reports";
const PROJECT_DATA_FILE: &str = "prj_data.bin";
const WEEK_DATA_FILE: &str = "week_data.bin";
const BACKUP_DIR: &str = "backups";

/// The storage handler struct
pub struct StorageHandler {
    /// Path to the project storage file
    project_data_file_path: String,
    /// Path to the week storage file
    week_data_file_path: String,
    /// Storage directory
    storage_dir: String,
    /// Report directory
    report_dir: String,
    /// User home directory
    user_home_dir: String,
    /// Flag to indicate if a successful init has been done
    init_success: bool,
    /// Flag to indicate if this is the first run
    first_run: bool,
    backup_organizer: BackupOrganizer,
}

/// Implementation for StorageHandler functionality
impl StorageHandler {
    pub fn new() -> Self {
        let mut instance = Self {
            project_data_file_path: PROJECT_DATA_FILE.to_string(),
            week_data_file_path: WEEK_DATA_FILE.to_string(),
            storage_dir: STORAGE_DIR.to_string(),
            report_dir: REPORT_DIR.to_string(),
            user_home_dir: String::new(),
            init_success: false,
            first_run: false,
            backup_organizer: BackupOrganizer::new("", "", "", ""), // dummy
        };
        instance.init();

        // Now the correct paths are set
        instance.backup_organizer = BackupOrganizer::new(
            format!("{}/{}", &instance.storage_dir, BACKUP_DIR).as_str(),
            &instance.project_data_file_path,
            &instance.week_data_file_path,
            &instance.storage_dir,
        );
        instance
    }

    /// Load projects from storage
    pub fn load_projects(&self) -> Option<Vec<Project>> {
        if self.init_success == false {
            tracing::error!("Storage handler not initialized correctly, unable to load projects");
            return None;
        }

        tracing::debug!(
            "Loading projects from file: {}",
            self.project_data_file_path
        );
        let prj_file = self.project_data_file_path.clone();
        if !fs::metadata(&prj_file).is_ok() {
            tracing::error!(
                "File {} does not exist - OK if running for first time or no projects stored",
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
        if self.init_success == false {
            tracing::error!("Storage handler not initialized correctly, unable to store projects");
            return Err(io::Error::new(
                io::ErrorKind::Other,
                "Storage handler not initialized correctly",
            ));
        }

        let prj_file = self.project_data_file_path.clone();
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
        let storage_dir = self.storage_dir.clone();

        tracing::debug!("Creating storage directory: {}", storage_dir);
        fs::create_dir_all(storage_dir)?;

        Ok(())
    }

    /// Create the report directory if not existing
    pub fn create_report_dir(&self) -> io::Result<()> {
        let report_dir = self.report_dir.clone();
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
        if self.init_success == false {
            tracing::error!("Storage handler not initialized correctly, unable to load weeks");
            return None;
        }
        let week_file = self.week_data_file_path.clone();
        tracing::debug!("Loading weeks from file: {}", week_file);
        if !fs::metadata(&week_file).is_ok() {
            tracing::error!(
                "File {} does not exist - OK if running for first time or no weeks stored",
                week_file
            );
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
        if self.init_success == false {
            tracing::error!("Storage handler not initialized correctly, unable to store weeks");
            return Err(io::Error::new(
                io::ErrorKind::Other,
                "Storage handler not initialized correctly",
            ));
        }
        let storage_dir = self.storage_dir.clone();
        if !fs::metadata(&storage_dir).is_ok() {
            tracing::debug!("Storage directory does not exist, creating it");
            self.create_storage_dir()?;
        }

        let week_file = self.week_data_file_path.clone();
        tracing::debug!("Storing projects to file: {}", week_file);

        let serialized_data = bincode::serialize(&weeks).map_err(|e| {
            io::Error::new(io::ErrorKind::Other, format!("Serialization error: {}", e))
        })?;
        let mut file = fs::File::create(week_file)?;
        file.write_all(&serialized_data)?;

        Ok(())
    }

    fn init(&mut self) {
        tracing::debug!("Initializing storage handler");
        if cfg!(not(target_os = "linux")) {
            tracing::error!("Time-butler is only supported on Linux");
        }

        // Todo: Check if a configuration file exists, if not set default paths
        if let Some(home_dir) = dirs::home_dir() {
            // Set struct default paths
            self.project_data_file_path = format!(
                "{}/{}/{}/{}",
                home_dir.display(),
                BASE_PATH,
                self.storage_dir,
                self.project_data_file_path
            );
            self.week_data_file_path = format!(
                "{}/{}/{}/{}",
                home_dir.display(),
                BASE_PATH,
                self.storage_dir,
                self.week_data_file_path
            );
            self.storage_dir = format!("{}/{}/{}", home_dir.display(), BASE_PATH, self.storage_dir);

            // Check if this is the first run
            if !fs::metadata(&self.storage_dir).is_ok() {
                tracing::info!("First run detected, creating storage directory");
                self.first_run = true;
            } else {
                tracing::debug!("Storage directory already exists, not first run");
            }

            if self.first_run {
                // Create the storage directory
                fs::create_dir_all(&self.storage_dir).expect("Failed to create storage directory");
            }

            self.report_dir = format!("{}/{}/{}", home_dir.display(), BASE_PATH, self.report_dir);

            self.user_home_dir = format!("{}", home_dir.display());

            tracing::debug!("Default paths set");
            self.init_success = true;
        } else {
            tracing::error!("Could not find home directory");
        }
    }

    pub fn startup_storage_directory(&self) -> String {
        self.storage_dir.clone()
    }

    pub fn user_home_directory(&self) -> String {
        self.user_home_dir.clone()
    }

    pub fn set_paths_from_config(&mut self, config: &AppConfiguration) {
        tracing::debug!("Setting storage paths from configuration");
        tracing::debug!("Storage directory: {}", config.storage_directory());
        self.storage_dir = config.storage_directory();

        tracing::debug!("Project data path: {}", config.project_data_path());
        self.project_data_file_path = config.project_data_path();
        tracing::debug!("Week data path: {}", config.week_data_path());
        self.week_data_file_path = config.week_data_path();
        tracing::debug!("Report directory: {}", config.report_directory());
        self.report_dir = config.report_directory();

        self.backup_organizer = BackupOrganizer::new(
            config.backup_directory().as_str(),
            self.project_data_file_path.as_str(),
            self.week_data_file_path.as_str(),
            self.storage_dir.as_str(),
        );
    }

    pub fn backup_storage_files(
        &self,
        backup_enabled: bool,
        override_existing_backup: bool,
        backup_duration_interval: u32,
    ) {
        match self.backup_organizer.backup_data(
            backup_enabled,
            override_existing_backup,
            backup_duration_interval,
            Some(true),
        ) {
            Ok(_) => {
                tracing::debug!("Backup completed successfully.");
            }
            Err(e) => {
                tracing::error!("Backup failed: {}", e);
            }
        }
    }

    pub fn do_backup_now(&self) -> Result<(), Box<dyn std::error::Error>> {
        // You can adjust these arguments as needed, or make them configurable
        let backup_enabled = true;
        let override_existing_backup = false;
        let backup_duration_interval = 0;

        self.backup_organizer.backup_data(
            backup_enabled,
            override_existing_backup,
            backup_duration_interval,
            Some(false),
        )
    }
}
