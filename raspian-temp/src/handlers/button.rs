use std::thread::{self, JoinHandle};

use std::sync::mpsc::Sender;

use rust_gpiozero::*;

pub fn get_button_thread_handler(
    btn_pin: u8,
    tx: Sender<()>,
) -> JoinHandle<Result<(), &'static str>> {
    thread::spawn(move || {
        let mut button = DigitalInputDevice::new(btn_pin);
        loop {
            button.wait_for_active(None);
            tx.send(()).unwrap();
        }
    })
}
