use crate::config::Config;
use crate::error::CliResult;
use structopt::StructOpt;

/// Compiler driven development cli tool
#[derive(StructOpt, Debug)]
#[structopt(raw(setting = "structopt::clap::AppSettings::ColoredHelp"))]
#[structopt(name = "cdd")]
struct Opt {
    // The number of occurrences of the `v/verbose` flag
    /// Verbose mode (-v, -vv, -vvv, etc.)
    #[structopt(short = "v", long = "verbose", parse(from_occurrences))]
    verbose: u8,

    /// Optionally specifies a config file location.
    #[structopt(help = "Config file location")]
    config: Option<String>,

    /// Initialises a new configuration file.
    #[structopt(help = "Initialise a new project")]
    init: bool,
}

pub fn run() -> CliResult<String> {
    let opt = Opt::from_args();
    let config = Config::read(opt.config);

    Ok(format!("{:?}", config))
}
