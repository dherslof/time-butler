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
    file_paths: FilePathsConfig,
    targets: TargetsConfig,
    backup: BackupConfig,
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

    pub fn week_target_hours(&self) -> f32 {
        self.targets.week_target_hours.clone()
    }

    pub fn backup_directory(&self) -> String {
        self.file_paths.backups_directory.clone()
    }

    pub fn month_target_hours(&self) -> f32 {
        self.targets.month_target_hours.clone()
    }

    pub fn weekly_target_for_month(&self) -> bool {
        self.targets.weekly_target_for_month.clone()
    }

    pub fn periodic_backup_enabled(&self) -> bool {
        self.backup.enable_periodic_backup.clone()
    }

    pub fn periodic_backup_interval(&self) -> u32 {
        self.backup.periodic_backup_interval_days.clone()
    }

    pub fn override_existing_backup(&self) -> bool {
        self.backup.override_existing_backup.clone()
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
            report_directory: format!("{}/.local/time-butler/generated_reports", user_home),
            backups_directory: format!("{}/.local/time-butler/backups", user_home),
        };
        let targets = TargetsConfig {
            week_target_hours: 40.0,
            month_target_hours: 160.0,
            weekly_target_for_month: false,
        };
        let backup = BackupConfig {
            enable_periodic_backup: false,
            periodic_backup_interval_days: 14,
            override_existing_backup: true,
        };
        Self {
            file_paths,
            targets,
            backup,
        }
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
    #[serde(rename = "time-butler-backups-directory")]
    pub backups_directory: String,
}
/// Targets configuration struct
#[derive(Serialize, Deserialize, Clone)]
pub struct TargetsConfig {
    /// Target hours for the week
    #[serde(rename = "total-week-target")]
    pub week_target_hours: f32,
    /// Target hours for the month
    #[serde(rename = "total-month-target")]
    pub month_target_hours: f32,
    /// Use the combined weekly target for the month
    #[serde(rename = "use-total-week-target-for-month")]
    pub weekly_target_for_month: bool,
}

/// Backup configuration struct
#[derive(Serialize, Deserialize, Clone)]
pub struct BackupConfig {
    /// Target hours for the week
    #[serde(rename = "enable-periodic-backup")]
    pub enable_periodic_backup: bool,
    /// Target hours for the week
    #[serde(rename = "periodic-backup-days-interval")]
    pub periodic_backup_interval_days: u32,
    /// Target hours for the week
    #[serde(rename = "periodic-backup-override")]
    pub override_existing_backup: bool,
}
