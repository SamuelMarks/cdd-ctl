use crate::config::Config;
use crate::CliResult;
use std::path::PathBuf;
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
enum Command {
    #[structopt(name = "init", about = "Initializes a new project configuration file and OpenAPI spec")]
    Init,

    #[structopt(name = "regenerate", about = "Regenerates templates (warning: overwrites existing templates)")]
    Regenerate,

    #[structopt(name = "sync", about = "Syncs CDD projects using language-specific adaptors")]
    Sync,
}

/// Compiler driven development cli tool
#[derive(StructOpt, Debug)]
#[structopt(raw(setting = "structopt::clap::AppSettings::ColoredHelp"))]
#[structopt(name = "cdd")]
struct Opt {

    #[structopt(subcommand)]
    cmd: Command,

    // The number of occurrences of the `v/verbose` flag
    /// Verbose mode (-v, -vv, -vvv, etc.)
    #[structopt(short = "v", long = "verbose", parse(from_occurrences))]
    verbose: u8,

    /// Optionally specifies a config file location.
    #[structopt(help = "Config file location", parse(from_os_str), name = "config")]
    config: Option<PathBuf>,
}

pub fn run() -> CliResult<String> {
    let opt = Opt::from_args();
    // if opt.init {
    //     println!("initialising new config...");
    //     Config::default().write(PathBuf::from(r"./config.yaml"))?;
    // }

    let config = Config::read(opt.config);

    match opt.cmd {
        Command::Init => crate::commands::init(),
        Command::Regenerate => crate::commands::regenerate(),
        Command::Sync => crate::commands::sync(),
    }
}
