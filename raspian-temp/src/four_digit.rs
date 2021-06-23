use std::thread::{self, sleep, JoinHandle};
use std::time::Duration;

use tm1637_gpio_driver::gpio_api::setup_gpio_cdev;
use tm1637_gpio_driver::{Brightness, TM1637Adapter};

use std::sync::mpsc::Receiver;

use crate::util;
use crate::RaspianError;

pub fn get_tm_1637_thread(
    dio_pin: u32,
    clk_pin: u32,
    brightness: Brightness,
    rx: Receiver<()>,
) -> JoinHandle<Result<(), RaspianError>> {
    thread::spawn(move || {
        let mut tm1637display = setup_gpio_cdev(
            clk_pin,
            dio_pin,
            Box::from(|| sleep(Duration::from_micros(100))),
            "/dev/gpiochip0",
        );
        let wait_duration = Duration::from_millis(500);
        tm1637display.set_brightness(brightness);
        let mut fahrenheit = false;
        let temp_file = util::get_raspberry_pi_temp_file()?;
        // We are mostly reading 2 bytes to be sure we got the temperature.
        let mut temp_vector = [0; 2];
        loop {
            //if received.is_ok() { fahrenheit = !fahrenheit; }
            fahrenheit ^= rx.recv_timeout(wait_duration).is_ok();
            tm1637display.write_segments_raw(
                &convert_u16_to_tm_array(
                    util::get_raspberry_pi_temp(&mut temp_vector, &temp_file)?,
                    fahrenheit,
                ),
                0,
            );
            temp_file.sync_all()?;
        }
    })
}

fn convert_u16_to_tm_array(num: u16, fahrenheit: bool) -> [u8; 4] {
    let mut cpu_array = TM1637Adapter::encode_number(num + (num * 4 / 5 + 32) * fahrenheit as u16);
    //cpu_array[0] = (((UpCharBits::UpF - UpCharBits::UpC) * fahrenheit as u8) - UpCharBits::UpC) as u8;
    cpu_array[0] = (56 * fahrenheit as u8) + 57;
    cpu_array
}
