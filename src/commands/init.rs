use crate::*;
use config::*;
use log::*;
use std::path::PathBuf;
use util::*;

pub fn init(_path: PathBuf) -> CliResult<()> {
    let config_path = PathBuf::from("./config.yml");

    if config_path.exists() {
        return Err(failure::format_err!("config.yml already exists."));
    };

    // let config = Config::read(config_file)?;
    // info!("Read config file at ./config.yml");

    let config = Config::default();
    config.write(config_path);
    info!("Wrote default config file to ./config.yml");

    Ok(())
}
