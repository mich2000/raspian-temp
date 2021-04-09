use rppal::i2c::I2c;
use std::error::Error;
use std::time::Duration;
use std::thread::sleep;

const ADDR_GROVE_TEMP: u16 = 0x44;

fn main() -> Result<(), Box<dyn Error>> {
    println!("Hello, world!");
    let mut bus = I2c::with_bus(1)?;
    bus.set_slave_address(0x44)?;
    bus.smbus_block_read(0x2c, [0x06])?;
    sleep(Duration::from_micros(500_000));
    let mut bytes = Vec::new();
    bus.smbus_block_read(0x44, &bytes)?;
    Ok(())
}
