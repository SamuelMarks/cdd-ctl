use crate::config::Config;
use crate::project::*;
use crate::*;
use log::*;
use openapiv3::OpenAPI;
use std::path::PathBuf;

pub struct ProjectGraph {
    config: Config,
    spec: OpenAPI,
    // graphs: Vec<ProjectGraph>,
}

impl ProjectGraph {
    pub fn read(path: &PathBuf) -> CliResult<Self> {
        let config = Config::read(path.join("config.yml"))?;
        let spec = load_openapi_spec()?;
        // let graphs = vec![];

        Ok(ProjectGraph {
            config,
            spec,
            // graphs,
        })
    }

    /// super basic one way spec -> projects sync
    pub fn simple_sync(&self) -> CliResult<()> {
        // let spec_graph = Project::from(self.spec.clone());
        let spec_graph = Project::parse_yml(self.spec.clone());

        info!(
            "Found {} models, {} routes in {}",
            spec_graph.models.len(),
            spec_graph.requests.len(),
            "openapi.yml"
        );
        for (_name, service) in self.config.services.clone() {
            service.sync_with(&spec_graph)?;
        }

        util::write_file(
            PathBuf::from("schema.sql"),
            &schema::generate("schema.sql", spec_graph),
        )?;

        Ok(())
    }

    pub fn copy_templates(&self) -> CliResult<()> {
        info!("Checking project directories");
        for (name, service) in self.config.services.clone() {
            let project_path = service.project_path.clone();
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

    // pub fn generate_instruction_tree(&self) -> CliResult<Vec<Instruction>> {
    //     info!("Generating project graphs");
    //     let spec_graph = project_graph::Project::from(self.spec.clone());
    //     let mut instruction_tree = Vec::new();

    //     for model in spec_graph.models {
    //         instruction_tree.push(Instruction::AddModel(model.clone()));
    //     }

    //     Ok(instruction_tree)
    // }

    // pub fn generate_project_graphs(&self) -> CliResult<Vec<Project>> {
    //     info!("Generating project graphs");
    //     let mut graphs = Vec::new();

    //     for (_name, service) in self.config.services.clone() {
    //         graphs.push(Project {
    //             models: service.extract_models()?,
    //             routes: service.extract_routes()?,
    //         })
    //     }

    //     Ok(graphs)
    // }
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
