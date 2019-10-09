use crate::services::*;
use crate::*;
use log::*;
use std::path::PathBuf;

pub fn sync() -> CliResult<()> {
    // load config
    let config = load_config_file()?;
    // ensure all projects are in place
    let _ = ensure_projects_exist(config)?;
    // load spec
    let _spec = load_openapi_spec()?;
    // collect model tree using adaptors
    // sync trees into an instruction graph
    // execute graph
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

fn load_openapi_spec() -> CliResult<()> {
    let spec_path: PathBuf = dirs::home_dir()
        .ok_or(failure::format_err!("config.yml already exists."))
        .and_then(|mut path| {
            path.push("/.cdd/openapi.yml");
            Ok(path)
        })?;

    if !spec_path.exists() {
        return Err(failure::format_err!(
            "Could not find {}",
            spec_path.to_str().unwrap_or("")
        ));
    };

    let _ = util::copy_file(spec_path, PathBuf::from("./openapi.yml"));
    info!("Copied OpenAPI spec to ./openapi.yml");

    Ok(())
}
