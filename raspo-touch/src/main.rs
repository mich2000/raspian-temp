use std::env::args;
use rust_gpiozero::DigitalInputDevice;
use std::time::Duration;

fn main() {
    let pin_number = args().nth(1).unwrap();
    let pin_number = pin_number.parse::<u8>().unwrap();
    let mut device = DigitalInputDevice::new(pin_number);
    loop {
        device.wait_for_active(Some(50.0));
        println!("touch");
    }
}
