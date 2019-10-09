use crate::*;
use config::*;
use log::*;
use std::path::PathBuf;

pub fn init(_path: PathBuf) -> CliResult<()> {
    let _ = init_config_file()?;
    let _ = copy_openapi_spec()?;

    Ok(())
}

fn init_config_file() -> CliResult<()> {
    let config_path = PathBuf::from("./config.yml");

    if config_path.exists() {
        return Err(failure::format_err!("config.yml already exists."));
    };

    let config = Config::default();
    config.write(config_path)?;
    info!("Wrote default config file to ./config.yml");

    Ok(())
}

fn copy_openapi_spec() -> CliResult<()> {
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
