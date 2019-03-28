use crate::error::CliResult;
use structopt::StructOpt;

/// Compiler driven development cli tool
#[derive(StructOpt, Debug)]
#[structopt(name = "cdd")]
struct Opt {
    // The number of occurrences of the `v/verbose` flag
    /// Verbose mode (-v, -vv, -vvv, etc.)
    #[structopt(short = "v", long = "verbose", parse(from_occurrences))]
    verbose: u8,
}

pub fn run() -> CliResult<String> {
    let opt = Opt::from_args();

    Ok(format!("{:?}", opt))
}
