use crate::project_graph::*;
use crate::*;
use log::*;
use serde::{Deserialize, Serialize};
use serde_json;

#[derive(Serialize, Deserialize, Clone)]
pub(crate) struct CDDService {
    pub bin_path: String,
    pub template_path: String,
    pub project_path: String,
    pub component_file: String,
}

impl CDDService {
    pub fn extract_models(&self) -> CliResult<Vec<Model>> {
        self.exec(vec!["list-models", "blah.rust"])
            .and_then(|json| Ok(serde_json::from_str::<Vec<Model>>(&json)?))
    }

    pub fn extract_routes(&self) -> CliResult<Vec<Route>> {
        self.exec(vec!["test-error"])
            .and_then(|json| Ok(serde_json::from_str::<Vec<Route>>(&json)?))
    }

    fn exec(&self, args: Vec<&str>) -> CliResult<String> {
        let bin_path = util::expand_home_path(self.bin_path.clone())?;

        if !util::file_exists(&bin_path) {
            return Err(failure::format_err!(
                "Service not found at {} as specified in config.yml",
                &self.bin_path
            ));
        }
        let cmd = util::exec(&bin_path, args);
        match &cmd {
            Ok(msg) => info!("{}", msg),
            Err(msg) => error!("{}", msg),
        };
        cmd
    }
}
