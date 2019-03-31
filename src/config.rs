use crate::error::*;
use dirs;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

/// search path locations
pub static CONFIG_SEARCH_PATHS: [&str; 1] = [r"./config.yaml"];

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Config {
    name: String,
    version: String,
    description: String,
    author: String,
}

fn pathbuf_to_string(pathbuf: PathBuf) -> CliResult<String> {
    Ok(pathbuf
        .clone()
        .into_os_string()
        .into_string()
        .map_err(|_| {
            CliError::ConfigError(format!(
                "cannot read configuaration file: {}",
                pathbuf.display()
            ))
        })?)
}

impl Config {
    /// Read a configuration file from an optional location, or try several default locations.
    pub fn read(file: Option<PathBuf>) -> CliResult<Self> {
        let file_contents: String;

        // if a config file location was provided,
        if let Some(file) = file {
            // read the file into a string
            file_contents = pathbuf_to_string(file)?;
        } else {
            // otherwise search predetermined locations
            let search_paths: Vec<PathBuf> = CONFIG_SEARCH_PATHS
                .iter()
                .map(|sp| PathBuf::from(sp))
                .collect();

            // find the first valid entry in the array
            if let Some(valid_path) = search_paths.into_iter().find(|p| p.exists()) {
                file_contents = pathbuf_to_string(valid_path)?;
            } else {
                return Err(Box::new(CliError::ConfigError("bad files".to_string())));
            }
        }

        // attempt to deserialise it
        Ok(serde_yaml::from_str(&file_contents)?)
    }
}
