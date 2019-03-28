use crate::error::CliError;
use crate::error::CliResult;

pub fn run() -> CliResult<String> {
    // Ok("done.".to_string())
    Err(Box::new(CliError::ArgumentError("not found".to_string())))
}
