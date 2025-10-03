/*
 * File: config.rs
 * Description: The application configuration container.
 * Author: dherslof
 * Created: 22-09-2025
 * License: MIT
 */

use serde::{Deserialize, Serialize};

/// Application configuration struct
#[derive(Serialize, Deserialize, Clone)]
pub struct AppConfiguration {
    pub file_paths: FilePathsConfig,
}

impl AppConfiguration {
    pub fn storage_directory(&self) -> String {
        self.file_paths.storage_directory.clone()
    }

    pub fn project_data_path(&self) -> String {
        self.file_paths.project_data_path.clone()
    }

    pub fn week_data_path(&self) -> String {
        self.file_paths.week_data_path.clone()
    }

    pub fn report_directory(&self) -> String {
        self.file_paths.report_directory.clone()
    }
}

impl AppConfiguration {
    /// Create a default configuration, but with a custom storage directory.
    pub fn new_default(user_home: &String) -> Self {
        let file_paths = FilePathsConfig {
            storage_directory: format!("{}/.local/time-butler", user_home),
            project_data_path: format!(
                "{}/.local/time-butler/.app_storage/prj_data.bin",
                user_home
            ),
            week_data_path: format!(
                "{}/.local/time-butler/.app_storage/week_data.bin",
                user_home
            ),
            report_directory: format!("{}/.local/time-butler/.generated_reports", user_home),
        };
        Self { file_paths }
    }
}

/// Fs configuration struct
#[derive(Serialize, Deserialize, Clone)]
pub struct FilePathsConfig {
    #[serde(rename = "time-butler-storage-directory")]
    pub storage_directory: String,
    #[serde(rename = "time-butler-project-data-path")]
    pub project_data_path: String,
    #[serde(rename = "time-butler-week-data-path")]
    pub week_data_path: String,
    #[serde(rename = "time-butler-report-generation-directory")]
    pub report_directory: String,
}
