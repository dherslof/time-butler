/*
 * File: version_info.rs
 * Description: The definition of the version information for the time-butler.
 * Author: dherslof
 * Created: 18-05-2026
 * License: MIT
 */

use serde::{Deserialize, Serialize};

/*
   Note: Time-butler has two types of versioning:
   1. Application Version: This is the version of the time-butler application itself, which is defined in the Cargo.toml file. It follows semantic versioning
   2. Storage File Version: This is the version of the storage file format. It is derived from the application version, but only includes the major version number (the first part of the semantic version). This allows for backward compatibility in storage file formats, as long as there are no breaking changes in the major version.
*/

/// Struct to hold version information
#[derive(Debug, Clone)]
pub struct VersionInfo {
    /// The version number of the application
    version: String,
    /// The version of the storage file format
    storage_file_version: String,
}

impl VersionInfo {
    /// Create a new VersionInfo instance with the current version info from the toml file
    pub fn new() -> Self {
        VersionInfo {
            version: env!("CARGO_PKG_VERSION").to_string(),
            storage_file_version: env!("CARGO_PKG_VERSION")
                .split('.')
                .next()
                .unwrap_or("")
                .to_string(),
        }
    }

    /// Get the main application version
    pub fn get_app_version(&self) -> String {
        self.version.clone()
    }

    /// Get the storage file version
    pub fn get_storage_file_version(&self) -> String {
        self.storage_file_version.clone()
    }

    pub fn as_metadata(&self) -> FileStorageMetadata {
        FileStorageMetadata {
            storage_file_version: self.storage_file_version.clone(),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FileStorageMetadata {
    /// The version of the storage file format
    pub storage_file_version: String,
}

#[cfg(test)]
mod tests {
    use super::*;

    // --- VersionInfo::new ---

    #[test]
    fn test_app_version_matches_cargo_pkg_version() {
        let v = VersionInfo::new();
        assert_eq!(v.get_app_version(), env!("CARGO_PKG_VERSION"));
    }

    #[test]
    fn test_storage_file_version_is_major_only() {
        let v = VersionInfo::new();
        let expected_major = env!("CARGO_PKG_VERSION").split('.').next().unwrap_or("");
        assert_eq!(v.get_storage_file_version(), expected_major);
    }

    #[test]
    fn test_storage_file_version_contains_no_dots() {
        let v = VersionInfo::new();
        assert!(!v.get_storage_file_version().contains('.'));
    }

    #[test]
    fn test_app_version_is_valid_semver_shape() {
        let v = VersionInfo::new();
        let version = v.get_app_version();
        let parts: Vec<&str> = version.split('.').collect();
        assert_eq!(parts.len(), 3, "expected major.minor.patch format");
    }

    // --- as_metadata ---

    #[test]
    fn test_as_metadata_storage_file_version_matches() {
        let v = VersionInfo::new();
        let meta = v.as_metadata();
        assert_eq!(meta.storage_file_version, v.get_storage_file_version());
    }

    // --- Clone and Debug ---

    #[test]
    fn test_version_info_clone_preserves_values() {
        let v = VersionInfo::new();
        let c = v.clone();
        assert_eq!(v.get_app_version(), c.get_app_version());
        assert_eq!(v.get_storage_file_version(), c.get_storage_file_version());
    }

    #[test]
    fn test_version_info_debug_is_non_empty() {
        let v = VersionInfo::new();
        assert!(!format!("{:?}", v).is_empty());
    }

    #[test]
    fn test_file_storage_metadata_clone_preserves_values() {
        let meta = VersionInfo::new().as_metadata();
        let cloned = meta.clone();
        assert_eq!(meta.storage_file_version, cloned.storage_file_version);
    }

    #[test]
    fn test_file_storage_metadata_debug_is_non_empty() {
        let meta = VersionInfo::new().as_metadata();
        assert!(!format!("{:?}", meta).is_empty());
    }

    // --- FileStorageMetadata serde round-trip ---

    #[test]
    fn test_file_storage_metadata_serde_round_trip() {
        let meta = VersionInfo::new().as_metadata();
        let json = serde_json::to_string(&meta).unwrap();
        let parsed: FileStorageMetadata = serde_json::from_str(&json).unwrap();
        assert_eq!(parsed.storage_file_version, meta.storage_file_version);
    }
}
