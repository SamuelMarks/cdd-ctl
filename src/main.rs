use log::*;

fn main() {
    if let Err(err) = cdd::run() {
        for line in format!("{}", err).lines() {
            error!("{}", line)
        }
    }
}
