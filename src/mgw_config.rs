use serde::Deserialize;
use std::fs::File;
use std::io::BufReader;
use std::error::Error;

#[derive(Debug, Deserialize)]
pub struct RegisterConfig {
    pub name: String,
    pub address: u16,
}

#[derive(Debug, Deserialize, Clone)]
pub struct ChunkConfig {
    pub name: String,
    pub start_address: u16,
    pub count: u16,
}

#[derive(Debug, Deserialize)]
pub struct Config {
    pub registers: Vec<RegisterConfig>,
    pub chunks: Vec<ChunkConfig>,
}

impl Config {
    pub fn from_file(file_path: &str) -> Result<Self, Box<dyn Error>> {
        let file = File::open(file_path)?;
        let reader = BufReader::new(file);
        let config: Config = serde_yaml::from_reader(reader)?;
        Ok(config)
    }

    pub fn print_config(&self) {
        println!("Configuration:");
        println!("Registers:");
        for register in &self.registers {
            println!("  Name: {}, Address: {}", register.name, register.address);
        }
        println!("Chunks:");
        for chunk in &self.chunks {
            println!("  Name: {}, Start Address: {}, Count: {}", chunk.name, chunk.start_address, chunk.count);
        }
    }
}
