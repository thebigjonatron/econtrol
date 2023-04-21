mod ec;
mod config_value;

use clap::Parser;

#[derive(Parser,Default,Debug)]
#[clap(author="Author Name", version)]
/// econtrol is a utility tool to read and write to embedded controller for msi laptops.
struct Arguments {
    #[clap(short, long)]
    /// This is the package name
    package_name: Option<String>,
    #[clap(short, long)]
    /// Enable or disable coolerbooster
    coolerbooster: bool,
}

fn main() {
    let args = Arguments::parse();
    println!("{:?}", args);
    let ec = ec::EmbeddedController::new();
}


