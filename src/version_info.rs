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
