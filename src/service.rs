use crate::project::*;
use crate::*;
use log::*;
use serde::{Deserialize, Serialize};
use serde_json;

#[derive(Serialize, Deserialize, Clone)]
pub(crate) struct CDDService {
    pub bin_path: String,
    pub address: String,
    pub template_path: String,
    pub project_path: String,
    pub component_file: String,
    pub requests_file: String,
}

impl CDDService {
    /// request the adaptor creates new project files
    pub fn create_template(&self) -> CliResult<()> {
        self.exec(vec!["create-template", &self.project_path])
            .map(|_| ())
    }

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
                for line in self.update_model(model.clone())?.lines() {
                    info!("{}", line);
                }
            } else {
                warn!(
                    "Model {} was not found in project, inserting...",
                    &model_name
                );
                for line in self.insert_model(model.clone())?.lines() {
                    info!("{}", line);
                }
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
                for line in self.update_request(request.clone())?.lines() {
                    info!("{}", line);
                }
            } else {
                warn!(
                    "Request {} was not found in project, inserting...",
                    &request_name
                );
                for line in self.insert_request(request.clone())?.lines() {
                    info!("{}", line);
                }
            }
        }

        Ok(())
    }

    pub fn write_tests(&self) -> CliResult<String> {
        info!("Writing tests for {}", self.project_path);

        Ok(self.exec(vec!["generate-tests"])?)
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

    pub fn request_files(&self) -> String {
        [self.project_path.clone(), self.requests_file.clone()].join("/")
    }

    pub fn extract_models(&self) -> CliResult<Vec<Model>> {
        info!("Extracting models from {}", self.model_files());
        self.exec(vec!["list-models", &self.model_files()])
            .and_then(|json| {
                Ok(serde_json::from_str::<Vec<Model>>(&json).map_err(|e| {
                    failure::format_err!("Error parsing JSON result: {}\n{}", e, json)
                })?)
            })
    }

    pub fn extract_requests(&self) -> CliResult<Vec<Request>> {
        info!("Extracting requests from {}", self.request_files());
        self.exec(vec!["list-requests", &self.request_files()])
            .and_then(|json| Ok(serde_json::from_str::<Vec<Request>>(&json)?))
    }

    pub fn insert_model(&self, model: Model) -> CliResult<String> {
        info!("Inserting model {}", model.name);
        Ok(self.exec(vec![
            "insert-model",
            &self.model_files(),
            &serde_json::to_string(&model)?,
        ])?)
    }

    pub fn insert_request(&self, request: Request) -> CliResult<String> {
        info!("Inserting request {}", request.name);
        Ok(self.exec(vec![
            "insert-request",
            &self.request_files(),
            &serde_json::to_string(&request)?,
        ])?)
    }

    pub fn update_model(&self, model: Model) -> CliResult<String> {
        info!("Updating model {}", model.name);
        Ok(self.exec(vec![
            "update-model",
            &self.model_files(),
            &serde_json::to_string(&model)?,
        ])?)
    }

    pub fn update_request(&self, request: Request) -> CliResult<String> {
        info!("Updating request {}", request.name);
        Ok(self.exec(vec![
            "update-request",
            &self.request_files(),
            &serde_json::to_string(&request)?,
        ])?)
    }

    pub fn delete_model(&self, name: &str) -> CliResult<String> {
        warn!("Deleting model {}", name);
        self.exec(vec!["delete-model", &self.model_files(), name])
    }

    pub fn delete_request(&self, name: &str) -> CliResult<String> {
        warn!("Deleting request {}", name);
        self.exec(vec!["delete-request", &self.request_files(), name])
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
