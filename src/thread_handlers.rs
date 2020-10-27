use std::time::Duration;
use std::thread::{ self, JoinHandle, sleep};

use tm1637_gpio_driver::gpio_api::setup_gpio_cdev;
use tm1637_gpio_driver::{TM1637Adapter,Brightness};
use tm1637_gpio_driver::mappings::UpCharBits;

use std::sync::mpsc::{Sender,Receiver};

use rust_gpiozero::*;

use crate::util;

pub fn get_tm_1637_thread(dio_pin : u32, clk_pin : u32, brightness : Brightness, rx : Receiver<bool>) -> JoinHandle<Result<(),&'static str>> {
    thread::spawn(move || {
        let mut cpu : u16 = util::get_rasperry_pi_temp()?;
        let mut fahrenheit = false;
        let mut tm1637display = setup_gpio_cdev(
            clk_pin,
            dio_pin,
            Box::from(|| sleep(Duration::from_micros(100))),
            "/dev/gpiochip0"
        );
        tm1637display.set_brightness(brightness);
        loop {
            let received = rx.recv_timeout(Duration::new(1,0));
            if received.is_ok() {
                fahrenheit = !fahrenheit;
            }
            tm1637display.write_segments_raw(&convert_u16_to_tm_array(cpu,fahrenheit), 0);
            cpu = util::get_rasperry_pi_temp()?;
        }
    })
}

fn convert_u16_to_tm_array(num : u16, fahrenheit : bool) -> [u8;4] {
    let mut cpu = num;
    if fahrenheit {
        cpu = (cpu * 9/5 + 32) as u16;
    }
    let mut cpu_array = TM1637Adapter::encode_number(cpu);
    cpu_array[0] = if fahrenheit { UpCharBits::UpF as u8 } else { UpCharBits::UpC as u8 };
    cpu_array
}

pub fn get_button_thread_handler(btn_pin : u8, tx : Sender<bool>) -> JoinHandle<Result<(),&'static str>> {
    thread::spawn(move || {
        let mut button = Button::new(btn_pin);
        loop {
            button.wait_for_press(None);
            tx.send(true).unwrap();
        }
    })
}