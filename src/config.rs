use crate::error::*;
use crate::services::*;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::PathBuf;

/// search path locations
pub static CONFIG_SEARCH_PATHS: [&str; 1] = [r"./config.yaml"];

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Component {
    tests: bool,
    routes: bool,
    validation: bool,
    models: bool,
}

#[derive(Serialize, Deserialize)]
pub struct Config {
    name: String,
    version: String,
    description: String,
    author: String,
    openapi: String,
    auth: String,
    services: HashMap<String, CDDService>,
}

impl Config {
    /// Read a configuration file from an optional location, or try several default locations.
    pub fn read(file: PathBuf) -> CliResult<Self> {
        let file_contents: String = pathbuf_to_string(file)?;
        Ok(serde_yaml::from_str(&file_contents)?)
    }

    pub fn write(&self, path: PathBuf) -> CliResult<()> {
        use std::fs::File;
        use std::io::Write;

        let yaml = serde_yaml::to_string(&self)?;
        let mut output = File::create(path)?;

        write!(output, "{}\n", yaml)?;

        Ok(())
    }
}

impl Default for Config {
    fn default() -> Self {
        let mut services = HashMap::new();
        services.insert(
            "swift".to_string(),
            CDDService {
                bin_path: "services/cdd-swift".to_string(),
            },
        );

        Config {
            name: "default project".to_string(),
            version: "0.0.1".to_string(),
            description: "description".to_string(),
            author: "me@me.com".to_string(),
            openapi: "openapi.yaml".to_string(),
            auth: "rfc6749".to_string(),
            services,
        }
    }
}

fn pathbuf_to_string(pathbuf: PathBuf) -> CliResult<String> {
    use std::fs::File;
    use std::io::prelude::*;

    let mut f = File::open(pathbuf)?;
    let mut buffer = String::new();

    f.read_to_string(&mut buffer)?;

    Ok(buffer)
}
