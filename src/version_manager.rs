/*
 * File: version_manager.rs
 * Description: Manages version information and compatibility
 * Author: dherslof
 * Created: 18-05-2026
 * License: MIT
 */

use crate::version_info::FileStorageMetadata;
use crate::version_info::VersionInfo;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum VersionCompatibility {
    Compatible,
    Incompatible(String), // Contains a message about the incompatibility
    Unknown,              // When no metadata is loaded
}

pub struct VersionManager {
    /// The current version of the application
    current_version: VersionInfo,
    /// The storage metadata
    loaded_storage_metadata: Option<FileStorageMetadata>,
    /// Campatibility status of the loaded storage file
    storage_compatibility: VersionCompatibility,
    override_on_incompatibility: bool,
}

impl VersionManager {
    /// Create a new VersionManager instance
    pub fn new(version_info: VersionInfo) -> Self {
        VersionManager {
            current_version: version_info,
            loaded_storage_metadata: None,
            storage_compatibility: VersionCompatibility::Unknown,
            override_on_incompatibility: false,
        }
    }

    pub fn ok_to_save_files(&self) -> bool {
        (self.storage_compatibility == VersionCompatibility::Compatible)
            || (self.storage_compatibility == VersionCompatibility::Unknown)
            || self.override_on_incompatibility
    }

    pub fn set_override_on_incompatibility(&mut self, override_on_incompatibility: bool) {
        self.override_on_incompatibility = override_on_incompatibility;
    }

    pub fn set_loaded_storage_metadata(&mut self, metadata: Option<FileStorageMetadata>) {
        self.loaded_storage_metadata = metadata;
    }

    pub fn is_compatible(&mut self) -> VersionCompatibility {
        // Consider compatible if the storage file version matches the current application's storage file version
        match &self.loaded_storage_metadata {
            Some(metadata) => {
                if metadata.storage_file_version == self.current_version.get_storage_file_version()
                {
                    self.storage_compatibility = VersionCompatibility::Compatible;
                    VersionCompatibility::Compatible
                } else {
                    self.storage_compatibility = VersionCompatibility::Incompatible(format!(
                        "Incompatible storage file version: {}. Expected: {}",
                        metadata.storage_file_version,
                        self.current_version.get_storage_file_version()
                    ));
                    VersionCompatibility::Incompatible(format!(
                        "Incompatible storage file version: {}. Expected: {}",
                        metadata.storage_file_version,
                        self.current_version.get_storage_file_version()
                    ))
                }
            }
            None => {
                self.storage_compatibility = VersionCompatibility::Unknown;
                VersionCompatibility::Unknown // If no metadata is loaded, we cannot determine compatibility
            }
        }
    }

    pub fn metadata(&self) -> FileStorageMetadata {
        self.current_version.as_metadata()
    }

    pub fn get_version(&self) -> VersionInfo {
        self.current_version.clone()
    }
}
