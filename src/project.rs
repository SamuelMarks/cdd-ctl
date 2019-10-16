use crate::config::Config;
use crate::instruction::Instruction;
use crate::project_graph::*;
use crate::services::*;
use crate::*;
use log::*;
use openapiv3::OpenAPI;
use std::path::PathBuf;

pub struct Project {
    config: Config,
    spec: OpenAPI,
    // graphs: Vec<ProjectGraph>,
}

impl Project {
    pub fn read(path: &PathBuf) -> CliResult<Self> {
        let config = Config::read(path.join("config.yml"))?;
        let spec = load_openapi_spec()?;
        // let graphs = vec![];

        Ok(Project {
            config,
            spec,
            // graphs,
        })
    }

    fn simple_sync_models(master_models: Vec<Model>, service: &CDDService) -> CliResult<()> {
        let models_in_project = service.extract_models()?.all_names();
        let models_in_spec = master_models;

        for model in models_in_project
            .into_iter()
            .filter(|model_name| !models_in_spec.all_names().contains(&model_name))
        {
            // delete
            service.delete_model(&model)?;
        }

        for model in models_in_spec {
            let model_name = &model.name;
            if service.contains_model(model_name)? {
                info!("Model {} was found in project", model_name);
            } else {
                warn!(
                    "Model {} was not found in project, inserting...",
                    &model_name
                );
                service.insert_or_update_model(model)?;
            }
        }

        Ok(())
    }

    fn simple_sync_routes(master_routes: Vec<Route>, service: &CDDService) -> CliResult<()> {
        let routes_in_project = service.extract_routes()?.all_names();
        let routes_in_spec = master_routes;

        for route_name in routes_in_project
            .into_iter()
            .filter(|route_name| !routes_in_spec.all_names().contains(&route_name))
        {
            // delete
            service.delete_route(&route_name)?;
        }

        for route in routes_in_spec {
            let route_name = &route.name;
            if service.contains_route(route_name)? {
                info!("Route {} was found in project", route_name);
            } else {
                warn!(
                    "Route {} was not found in project, inserting...",
                    &route_name
                );
                service.insert_or_update_route(route)?;
            }
        }

        Ok(())
    }

    /// super basic one way spec -> projects sync
    pub fn simple_sync(&self) -> CliResult<()> {
        for (_name, service) in self.config.services.clone() {
            let spec_graph = project_graph::ProjectGraph::from(self.spec.clone());
            info!(
                "Found {} models, {} routes in {}",
                spec_graph.models.len(),
                spec_graph.routes.len(),
                "openapi.yml"
            );
            let _ = Project::simple_sync_models(spec_graph.models, &service)?;
            let _ = Project::simple_sync_routes(spec_graph.routes, &service)?;
        }

        Ok(())
    }

    pub fn copy_templates(&self) -> CliResult<()> {
        info!("Checking project directories");
        for (name, service) in self.config.services.clone() {
            let project_path = service.project_path.clone();
            // let project_path = PathBuf::from(".");
            if !util::file_exists(project_path.clone()) {
                warn!(
                    "Could not find local project for {} at {} - copying fresh template from {}",
                    name.clone(),
                    project_path,
                    service.template_path,
                );

                let template_path = util::expand_home_path(service.template_path.clone())?;
                util::copy_dir(template_path, ".")?;
            } else {
                info!("Found service: {}", name);
            }
        }
        Ok(())
    }

    pub fn generate_instruction_tree(&self) -> CliResult<Vec<Instruction>> {
        info!("Generating project graphs");
        let spec_graph = project_graph::ProjectGraph::from(self.spec.clone());
        let mut instruction_tree = Vec::new();

        for model in spec_graph.models {
            instruction_tree.push(Instruction::AddModel(model.clone()));
        }

        Ok(instruction_tree)
    }

    pub fn generate_project_graphs(&self) -> CliResult<Vec<ProjectGraph>> {
        info!("Generating project graphs");
        let mut graphs = Vec::new();

        for (_name, service) in self.config.services.clone() {
            graphs.push(ProjectGraph {
                models: service.extract_models()?,
                routes: service.extract_routes()?,
            })
        }

        Ok(graphs)
    }
}

// fn load_config_file() -> CliResult<config::Config> {
//     let config_path = PathBuf::from("./config.yml");

//     if !config_path.clone().exists() {
//         return Err(failure::format_err!(
//             "Could not find a config.yml. Try running the init command first if this is a new project."
//         ));
//     };

//     let config = config::Config::read(config_path)?;
//     info!("Read config file from ./config.yml");

//     Ok(config)
// }

fn load_openapi_spec() -> CliResult<OpenAPI> {
    let spec_path: PathBuf = PathBuf::from("openapi.yml");

    if !spec_path.exists() {
        return Err(failure::format_err!("Could not find openapi.yml"));
    };

    let spec = std::fs::read_to_string(spec_path).unwrap();
    let openapi: OpenAPI = serde_yaml::from_str(&spec).expect("Could not deserialize input");

    Ok(openapi)
}
