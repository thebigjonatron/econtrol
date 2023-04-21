mod ec;
mod config_value;

use clap::Parser;

#[derive(Parser,Default,Debug)]
#[clap(author="Author Name", version)]
/// econtrol is a utility tool to read and write to embedded controller for msi laptops.
struct Arguments {
    #[clap(short, long)]
    /// Enable or disable cooler booster
    coolerbooster: bool,
    #[clap(short, long)]
    /// Print contents of embedded controller
    print: bool,
    #[clap(short, long)]
    /// Switch fn and windows key
    switchfn: bool,
    #[clap(short, long)]
    /// Write to embedded controller
    write: bool,
}

fn main() {
    let args = Arguments::parse();
    println!("{:?}", args);
    let ec = ec::EmbeddedController::new();
}


