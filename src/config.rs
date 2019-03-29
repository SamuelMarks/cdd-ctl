use crate::error::*;
use dirs;
use serde::{Deserialize, Serialize};
use std::fs;

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Config {
    name: String,
    version: String,
    description: String,
    author: String,
}

impl Config {
    pub fn read(file: Option<String>) -> CliResult<Self> {
        pub fn file_exists(path: &str) -> bool {
            fs::metadata(path).map(|p| p.is_file()).is_ok()
        }

        fn str_from_file_path(path: &str) -> CliResult<String> {
            use std::io::prelude::*;

            let mut handle = ::std::fs::File::open(path)?;
            let mut bytebuffer = Vec::new();

            handle.read_to_end(&mut bytebuffer)?;

            Ok(String::from_utf8(bytebuffer)?)
        }

        // search paths for config files, in order of search preference.
        let search_paths: Vec<String> = vec![
            file,
            Some(format!("./config.yaml")),
            dirs::home_dir().map(|home_dir| format!("{}/config.yaml", home_dir.display())),
        ]
        .into_iter()
        .filter_map(|e| e)
        .collect();

        for path in search_paths.clone() {
            if file_exists(&path) {
                return Ok(serde_yaml::from_str(&str_from_file_path(&path)?)?);
            }
        }

        // else generate a config file here and return it.
        Err(Box::new(CliError::ArgumentError(
            "temporary error".to_string(),
        )))
    }
}
