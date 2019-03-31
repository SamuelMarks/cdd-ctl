use crate::config::Config;
use crate::error::CliResult;
use std::path::PathBuf;
use structopt::StructOpt;

// #[derive(StructOpt, Debug)]
// #[structopt(name = "command")]
// enum Command {
//     #[structopt(name = "init")]
//     Init,
// }

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
    #[structopt(help = "Config file location", parse(from_os_str), name = "config")]
    config: Option<PathBuf>,

    // /// Main set of command directives.
    // #[structopt(name = "command")]
    // command: Command,
    /// Initialises a new configuration file.
    #[structopt(short = "i", long = "init", help = "Initialise a new project")]
    init: bool,
}

pub fn run() -> CliResult<String> {
    let opt = Opt::from_args();
    if opt.init {
        println!("initialising new config...");
        Config::default().write(PathBuf::from(r"./config.yaml"))?;
    }

    let config = Config::read(opt.config);

    Ok(format!("{:?}", config))
}
