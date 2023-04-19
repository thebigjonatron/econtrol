use std::fs;
use toml::Table;
use toml::Value;
use serde::Deserialize;
use log::{info,trace};

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

    // Prints content of ec
    pub fn print(&self) {
        print!("Contents of {}: \n", EC_FILE);
        print!("      ");
        for col in 0..16 {
            print!("{:02X} ", col);
        }
        println!();
        for row in 0..16 {
            print!("{}{:04X}: ", "\x1B[37m", row * 16);
            for col in 0..16 {
                let index = row * 16 + col;
                if index < self.contents.len() {
                    print!("{}{:02X} ", EmbeddedController::check_index(self, index as u8) ,self.contents[index]);
                }
                else {
                    print!("    ");
                }
            }
            println!();
        }
    }


    //POC RE_WRITE
    fn check_index (&self, position: u8) -> &str{
        let mut a = self.config.cpu().addresses();
        let mut b = self.config.battery_charging_threshold().addresses();
        let mut c = self.config.gpu().addresses();
        let mut d = self.config.battery_charging_threshold().addresses();
        let mut e = self.config.coolerboost().addresses();
        let mut f = self.config.threashold_cpu_temperature_range().addresses();
        let mut g =self.config.threashold_gpu_temperature_range().addresses();
        let mut h = self.config.fan_mode().addresses();
        let mut i = self.config.fan_preset().addresses();
        let mut j = self.config.gpu_fanspeed_range().addresses();
        let mut k = self.config.cpu_fanspeed_range().addresses();
        a.extend(&b);
        a.extend(&c);
        a.extend(&d);
        a.extend(&e);
        a.extend(&f);
        a.extend(&g);
        a.extend(&h);
        a.extend(&i);
        a.extend(&j);
        a.extend(&k);

        for element in &a {
            if element == &position {
                return "\x1B[31m";
            }
        }
        return "\x1B[37m";
    }

    //Dumps content of ec
    pub fn dump(&self){
        for byte in &self.contents{
            print!("{:02x} \n", byte)
        }
    }



}
