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
                // no files were found, so attempt to create a default in the default global configuration.
                let config = Config::default();
                config.write(PathBuf::from(r"./config.yaml"))?;
                return Ok(config);
            }
        }

        // attempt to deserialise it
        Ok(serde_yaml::from_str(&file_contents)?)
    }

    pub fn write(&self, path: PathBuf) -> CliResult<()> {
        use std::fs::File;
        use std::io::Write;

        let yaml = serde_yaml::to_string(&self)?;
        let mut output = File::create(path)?;

        write!(output, "{}", yaml)?;

        Ok(())
    }
}

impl Default for Config {
    fn default() -> Self {
        Config {
            name: "default project".to_string(),
            version: "0.0.1".to_string(),
            description: "description".to_string(),
            author: "me@me.com".to_string(),
        }
    }
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
