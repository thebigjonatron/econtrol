mod ec;
mod config_value;

fn main() {

    let ec = ec::EmbeddedController::new();
    ec.write(1,10)
}


