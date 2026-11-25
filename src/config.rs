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
    version: VersionConfiguration,
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
        self.targets.week_target_hours
    }

    pub fn backup_directory(&self) -> String {
        self.file_paths.backups_directory.clone()
    }

    pub fn month_target_hours(&self) -> f32 {
        self.targets.month_target_hours
    }

    pub fn weekly_target_for_month(&self) -> bool {
        self.targets.weekly_target_for_month
    }

    pub fn periodic_backup_enabled(&self) -> bool {
        self.backup.enable_periodic_backup
    }

    pub fn periodic_backup_interval(&self) -> u32 {
        self.backup.periodic_backup_interval_days
    }

    pub fn override_existing_backup(&self) -> bool {
        self.backup.override_existing_backup
    }

    pub fn always_force_halt_on_version_incompatibility(&self) -> bool {
        self.version.always_force_halt_on_version_incompatibility
    }

    pub fn get_as_string(&self) -> String {
        // format complete config as a String in order to print/write it
        let mut out = String::new();
        out.push_str("Configuration:\n");
        out.push_str(&format!(
            "  time-butler-storage-directory: {}\n",
            self.file_paths.storage_directory
        ));
        out.push_str(&format!(
            "  time-butler-project-data-path: {}\n",
            self.file_paths.project_data_path
        ));
        out.push_str(&format!(
            "  time-butler-week-data-path: {}\n",
            self.file_paths.week_data_path
        ));
        out.push_str(&format!(
            "  time-butler-report-generation-directory: {}\n",
            self.file_paths.report_directory
        ));
        out.push_str(&format!(
            "  time-butler-backups-directory: {}\n",
            self.file_paths.backups_directory
        ));
        out.push_str(&format!(
            "  total-week-target: {}\n",
            self.targets.week_target_hours
        ));
        out.push_str(&format!(
            "  total-month-target: {}\n",
            self.targets.month_target_hours
        ));
        out.push_str(&format!(
            "  use-total-week-target-for-month: {}\n",
            self.targets.weekly_target_for_month
        ));
        out.push_str(&format!(
            "  enable-periodic-backup: {}\n",
            self.backup.enable_periodic_backup
        ));
        out.push_str(&format!(
            "  periodic-backup-days-interval: {}\n",
            self.backup.periodic_backup_interval_days
        ));
        out.push_str(&format!(
            "  periodic-backup-override: {}\n",
            self.backup.override_existing_backup
        ));
        out.push_str(&format!(
            "  always-force-halt-on-version-incompatibility: {}\n",
            self.version.always_force_halt_on_version_incompatibility
        ));
        out
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
            report_directory: format!("{}/.local/time-butler/generated-reports", user_home),
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
        let version = VersionConfiguration {
            always_force_halt_on_version_incompatibility: true,
        };
        Self {
            file_paths,
            targets,
            backup,
            version,
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

/// Version configuration struct
#[derive(Serialize, Deserialize, Clone)]
pub struct VersionConfiguration {
    /// Force halt on version incompatibility
    #[serde(rename = "always-force-halt-on-version-incompatibility")]
    pub always_force_halt_on_version_incompatibility: bool,
}

#[cfg(test)]
mod tests {
    use super::*;

    fn make_default() -> AppConfiguration {
        AppConfiguration::new_default(&"/home/testuser".to_string())
    }

    // --- new_default path construction ---

    #[test]
    fn test_new_default_storage_directory() {
        let cfg = make_default();
        assert_eq!(cfg.storage_directory(), "/home/testuser/.local/time-butler");
    }

    #[test]
    fn test_new_default_project_data_path() {
        let cfg = make_default();
        assert_eq!(
            cfg.project_data_path(),
            "/home/testuser/.local/time-butler/.app_storage/prj_data.bin"
        );
    }

    #[test]
    fn test_new_default_week_data_path() {
        let cfg = make_default();
        assert_eq!(
            cfg.week_data_path(),
            "/home/testuser/.local/time-butler/.app_storage/week_data.bin"
        );
    }

    #[test]
    fn test_new_default_report_directory() {
        let cfg = make_default();
        assert_eq!(
            cfg.report_directory(),
            "/home/testuser/.local/time-butler/generated-reports"
        );
    }

    #[test]
    fn test_new_default_backup_directory() {
        let cfg = make_default();
        assert_eq!(
            cfg.backup_directory(),
            "/home/testuser/.local/time-butler/backups"
        );
    }

    // --- default target values ---

    #[test]
    fn test_new_default_week_target_hours() {
        let cfg = make_default();
        assert_eq!(cfg.week_target_hours(), 40.0);
    }

    #[test]
    fn test_new_default_month_target_hours() {
        let cfg = make_default();
        assert_eq!(cfg.month_target_hours(), 160.0);
    }

    #[test]
    fn test_new_default_weekly_target_for_month_is_false() {
        let cfg = make_default();
        assert!(!cfg.weekly_target_for_month());
    }

    // --- default backup values ---

    #[test]
    fn test_new_default_periodic_backup_disabled() {
        let cfg = make_default();
        assert!(!cfg.periodic_backup_enabled());
    }

    #[test]
    fn test_new_default_periodic_backup_interval() {
        let cfg = make_default();
        assert_eq!(cfg.periodic_backup_interval(), 14);
    }

    #[test]
    fn test_new_default_override_existing_backup_is_true() {
        let cfg = make_default();
        assert!(cfg.override_existing_backup());
    }

    // --- default version values ---

    #[test]
    fn test_new_default_always_force_halt_on_version_incompatibility_is_true() {
        let cfg = make_default();
        assert!(cfg.always_force_halt_on_version_incompatibility());
    }

    // --- get_as_string ---

    #[test]
    fn test_get_as_string_contains_all_key_labels() {
        let s = make_default().get_as_string();

        assert!(s.contains("time-butler-storage-directory"));
        assert!(s.contains("time-butler-project-data-path"));
        assert!(s.contains("time-butler-week-data-path"));
        assert!(s.contains("time-butler-report-generation-directory"));
        assert!(s.contains("time-butler-backups-directory"));
        assert!(s.contains("total-week-target"));
        assert!(s.contains("total-month-target"));
        assert!(s.contains("use-total-week-target-for-month"));
        assert!(s.contains("enable-periodic-backup"));
        assert!(s.contains("periodic-backup-days-interval"));
        assert!(s.contains("periodic-backup-override"));
        assert!(s.contains("always-force-halt-on-version-incompatibility"));
    }

    #[test]
    fn test_get_as_string_contains_expected_values() {
        let s = make_default().get_as_string();
        assert!(s.contains("/home/testuser/.local/time-butler"));
        assert!(s.contains("40"));
        assert!(s.contains("160"));
        assert!(s.contains("14"));
    }

    // --- Clone ---

    #[test]
    fn test_clone_preserves_values() {
        let cfg = make_default();
        let cloned = cfg.clone();
        assert_eq!(cfg.week_target_hours(), cloned.week_target_hours());
        assert_eq!(cfg.storage_directory(), cloned.storage_directory());
        assert_eq!(
            cfg.periodic_backup_interval(),
            cloned.periodic_backup_interval()
        );
    }

    // --- Serde round-trip via JSON ---

    #[test]
    fn test_serde_round_trip() {
        let cfg = make_default();
        let json = serde_json::to_string(&cfg).unwrap();
        let parsed: AppConfiguration = serde_json::from_str(&json).unwrap();
        assert_eq!(cfg.week_target_hours(), parsed.week_target_hours());
        assert_eq!(cfg.storage_directory(), parsed.storage_directory());
        assert_eq!(cfg.backup_directory(), parsed.backup_directory());
    }
}
