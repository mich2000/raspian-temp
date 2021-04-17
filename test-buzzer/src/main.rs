use rppal::gpio::Gpio;
use std::time::Duration;
use std::error::Error;
use std::thread;

fn main() -> Result<(), Box<dyn Error>> {
    let mut pin = Gpio::new()?.get(2)?.into_output();

    loop {
        pin.toggle();
        thread::sleep(Duration::from_millis(500));
    }
}
