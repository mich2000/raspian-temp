use rust_gpiozero::DigitalInputDevice;
use std::env::args;
use std::error::Error;
use std::sync::mpsc;
use std::sync::mpsc::Sender;
use std::thread::JoinHandle;

mod conf;
mod four_digit;
mod util;

/**
 * Arguments application:
 * app dio_pin_tm clk_pin_tm brightness button_pin
*/
fn main() -> Result<(), Box<dyn Error>> {
    let config: conf::RaspianConfig =
        serde_json::from_str(&args().nth(1).expect("Give in a json configuration."))?;
    let (button_sender, tm1637_receiver) = mpsc::channel();
    let mut button = DigitalInputDevice::new(config.get_btn_pin());
    loop {
        button.wait_for_active(None);
        tx.send(()).unwrap();
    }
    four_digit::get_tm_1637_thread(
        config.get_dio_pin(),
        config.get_clk_pin(),
        config.get_brightness()?,
        tm1637_receiver,
    )
    .join()
    .unwrap()?;

    Ok(())
}
