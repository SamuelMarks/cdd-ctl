use crate::*;
use log::*;
use std::process::Command;

pub fn exec(cmd: &str, args: Vec<&str>) -> CliResult<String> {
    info!("CMD: {:?} {:?}", cmd, args);

    let output = Command::new(cmd).args(&args).output()?;

    match output.status.success() {
        true => Ok(String::from_utf8(output.stdout)?),
        false => Err(failure::format_err!(
            "{}",
            String::from_utf8(output.stderr)?
        )),
    }
}
