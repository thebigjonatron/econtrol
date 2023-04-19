use std::fs;
use toml::Table;
use toml::Value;
use serde::Deserialize;
use crate::config_value::*;

const EC_FILE: &str = "/dev/ec";
const CFG_FILE: &str = "./config.toml";

pub struct EmbeddedController {
    contents: Vec<u8>
}

impl EmbeddedController {

    //Create new embedded controller
    pub fn new() -> EmbeddedController {
        EmbeddedController {
            contents: fs::read(EC_FILE).expect("Can't read /dev/ec"),
        }
    }

    // Dumps content of ec
    pub fn print (&self) {
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

    //Dumps content of ec
    pub fn dump(&self){
        for byte in &self.contents{
            print!("{:02x} \n", byte)
        }
    }

    pub fn read_config_file(&self){
        let config_file = fs::read_to_string(CFG_FILE).expect("Can't read config file");
        let config = toml::from_str::<Config>(&config_file).unwrap();
        let realcpu = config.cpu().realtime_cpu_temperature();
        println!("{}",realcpu)
    }

}
