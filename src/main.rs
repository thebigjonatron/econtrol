mod ec;
mod config_value;

fn main() {
    let ec = ec::EmbeddedController::new();
    ec.print();
    ec.print_key_values();

}


