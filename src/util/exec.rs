use crate::*;
use log::*;
use std::process::Command;

pub fn exec(cmd: &str, args: Vec<&str>) -> CliResult<String> {
    info!("CMD: {} {}", cmd, args.join(" "));

    let output = Command::new(cmd).args(&args).output()?;
    let stdout = String::from_utf8(output.stdout)?;
    let stderr = String::from_utf8(output.stderr)?;

    for line in stdout.lines() {
        info!("[{}] {}", cmd, line);
    }

    for line in stderr.lines() {
        error!("[{}] {}", cmd, line);
    }

    match output.status.success() {
        true => Ok(stdout),
        false => Err(failure::format_err!("{}", stderr)),
    }
}
