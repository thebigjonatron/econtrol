mod ec;
mod config_value;

fn main() {
    let ec = ec::EmbeddedController::new();
    ec.read_config_file();
}


