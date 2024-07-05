use std::collections::HashMap;
use std::net::SocketAddr;
use std::time::{SystemTime, UNIX_EPOCH};
use tokio_modbus::client::tcp;
use tokio_modbus::prelude::*;
use tokio_modbus::client::Context as ModbusContext;
use serde_json::json;
use log::{error, info};
use std::error::Error;
mod mgw_config;
use mgw_config::{Config, RegisterConfig, ChunkConfig};



#[derive(Debug)]
struct Register {
    address: u16,
}

#[derive(Debug)]
struct Janitza604 {
    context: ModbusContext,
    registers: Vec<Register>,
}

impl Janitza604 {
    pub async fn new(ip: &str, port: u16) -> Result<Self, Box<dyn std::error::Error>> {
        let socket_addr: SocketAddr = format!("{}:{}", ip, port).parse()?;
        println!("Connecting to {}:{}", ip, port);
        let context = tcp::connect(socket_addr).await?;
        println!("Connected to {}:{}", ip, port);
        let registers = vec![
            Register { address: 19000 }, // voltage_L1_N
            Register { address: 19002 }, // voltage_L2_N
            Register { address: 19004 }, // voltage_L3_N
            Register { address: 19012 }, // current_L1_N
            Register { address: 19014 }, // current_L2_N
            Register { address: 19016 }, // current_L3_N
            Register { address: 19020 }, // active_power_L1_N
            Register { address: 19022 }, // active_power_L2_N
            Register { address: 19024 }, // active_power_L3_N
            Register { address: 19036 }, // reactive_power_L1_N
            Register { address: 19038 }, // reactive_power_L2_N
            Register { address: 19040 }, // reactive_power_L3_N
            Register { address: 19044 }, // power_factor_L1
            Register { address: 19046 }, // power_factor_L2
            Register { address: 19048 }, // power_factor_L3
            Register { address: 19050 }, // grid_freq
            Register { address: 19018 }, // total_current
            Register { address: 19026 }, // total_active_power
            Register { address: 19034 }, // total_apparent_power
            Register { address: 19042 }, // total_reactive_power
        ];
        Ok(Janitza604 { context, registers })
    }


    pub async fn read_register(&mut self, register: &Register) -> Result<f32, Box<dyn std::error::Error>> {
        let response = self.context.read_holding_registers(register.address, 2).await?;
        let response_vec: Vec<u16> = response?;
        let value = f32::from_bits(((response_vec[0] as u32) << 16) | (response_vec[1] as u32));
        
        // Print the register comment and its value
        match register.address {
            19000 => println!("Voltage L1-N: {:.2} V", value),
            19002 => println!("Voltage L2-N: {:.2} V", value),
            19004 => println!("Voltage L3-N: {:.2} V", value),
            19012 => println!("Current L1-N: {:.2} A", value),
            19014 => println!("Current L2-N: {:.2} A", value),
            19016 => println!("Current L3-N: {:.2} A", value),
            19020 => println!("Active Power L1-N: {:.2} W", value),
            19022 => println!("Active Power L2-N: {:.2} W", value),
            19024 => println!("Active Power L3-N: {:.2} W", value),
            19036 => println!("Reactive Power L1-N: {:.2} VAR", value),
            19038 => println!("Reactive Power L2-N: {:.2} VAR", value),
            19040 => println!("Reactive Power L3-N: {:.2} VAR", value),
            19044 => println!("Power Factor L1: {:.2}", value),
            19046 => println!("Power Factor L2: {:.2}", value),
            19048 => println!("Power Factor L3: {:.2}", value),
            19050 => println!("Grid Frequency: {:.2} Hz", value),
            19018 => println!("Total Current: {:.2} A", value),
            19026 => println!("Total Active Power: {:.2} W", value),
            19034 => println!("Total Apparent Power: {:.2} VA", value),
            19042 => println!("Total Reactive Power: {:.2} VAR", value),
            _ => println!("Unknown register address: {} with value: {:.2}", register.address, value),
        }
        
        Ok(value)
    }
    

    pub async fn read_register_chunk(&mut self, start_address: u16, count: u16) -> Result<Vec<f32>, Box<dyn std::error::Error>> {
        let response = self.context.read_holding_registers(start_address, count).await?;
        let response_vec: Vec<u16> = response?;
        
        // Parse the response into individual floating point values
        let mut values = Vec::new();
        for i in 0..(response_vec.len() / 2) {
            let value = f32::from_bits(((response_vec[i * 2] as u32) << 16) | (response_vec[i * 2 + 1] as u32));
            values.push(value);
        }
        
        Ok(values)
    }
   

    pub async fn read(&mut self) -> Result<serde_json::Value, Box<dyn std::error::Error>> {
        let mut payload = HashMap::new();
        
        // Read voltage L1-N to L3-N registers (19000 to 19004)
        let voltage_registers = self.read_register_chunk(19000, 6).await?;
        for (i, voltage) in voltage_registers.iter().enumerate() {
            let address: u16 = 19000 + (i as u16) * 2;
            println!("Voltage L{}-N: {:.2} V", i + 1, *voltage);
            let key = format!("register_{}", address);
            payload.insert(key, *voltage);
        }

        // Read current L1-N, L2-N, L3-N registers (19012, 19014, 19016)
        let current_registers = self.read_register_chunk(19012, 6).await?;
        for (i, current) in current_registers.iter().enumerate() {
            let address: u16 = 19012 + (i as u16) * 2;
            println!("Current L{}-N: {:.2} A", i + 1, *current);
            let key = format!("register_{}", address);
            payload.insert(key, *current);
    }

        
        // Read other registers as before
        let register_addresses: Vec<u16> = self.registers.iter().map(|r| r.address).collect();
        for address in register_addresses {

            if (19000..=19004).contains(&address) {
                continue; // Skip the registers we already read
            }
            if (19012..=19016).contains(&address) {
                continue; // Skip the registers we already read
            }
            let register = Register { address };
            let value = self.read_register(&register).await?;
            let key = format!("register_{}", address);
            println!("Inserting value {} for key {}", value, key);
            payload.insert(key, value);
        }
        
        let timestamp = SystemTime::now().duration_since(UNIX_EPOCH)?.as_secs();
        println!("Current timestamp: {}", timestamp);
        payload.insert("meter_values_timestamp".to_string(), timestamp as f32);
        Ok(json!(payload))
    }
    



}

#[tokio::main]
async fn main() {
    env_logger::init();
    println!("Starting the application...");

    // Load configuration from config.yaml
    let config: Result<Config, Box<dyn Error>> = Config::from_file("mgw_config.yaml");
    config.unwrap().print_config(); // Call print_config on the loaded Config instance

    let mut meter: Janitza604 = match Janitza604::new("10.15.1.17", 502).await {
        Ok(meter) => {
            println!("Meter created successfully.");
            meter
        },
        Err(err) => {
            error!("Failed to create meter: {}", err);
            println!("Error: Failed to create meter: {}", err);
            return;
        },
    };

    match meter.read().await {
        Ok(payload) => {
            info!("Readings: {:?}", payload);
            println!("Readings: {:?}", payload);
        },
        Err(err) => {
            error!("Failed to read meter: {}", err);
            println!("Error: Failed to read meter: {}", err);
        },
    }
}
