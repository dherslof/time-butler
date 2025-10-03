/*
 * File: config_reader.rs
 * Description: The application configuration reader. Should read the configuration file and provide the configuration parameters.
 * Author: dherslof
 * Created: 22-09-2025
 * License: MIT
 */

use serde_json;
use std::fs;
use std::io::Write;
use std::path::Path;

use crate::config::AppConfiguration;

pub struct ConfigReader {
    config_path: String,
    configuration: Option<AppConfiguration>,
}

impl ConfigReader {
    pub fn new(config_path: &str) -> Self {
        Self {
            config_path: config_path.to_string(),
            configuration: None,
        }
    }

    /// Read the configuration file and populate the configuration struct
    pub fn read_config(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let path = Path::new(&self.config_path);
        if !path.exists() {
            return Err(format!("Configuration file not found: {}", self.config_path).into());
        }

        let config_content = fs::read_to_string(path)?;
        let config: AppConfiguration = serde_json::from_str(&config_content)?;
        self.configuration = Some(config);

        Ok(())
    }

    /// Get a reference to the configuration
    pub fn get_configuration(&self) -> Option<&AppConfiguration> {
        self.configuration.as_ref()
    }

    /// Write new configuration to the file
    pub fn write_config(
        &self,
        config: &AppConfiguration,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let json = serde_json::to_string_pretty(config)?;
        let path = Path::new(&self.config_path);
        let mut file = fs::File::create(path)?;
        file.write_all(json.as_bytes())?;
        Ok(())
    }
}
