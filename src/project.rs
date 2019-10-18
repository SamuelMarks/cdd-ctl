use crate::*;
use log::*;
use serde::{Deserialize, Serialize};

// A graph is generated from spec or project source, compared and derived as a source of truth.
#[derive(Debug, PartialEq)]
pub struct Project {
    pub models: Vec<Model>,
    pub routes: Vec<Route>,
    // config: ProjectConfig
    // service: Service
}
pub trait CustomIterators {
    fn all_names(&self) -> Vec<String>;
}
impl CustomIterators for Vec<Model> {
    fn all_names(&self) -> Vec<String> {
        self.into_iter().map(|model| model.name.clone()).collect()
    }
}
impl CustomIterators for Vec<Route> {
    fn all_names(&self) -> Vec<String> {
        self.into_iter()
            .map(|route| route.url_path.clone())
            .collect()
    }
}

// instruction_tree: HashMap<String, MergeInstruction>
// pub enum MergeInstruction {
//     DeleteModel(Model),
//     UpdateModel(Model),
//     AddModel(Model),
// }

#[derive(Debug, PartialEq, Clone, Deserialize, Serialize)]
pub struct Model {
    pub name: String,
    // date_modified: Date,
    pub vars: Vec<Variable>,
}

#[derive(Debug, PartialEq, Clone, Deserialize, Serialize)]
pub struct Route {
    // pub name: String,
    pub method: String,
    pub url_path: String,
    pub response_type: String,
    pub error_type: String,
    pub vars: Vec<Variable>,
}

#[derive(Debug, PartialEq, Clone, Deserialize, Serialize)]
pub struct Variable {
    name: String,
    required: bool,
    vartype: String,
    value: String,
}

impl From<openapiv3::OpenAPI> for Project {
    fn from(openapi: openapiv3::OpenAPI) -> Self {
        info!("Extracting objects from OpenAPI spec");
        let mut models = Vec::new();
        let mut routes = Vec::new();

        for component in openapi.components {
            for (name, schema) in component.schemas {
                println!("MODEL: {:?}", schema);
                models.push(Model {
                    name: name,
                    vars: Vec::new(),
                })
            }
        }

        for (path, item) in openapi.paths {
            if let openapiv3::ReferenceOr::Item(item) = item {
                fn make_route(
                    method: &str,
                    url_path: &str,
                    operation: &openapiv3::Operation,
                ) -> Route {
                    Route {
                        method: method.to_string(),
                        error_type: "".to_string(),
                        response_type: "".to_string(),
                        url_path: url_path.to_string(),
                        vars: Vec::new(),
                    }
                }

                if let Some(operation) = item.get {
                    println!("GET: {:#?}", operation);
                    routes.push(make_route("GET", &path, &operation));
                }
                if let Some(operation) = item.put {
                    routes.push(make_route("PUT", &path, &operation));
                }
                if let Some(operation) = item.post {
                    routes.push(make_route("POST", &path, &operation));
                }
                if let Some(operation) = item.delete {
                    routes.push(make_route("DELETE", &path, &operation));
                }
            }
        }

        Project { models, routes }
    }
}
