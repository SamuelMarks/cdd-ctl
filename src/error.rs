use failure::{Context, Error, Fail};

pub type CliResult<T> = Result<T, Error>;

#[derive(Debug, Fail)]
pub enum CliError {
    #[fail(display = "error initialising new project: {}", arg)]
    InitError { arg: String },

    #[fail(display = "invalid argument: {}", arg)]
    InvalidArgument { arg: String },

    #[fail(display = "reading config: {}", msg)]
    InvalidConfig { msg: String },

    #[fail(display = "IO error: {}", error)]
    IoError { error: std::io::Error },

    #[fail(display = "An unknown error has occurred.")]
    UnknownError,
}

// impl std::convert::From<std::io::Error> for CliError {
//     fn from(error: io::Error) -> Self {
//         CliError::IoError(error)
//     }
// }
