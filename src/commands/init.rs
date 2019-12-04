use crate::*;
use config::*;
use log::*;
use std::path::PathBuf;

pub fn init(name: &str) -> CliResult<()> {
    let _ = init_config_file(name)?;
    let _ = copy_openapi_spec()?;

    Ok(())
}

fn init_config_file(name: &str) -> CliResult<()> {
    let config_path = PathBuf::from("./config.yml");

    if config_path.exists() {
        return Err(failure::format_err!("config.yml already exists."));
    };

    let config = Config::new(name);
    config.write(config_path)?;
    info!("Wrote default config file to ./config.yml");

    Ok(())
}

fn copy_openapi_spec() -> CliResult<()> {
    let spec_path: PathBuf = dirs::home_dir()
        .ok_or(failure::format_err!(
            "There was a problem locating your home directory."
        ))
        .map(|path| path.join(".cdd/openapi.yml"))?;

    if !spec_path.exists() {
        return Err(failure::format_err!(
            "Could not find spec file at ~{}",
            spec_path.to_str().unwrap_or("")
        ));
    };

    let _ = util::copy_file(spec_path, PathBuf::from("./openapi.yml"));
    info!("Copied OpenAPI spec to ./openapi.yml");

    Ok(())
}
