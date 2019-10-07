use failure::Fail;

pub type CliResult<T> = Result<T, failure::Error>;

#[derive(Debug, Fail)]
pub enum CliError {
    #[fail(display = "invalid argument: {}", arg)]
    InvalidArgument { arg: String },
    #[fail(display = "reading config: {}", msg)]
    InvalidConfig { msg: String },
}

// use core::fmt;
// use std::error::Error;

// pub type CliResult<T> = Result<T, Box<dyn std::error::Error>>;

// #[derive(Debug, Clone)]
// pub enum CliError {
//     ArgumentError(String),
//     ConfigError(String),
// }

// impl fmt::Display for CliError {
//     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//         match self.clone() {
//             CliError::ArgumentError(why) => write!(f, "ArgumentError: {}", why),
//             CliError::ConfigError(why) => write!(f, "ConfigError: {}", why),
//         }
//     }
// }

// impl Error for CliError {
//     fn description(&self) -> &str {
//         "Cli Error."
//     }
// }
