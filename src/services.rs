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
        info!("Extracting models from {}", self.component_file);
        self.exec(vec!["list-models", &self.component_file])
            .and_then(|json| Ok(serde_json::from_str::<Vec<Model>>(&json)?))
    }

    pub fn extract_routes(&self) -> CliResult<Vec<Route>> {
        info!("Extracting routes from {}", self.component_file);
        self.exec(vec!["list-routes", &self.component_file])
            .and_then(|json| Ok(serde_json::from_str::<Vec<Route>>(&json)?))
    }

    pub fn insert_or_update_model(&self, model: Model) -> CliResult<String> {
        info!("Inserting/Updating model {}", model.name);
        error!(":::{}", &serde_json::to_string(&model)?);
        Ok(self.exec(vec!["update-model", &serde_json::to_string(&model)?])?)
    }

    pub fn insert_or_update_route(&self, route: Route) -> CliResult<String> {
        info!("Inserting/Updating route {}", route.name);
        Ok(self.exec(vec!["update-route", &serde_json::to_string(&route)?])?)
    }

    pub fn delete_model(&self, name: &str) -> CliResult<String> {
        warn!("Deleting model {}", name);
        self.exec(vec!["delete-model", name])
    }

    pub fn delete_route(&self, name: &str) -> CliResult<String> {
        warn!("Deleting route {}", name);
        self.exec(vec!["delete-route", name])
    }

    pub fn model_names(&self) -> CliResult<Vec<String>> {
        Ok(self
            .extract_models()?
            .into_iter()
            .map(|model| model.name)
            .collect())
    }

    pub fn contains_model(&self, model_name: &str) -> CliResult<bool> {
        Ok(self.model_names()?.contains(&model_name.to_string()))
    }

    fn exec(&self, args: Vec<&str>) -> CliResult<String> {
        let bin_path = util::expand_home_path(self.bin_path.clone())?;

        if !util::file_exists(&bin_path) {
            return Err(failure::format_err!(
                "Service not found at {} as specified in config.yml",
                &self.bin_path
            ));
        }

        util::exec(&bin_path, args)
    }
}
