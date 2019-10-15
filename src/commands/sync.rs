use crate::services::*;
use crate::*;
use log::*;
use std::path::PathBuf;

pub fn sync() -> CliResult<()> {
    let project_path = PathBuf::from(".");
    let project = project::Project::read(&project_path)?;

    // ensure all projects are in place
    project.copy_templates()?;
    project.simple_sync()?;

    // let graph = project.generate_project_graphs()?;
    // info!(
    //     "Successfully generated project graph with models ({:?}) and routes ({:?})",
    //     graph, graph
    // );

    // load config
    // let config = load_config_file()?;
    // let _ = ensure_projects_exist(config)?;
    // // load spec
    // let spec = load_openapi_spec()?;
    // // collect model graph using adaptors
    // let spec_graph = project_graph::ProjectGraph::from(spec);
    // println!("graph: {:?}", &spec_graph);
    // sync graphs into an instruction tree
    // execute tree
    Ok(())
}

fn ensure_projects_exist(config: config::Config) -> CliResult<()> {
    info!("Checking project directories");
    for (name, service) in config.services {
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
            info!("Found: {}", name);
        }
    }
    Ok(())
}

fn load_config_file() -> CliResult<config::Config> {
    let config_path = PathBuf::from("./config.yml");

    if !config_path.clone().exists() {
        return Err(failure::format_err!(
            "Could not find a config.yml. Try running the init command first if this is a new project."
        ));
    };

    let config = config::Config::read(config_path)?;
    info!("Read config file from ./config.yml");

    Ok(config)
}

use openapiv3::OpenAPI;
fn load_openapi_spec() -> CliResult<OpenAPI> {
    let spec_path: PathBuf = PathBuf::from("openapi.yml");

    if !spec_path.exists() {
        return Err(failure::format_err!("Could not find openapi.yml"));
    };

    let spec = std::fs::read_to_string(spec_path).unwrap();
    let openapi: OpenAPI = serde_yaml::from_str(&spec).expect("Could not deserialize input");

    Ok(openapi)
}
