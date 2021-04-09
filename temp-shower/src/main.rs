use ruspiro_i2c::I2C;
use std::error::Error;
use std::time::Duration;
use std::thread::sleep;

use i2c_linux::I2c;

const ADDR_GROVE_TEMP: u16 = 0x44;

fn main() -> Result<(), Box<dyn Error>> {
    let mut i2c = I2c::from_path("/dev/i2c-1")?;
    i2c.smbus_set_slave_address(ADDR_GROVE_TEMP, false)?;
    let data = i2c.smbus_read_byte()?;
    println!("Read I2C data: {}", data);
    Ok(())
}
