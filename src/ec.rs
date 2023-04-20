use std::fs::{File, OpenOptions};
use std::fs;
use std::io::{Error, Result, Seek, SeekFrom, Read};
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

    pub fn refresh_contents(&mut self){
        self.contents = fs::read(EC_FILE).expect("Can't read /dev/ec");
    }

    //Dumps content of ec
    pub fn dump(&self, index: usize){
        print!("{:02X}", self.contents[index])
    }

    /*
    Writes a single byte to the given position.
     */
    pub fn single_write(&self, byte: u8, position: u8){
        match Self::open() {
            Ok(mut file) => {
                file.seek(SeekFrom::Start(10)).unwrap();
            },
            Err(error) => {
                panic!("Can't open /dev/ec, {}", error)
            }

        }
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


    //POC RE_WRITE
    fn check_index (&self, position: u8) -> &str{
        let mut a = self.config.cpu().addresses();
        let mut b = self.config.battery_charging_threshold().addresses();
        let mut c = self.config.gpu().addresses();
        let mut d = self.config.battery_charging_threshold().addresses();
        let mut e = self.config.coolerboost().addresses();
        let mut f = self.config.threashold_cpu_temperature_range().addresses();
        let mut g = self.config.threashold_gpu_temperature_range().addresses();
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

    fn open()-> Result<File>{
        let file = OpenOptions::new()
            .write(true)
            .create(false)
            .open(CFG_FILE)?; //Should unwrap ?
        Ok(file)
    }

}
