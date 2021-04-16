use std::env::args;
use std::error::Error;
use std::sync::mpsc;
use std::thread::JoinHandle;
use std::sync::mpsc::Sender;
use rust_gpiozero::DigitalInputDevice;

mod conf;
mod handlers;
mod util;

/**
 * Arguments application:
 * app dio_pin_tm clk_pin_tm brightness button_pin
*/
fn main() -> Result<(), Box<dyn Error>> {
    let (button_sender, tm1637_receiver) = mpsc::channel();
    let config: conf::RaspianConfig =
        serde_json::from_str(&args().nth(1).expect("Give in a json configuration."))?;
    let tm_handler: JoinHandle<Result<(), &'static str>> = handlers::four_digit::get_tm_1637_thread(
        config.get_dio_pin(),
        config.get_clk_pin(),
        config.get_brightness()?,
        tm1637_receiver,
    );
    let mut button = DigitalInputDevice::new(config.get_btn_pin();
    loop {
        button.wait_for_active(None);
        tx.send(()).unwrap();
    }
    tm_handler.join().unwrap()?;

    Ok(())
}
