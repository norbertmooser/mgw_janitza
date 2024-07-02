use std::net::ToSocketAddrs;

use anyhow::{anyhow, Context, Result};
use tokio_modbus::client::tcp;
use tokio_modbus::client::Context as ModbusContext;
use tokio_modbus::prelude::*;

#[tokio::main]
async fn main() -> Result<()> {
    // Define the IP address and port of the Modbus device
    let ip_address = "10.15.1.17";
    let port = 502;

    // Create the address in the form of "ip:port"
    let address = format!("{}:{}", ip_address, port);
    println!("Attempting to connect to Modbus server at {}", address);

    // Resolve the address
    let socket_addr = address.to_socket_addrs()?.next().ok_or_else(|| anyhow!("Invalid address"))?;

    // Connect to the Modbus server using the Modbus TCP client
    let mut ctx = tcp::connect(socket_addr)
        .await
        .context("Could not connect to Modbus device")?;

    // Perform a simple read to test the connection
    let operation_res = read_registers(&mut ctx).await;

    // Disconnect the client
    let disconnect_res = ctx.disconnect().await.context("Could not disconnect Modbus client");

    // Combine the results of the operation and disconnection
    disconnect_res.and(operation_res)
}

async fn read_registers(ctx: &mut ModbusContext) -> Result<()> {
    let registers = ctx
        .read_holding_registers(0, 1)
        .await
        .context("Failed to read holding registers")?;
    
    println!("Read registers: {:?}", registers);
    Ok(())
}