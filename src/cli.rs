use clap::Parser;
use crate::ec::EmbeddedController;

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

pub fn parse_args(){
    let args = Arguments::parse();
    print!("{:?}", args);
    if args.coolerbooster {
        print!("he");
        let controller = EmbeddedController::new();
        controller.coolerbooster()

    }
}
