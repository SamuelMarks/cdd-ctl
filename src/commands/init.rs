// use std::path::PathBuf;

use log::*;

// initialise a new project with a configuration file.
// pub fn init(_path: PathBuf) {}
use crate::*;

pub fn init() -> CliResult<String> {
    Ok("init".to_string())
}
