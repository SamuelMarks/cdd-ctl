use crate::*;
use log::*;
use std::path::PathBuf;

pub fn sync() -> CliResult<()> {
    // load config
    let config = load_config_file()?;
    let spec = load_openapi_spec()?;
    // load spec
    // ensure all projects are in place
    // collect model tree using adaptors
    // sync trees into an instruction graph
    // execute graph
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
        return Err(failure::format_err!("{}", spec_path.to_str().unwrap_or("")));
    };

    let _ = util::copy_file(spec_path, PathBuf::from("./openapi.yml"));
    info!("Copied OpenAPI spec to ./openapi.yml");

    Ok(())
}
