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
    pub routes_file: String,
}

impl CDDService {
    pub fn sync_with(&self, spec_project: &Project) -> CliResult<()> {
        let project = self.extract_project()?;

        let project_model_names = project.models.all_names();
        let spec_model_names = spec_project.models.all_names();
        let project_request_names = project.requests.all_names();
        let spec_request_names = spec_project.requests.all_names();

        info!(
            "Found {} models ({}), {} requests ({}) in {}",
            project.models.len(),
            project_model_names.join(", "),
            project.requests.len(),
            project_request_names.join(", "),
            self.project_path,
        );

        for model in project_model_names
            .iter()
            .filter(|model_name| !spec_model_names.contains(model_name))
        {
            self.delete_model(&model)?;
        }

        for model in spec_project.models.iter() {
            let model_name = &model.name;
            if project_model_names.contains(model_name) {
                info!("Model {} was found in project", model_name);
            } else {
                warn!(
                    "Model {} was not found in project, inserting...",
                    &model_name
                );
                self.insert_or_update_model(model.clone())?;
            }
        }

        for request in project_request_names
            .iter()
            .filter(|request_name| !spec_request_names.contains(request_name))
        {
            self.delete_request(&request)?;
        }

        for request in spec_project.requests.iter() {
            let request_name = &request.name;
            if project_request_names.contains(request_name) {
                info!("Request {} was found in project", request_name);
            } else {
                warn!(
                    "Request {} was not found in project, inserting...",
                    &request_name
                );
                self.insert_or_update_request(request.clone())?;
            }
        }

        Ok(())
    }

    pub fn extract_project(&self) -> CliResult<Project> {
        info!("Extracting objects from {}", self.project_path);

        Ok(Project {
            models: self.extract_models()?,
            requests: self.extract_requests()?,
            info: crate::project::Info {
                host: "".to_string(),
                endpoint: "".to_string(),
            },
        })
    }

    pub fn model_files(&self) -> String {
        [self.project_path.clone(), self.component_file.clone()].join("/")
    }

    pub fn route_files(&self) -> String {
        [self.project_path.clone(), self.routes_file.clone()].join("/")
    }

    pub fn extract_models(&self) -> CliResult<Vec<Model>> {
        info!("Extracting models from {}", self.model_files());
        self.exec(vec!["list-models", &self.model_files()])
            .and_then(|json| Ok(serde_json::from_str::<Vec<Model>>(&json)?))
    }

    pub fn extract_requests(&self) -> CliResult<Vec<Request>> {
        info!("Extracting requests from {}", self.model_files());
        self.exec(vec!["list-requests", &self.model_files()])
            .and_then(|json| Ok(serde_json::from_str::<Vec<Request>>(&json)?))
    }

    pub fn insert_or_update_model(&self, model: Model) -> CliResult<String> {
        info!("Inserting/Updating model {}", model.name);
        Ok(self.exec(vec![
            "update-model",
            &self.model_files(),
            &serde_json::to_string(&model)?,
        ])?)
    }

    pub fn insert_or_update_request(&self, request: Request) -> CliResult<String> {
        info!("Inserting/Updating request {}", request.name);
        Ok(self.exec(vec![
            "update-request",
            &self.route_files(),
            &serde_json::to_string(&request)?,
        ])?)
    }

    pub fn delete_model(&self, name: &str) -> CliResult<String> {
        warn!("Deleting model {}", name);
        self.exec(vec!["delete-model", &self.model_files(), name])
    }

    pub fn delete_request(&self, name: &str) -> CliResult<String> {
        warn!("Deleting request {}", name);
        self.exec(vec!["delete-request", &self.route_files(), name])
    }

    // pub fn model_names(&self) -> CliResult<Vec<String>> {
    //     Ok(self
    //         .extract_models()?
    //         .into_iter()
    //         .map(|model| model.name)
    //         .collect())
    // }

    // pub fn request_names(&self) -> CliResult<Vec<String>> {
    //     Ok(self
    //         .extract_requests()?
    //         .into_iter()
    //         .map(|request| request.name)
    //         .collect())
    // }

    // pub fn contains_model(&self, model_name: &str) -> CliResult<bool> {
    //     Ok(self.model_names()?.contains(&model_name.to_string()))
    // }

    // pub fn contains_request(&self, request_name: &str) -> CliResult<bool> {
    //     Ok(self.request_names()?.contains(&request_name.to_string()))
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
