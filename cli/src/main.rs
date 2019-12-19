use cdd;
use log::*;

fn main() {
    if let Err(err) = cdd::run() {
        for line in format!("{}", err).lines() {
            error!("[CDD] {}", line);
        }
    } else {
        info!("Sync successful");
    }
}