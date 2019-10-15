use crate::*;
use log::*;
use serde::{Deserialize, Serialize};

// A graph is generated from spec or project source, compared and derived as a source of truth.
#[derive(Debug, PartialEq)]
pub struct ProjectGraph {
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
}

#[derive(Debug, PartialEq, Deserialize, Serialize)]
pub struct Route {
    pub name: String,
}

impl From<openapiv3::OpenAPI> for ProjectGraph {
    fn from(openapi: openapiv3::OpenAPI) -> Self {
        info!("Creating project graph from OpenAPI spec");
        let mut models = vec![];

        for component in openapi.components {
            for (name, schema) in component.schemas {
                models.push(Model { name: name })
            }
        }
        ProjectGraph {
            models,
            routes: vec![],
        }
    }
}

// use crate::services::CDDService;
// impl From<CDDService> for ProjectGraph {
//     fn from(service: CDDService) -> Self {
//         info!("Extracting models from: {}", service.component_file);
//         let models = service.extract_models()?;
//         ProjectGraph {
//             models,
//             routes: vec![],
//         }
//     }
// }
