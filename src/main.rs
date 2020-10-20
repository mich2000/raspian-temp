use std::thread::sleep;
use std::time::Duration;

use tm1637_gpio_driver::gpio_api::setup_gpio_cdev;
use tm1637_gpio_driver::TM1637Adapter;

use std::convert::TryInto;

mod util;

/**
 * Arguments application:
 * app dio_pin clk_pin brightness
*/
fn main() {
    let mut cpu = util::get_rasperry_pi_temp();
    let args = util::get_args();
    let brightness = util::get_brightness(util::string_to_u32(&args[3]).unwrap()).unwrap();

    let bit_delay_fn = Box::from(|| sleep(Duration::from_micros(100)));
    let mut tm1637display = setup_gpio_cdev( util::string_to_u32(&args[2]).unwrap(), util::string_to_u32(&args[1]).unwrap() , bit_delay_fn, "/dev/gpiochip0");
    tm1637display.set_brightness(brightness);
	loop {
		sleep(Duration::new(1,0));
		tm1637display.write_segments_raw(&TM1637Adapter::encode_number(cpu.try_into().unwrap()), 0);
		cpu = util::get_rasperry_pi_temp();
	}
}
