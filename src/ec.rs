use std::fs::{File, OpenOptions};
use std::fs;
use std::io::{Error, Result, Seek, SeekFrom, Read, Write};
use serde::Deserialize;

use crate::config_value::*;

const EC_FILE: &str = "/dev/ec";
const CFG_FILE: &str = "./config.toml";

pub struct EmbeddedController {
    contents: Vec<u8>,
    config: Config
}

impl EmbeddedController {

    //Create new embedded controller
    pub fn new() -> EmbeddedController {
        EmbeddedController {
            contents: fs::read(EC_FILE).expect("Can't read /dev/ec"),
            config: toml::from_str::<Config>(
                &(fs::read_to_string(CFG_FILE).expect("Can't read config file")))
                .unwrap()
        }
    }

    pub fn refresh_contents(&mut self){
        self.contents = fs::read(EC_FILE).expect("Can't read /dev/ec");
    }

    //Dumps content of ec
    pub fn dump(&self, index: usize){
        print!("{:02X}", self.contents[index])
    }

    /*
    Writes a byte to the given position.
     */
    pub fn write(&self, byte: u8, position: u8){
        let mut file = OpenOptions::new()
            .write(true)
            .read(false)
            .create(false)
            .open(EC_FILE).unwrap();
        file.seek(SeekFrom::Start(position as u64)).unwrap();
        file.write_all(&[byte]).unwrap();
    }

    // Prints content of ec
    pub fn print(&self) {
        print!("Contents of {}: \n", EC_FILE);
        print!("      ");
        for col in 0..16 {
            print!("{:02X} ", col);
        }
        print!("   ");
        for col in 0..16 {
            print!("{:02}  ", col);
        }
        println!();
        for row in 0..16 {
            print!("{}{:04X}: ", "\x1B[37m", row * 16);
            for col in 0..16 {
                let index = row * 16 + col;
                if index < self.contents.len() {
                    print!("{}{:02X} ", self.check_index(index as u8), self.contents[index]);
                }
                else {
                    print!("    ");
                }
            }
            print!("  ");
            for col in 0..16 {
                let index = row * 16 + col;
                if index < self.contents.len() {
                    print!("{}{:3} ", self.check_index(index as u8),self.contents[index]);
                }
                else {
                    print!("   ");
                }
            }
            println!();
        }
    }

    pub fn print_key_values(&self){
        print!("\n");
        print!("CPU temp : {} \n", self.contents[self.config.cpu().realtime_cpu_temperature() as usize]);
        print!("CPU fan speed : {} \n", self.contents[self.config.cpu().realtime_cpu_fanspeed() as usize]);
        print!("GPU temp : {} \n", self.contents[self.config.gpu().realtime_gpu_temperature() as usize]);
        print!("GPU fan speed : {} \n", self.contents[self.config.gpu().realtime_gpu_fanspeed() as usize]);
    }


    fn check_index (&self, position: u8) -> &str{
        let addresses = self.config.addresses();
        for element in &addresses {
            if element == &position {
                return "\x1B[31m";
            }
        }
        return "\x1B[37m";
    }

}
