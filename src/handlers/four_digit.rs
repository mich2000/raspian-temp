use std::thread::{self, sleep, JoinHandle};
use std::time::Duration;

use tm1637_gpio_driver::gpio_api::setup_gpio_cdev;
use tm1637_gpio_driver::{Brightness, TM1637Adapter};

use std::sync::mpsc::Receiver;

use crate::util;

pub fn get_tm_1637_thread(
    dio_pin: u32,
    clk_pin: u32,
    brightness: Brightness,
    rx: Receiver<()>,
) -> JoinHandle<Result<(), &'static str>> {
    thread::spawn(move || {
        let mut tm1637display = setup_gpio_cdev(
            clk_pin,
            dio_pin,
            Box::from(|| sleep(Duration::from_micros(100))),
            "/dev/gpiochip0",
        );
        tm1637display.set_brightness(brightness);
        let mut fahrenheit = false;
        loop {
            //if received.is_ok() { fahrenheit = !fahrenheit; }
            //cpu_array[0] = (((UpCharBits::UpF - UpCharBits::UpC) * fahrenheit as u8) - UpCharBits::UpC) as u8;
            fahrenheit ^= rx.recv_timeout(Duration::from_millis(500)).is_ok();
            tm1637display.write_segments_raw(
                &convert_u16_to_tm_array(util::get_rasperry_pi_temp()?, fahrenheit),
                0,
            );
        }
    })
}

fn convert_u16_to_tm_array(num: u16, fahrenheit: bool) -> [u8; 4] {
    //if fahrenheit { cpu = (cpu * 9 / 5 + 32) as u16; }
    let mut cpu_array =
        TM1637Adapter::encode_number((num + ((num * 0.8) + 32) * fahrenheit as u16) as u16);
    cpu_array[0] = ((56 * fahrenheit as u8) + 57) as u8;
    cpu_array
}
