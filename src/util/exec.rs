use crate::*;
use std::process::{Command, ExitStatus};

pub fn exec(cmd: &str, args: Vec<&str>) -> CliResult<String> {
    let output = Command::new(cmd).args(&args).output()?;

    match output.status.success() {
        true => Ok(String::from_utf8(output.stdout)?),
        false => Err(failure::format_err!(
            "{}",
            String::from_utf8(output.stderr)?
        )),
    }
}
