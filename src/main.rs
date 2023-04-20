mod ec;
mod config_value;

fn main() {
    let ec = ec::EmbeddedController::new();
    ec.single_write(1,1)
}


