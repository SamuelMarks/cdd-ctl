use log::*;

fn main() {
    let result = cdd::run();
    match result {
        Ok(msg) => info!("{}", msg),
        Err(err) => error!("{}", err),
    };
}
