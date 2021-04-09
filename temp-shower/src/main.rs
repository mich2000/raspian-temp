use ruspiro_i2c::I2C;
use std::error::Error;
use std::time::Duration;
use std::thread::sleep;

const ADDR_GROVE_TEMP: u16 = 0x44;

fn main() -> Result<(), Box<dyn Error>> {
    let device_addr = 0x68;
    I2C.take_for(|i2c| {
        if i2c.check_device(device_addr).is_ok() {
            println!("tm1637 is okay");
        }
    });
    Ok(())
}
