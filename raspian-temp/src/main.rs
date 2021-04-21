use error::RaspianError;
use rust_gpiozero::DigitalInputDevice;
use std::env::args;
use std::error::Error;
use std::sync::mpsc;
use std::thread::JoinHandle;

mod conf;
mod error;
mod four_digit;
mod util;

fn main() -> Result<(), Box<dyn Error>> {
    let (button_sender, tm1637_receiver) = mpsc::sync_channel(0);
    let config: conf::RaspianConfig =
        serde_json::from_str(&args().nth(1).ok_or(RaspianError::JsonConfigFaulthy)?)
            .or(Err(RaspianError::JsonParsingFailed))?;
    let tm_handler: JoinHandle<Result<(), RaspianError>> = four_digit::get_tm_1637_thread(
        config.get_dio_pin(),
        config.get_clk_pin(),
        config.get_brightness()?,
        tm1637_receiver,
    );
    let mut button = DigitalInputDevice::new(config.get_btn_pin());
    loop {
        button.wait_for_active(None);
        button_sender.send(()).unwrap();
    }
    tm_handler.join().unwrap()?;

    Ok(())
}
