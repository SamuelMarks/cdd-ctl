use crate::error::*;
use dirs;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Config {
    name: String,
    version: String,
    description: String,
    author: String,
}

impl Config {
    /// Read a configuration file from an optional location, or try several default locations.
    pub fn read(file: Option<PathBuf>) -> CliResult<Self> {
        let file_contents: String;

        // if a config file location was provided,
        if let Some(file) = file {
            // read the file into a string
            file_contents = file.into_os_string().into_string().map_err(|_| {
                CliError::ConfigError(format!(
                    "cannot read configuaration file: {}",
                    file.display()
                ))
            })?;
        } else {
            // otherwise search predetermined locations
            let search_paths: Vec<PathBuf> = vec![PathBuf::from(r"./config.yaml")];

            if let Some(valid_path) = search_paths.into_iter().find(|p| p.exists()) {
                let file_contents = valid_path.into_os_string().into_string();
            } else {
                return Err(Box::new(CliError::ConfigError("bad files".to_string())));
            }
        }

        // attempt to deserialise it
        Ok(serde_yaml::from_str(&file_contents)?)
    }
}
