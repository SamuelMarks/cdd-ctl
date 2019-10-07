use crate::*;
mod init;
pub use init::*;

pub fn regenerate() -> CliResult<String> {
    Ok("regenerate".to_string())
}

pub fn sync() -> CliResult<String> {
    Ok("sync".to_string())
}
