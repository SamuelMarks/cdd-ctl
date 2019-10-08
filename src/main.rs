use log::*;

fn main() {
    let result = cdd::run();
    match result {
        Ok(_) => (),
        Err(err) => error!("{}", err),
    };
}
