mod ec;
mod config_value;
mod cli;

use std::fs::{File, OpenOptions};
use std::fs;

fn main() {
    //let a = fs::read("/dev/ec").expect("Can't read /dev/ec");
    let a = ec::EmbeddedController::new();
    //cli::parse_args();
}

