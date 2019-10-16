use crate::project::*;
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
    pub fn sync_with(&self, spec_project: &Project) -> CliResult<()> {
        let project = self.extract_project()?;

        let project_model_names = project.models.all_names();
        let spec_model_names = spec_project.models.all_names();
        let project_route_names = project.routes.all_names();
        let spec_route_names = spec_project.routes.all_names();

        info!(
            "Found {} models ({}), {} routes ({}) in {}",
            project.models.len(),
            project_model_names.join(", "),
            project.routes.len(),
            project_route_names.join(", "),
            self.project_path,
        );

        for model in project_model_names
            .iter()
            .filter(|model_name| !spec_model_names.contains(model_name))
        {
            self.delete_model(&model)?;
        }

        for model in spec_project.models.clone() {
            let model_name = &model.name;
            if project_model_names.contains(model_name) {
                info!("Model {} was found in project", model_name);
            } else {
                warn!(
                    "Model {} was not found in project, inserting...",
                    &model_name
                );
                self.insert_or_update_model(model)?;
            }
        }

        for route in project_route_names
            .iter()
            .filter(|route_name| !spec_route_names.contains(route_name))
        {
            self.delete_route(&route)?;
        }

        for route in spec_project.routes.clone() {
            let route_name = &route.name;
            if project_route_names.contains(route_name) {
                info!("Route {} was found in project", route_name);
            } else {
                warn!(
                    "Route {} was not found in project, inserting...",
                    &route_name
                );
                self.insert_or_update_route(route)?;
            }
        }

        Ok(())
    }

    pub fn extract_project(&self) -> CliResult<Project> {
        info!("Extracting objects from {}", self.project_path);

        Ok(Project {
            models: self.extract_models()?,
            routes: self.extract_routes()?,
        })
    }

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

    // pub fn model_names(&self) -> CliResult<Vec<String>> {
    //     Ok(self
    //         .extract_models()?
    //         .into_iter()
    //         .map(|model| model.name)
    //         .collect())
    // }

    // pub fn route_names(&self) -> CliResult<Vec<String>> {
    //     Ok(self
    //         .extract_routes()?
    //         .into_iter()
    //         .map(|route| route.name)
    //         .collect())
    // }

    // pub fn contains_model(&self, model_name: &str) -> CliResult<bool> {
    //     Ok(self.model_names()?.contains(&model_name.to_string()))
    // }

    // pub fn contains_route(&self, route_name: &str) -> CliResult<bool> {
    //     Ok(self.route_names()?.contains(&route_name.to_string()))
    // }

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
