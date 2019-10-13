use crate::*;
use log::*;

// A graph is generated from spec or project source, compared and derived as a source of truth.
#[derive(Debug, PartialEq)]
pub struct ProjectGraph {
    pub models: Vec<Model>,
    pub routes: Vec<Route>,
    // config: ProjectConfig
    // service: Service
}

// instruction_tree: HashMap<String, MergeInstruction>
// pub enum MergeInstruction {
//     DeleteModel(Model),
//     UpdateModel(Model),
//     AddModel(Model),
// }

#[derive(Debug, PartialEq, Clone)]
pub struct Model {
    name: String,
    // date_modified: Date,
}

#[derive(Debug, PartialEq)]
pub struct Route;

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
