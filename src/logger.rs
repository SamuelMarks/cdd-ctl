use simplelog::*;
use crate::error::*;

pub(crate) fn start_logger(level: u8, file_printing: bool) -> CliResult<()> {
    
    let logger = match level {
        0 => TermLogger::new(LevelFilter::Error, Config::default(), TerminalMode::Mixed).unwrap(),
        1 => TermLogger::new(LevelFilter::Warn, Config::default(), TerminalMode::Mixed).unwrap(),
        _ => TermLogger::new(LevelFilter::Info, Config::default(), TerminalMode::Mixed).unwrap(),
    };

    CombinedLogger::init(
        vec![
            logger,
            // WriteLogger::new(LevelFilter::Info, Config::default(), File::create("my_rust_binary.log").unwrap()),
        ]
    )?;

    Ok(())
}
