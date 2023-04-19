use std::fs;

const EC_FILE: &str = "/dev/ec";

pub struct EmbeddedController {
    contents: Vec<u8>
}

impl EmbeddedController {

    //Create new embedded controller
    pub fn new() -> EmbeddedController {
        EmbeddedController {
            contents: fs::read(EC_FILE).expect("Could not read file"),
        }
    }

    // Dumps content of ec
    pub fn dump (&self) {
        print!("Contents of {}: \n", EC_FILE);
        print!("      ");
        for col in 0..16 {
            print!("{:02X} ", col);
        }
        println!();
        for row in 0..16 {
            print!("{:04X}: ", row * 16);
            for col in 0..16 {
                let index = row * 16 + col;
                if index < self.contents.len() {
                    print!("{:02X} ", self.contents[index]);
                } else {
                    print!("   ");
                }
            }
            println!();
        }
    }
}
