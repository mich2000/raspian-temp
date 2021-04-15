use std::env::args;
use rust_gpiozero::DigitalInputDevice;
use std::time::Duration;

fn main() {
    let pin_number = args().next().unwrap();
    let pin_number = pin_number.parse::<u16>().unwrap();
    let mut device = DigitalInputDevice::new(pin_number);
    loop {
        device.wait_for_active(Some(Duration::from_secs(5)));
        println!("touch");
    }
}