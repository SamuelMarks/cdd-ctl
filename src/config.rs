use crate::error::*;
use crate::service::*;
use crate::util;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::PathBuf;

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Component {
    tests: bool,
    routes: bool,
    validation: bool,
    models: bool,
}

#[derive(Serialize, Deserialize)]
pub(crate) struct Config {
    name: String,
    version: String,
    description: String,
    author: String,
    openapi: String,
    auth: String,
    pub(crate) services: HashMap<String, CDDService>,
}

impl Config {
    /// Read a configuration file from an optional location, or try several default locations.
    pub fn read(file: PathBuf) -> CliResult<Self> {
        let file_contents: String = util::read_file(file)?;
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

        #[cfg(target_os = "macos")]
        let bin_path = "~/.cdd/bin/darwin";

        #[cfg(target_os = "linux")]
        let bin_path = "~/.cdd/bin/linux";

        services.insert(
            "rust".to_string(),
            CDDService {
                bin_path: format!("{}/cdd-rust", bin_path),
                template_path: "~/.cdd/templates/rust".to_string(),
                project_path: "./rust".to_string(),
                component_file: "src/api/models.rs".to_string(),
                requests_file: "src/api/routes.rs".to_string(),
            },
        );

        services.insert(
            "typescript".to_string(),
            CDDService {
                bin_path: format!("{}/cdd-rust", bin_path),
                template_path: "~/.cdd/templates/typescript".to_string(),
                project_path: "./typescript".to_string(),
                component_file: "API/Models.ts".to_string(),
                requests_file: "API/Requests.ts".to_string(),
            },
        );

        services.insert(
            "kotlin".to_string(),
            CDDService {
                bin_path: format!("{}/cdd-rust", bin_path),
                template_path: "~/.cdd/templates/kotlin".to_string(),
                project_path: "./kotlin".to_string(),
                component_file: "API/Models.kt".to_string(),
                requests_file: "API/Requests.kt".to_string(),
            },
        );

        #[cfg(target_os = "macos")]
        services.insert(
            "ios".to_string(),
            CDDService {
                bin_path: format!("{}/cdd-rust", bin_path),
                template_path: "~/.cdd/templates/iOS".to_string(),
                project_path: "./iOS".to_string(),
                component_file: "cddTemplate/Source/API/APIModels.swift".to_string(),
                requests_file: "cddTemplate/Source/API/APIRequests.swift".to_string(),
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
