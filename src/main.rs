use std::time::Duration;
use std::thread::{ self, JoinHandle, sleep};

use tm1637_gpio_driver::gpio_api::setup_gpio_cdev;
use tm1637_gpio_driver::TM1637Adapter;

use std::sync::mpsc;

use std::error::Error;

use rust_gpiozero::*;

mod util;

/**
 * Arguments application:
 * app dio_pin_tm clk_pin_tm brightness button_pin
*/
fn main() -> Result<(), Box<dyn Error>> {
    let (tx, rx) = mpsc::channel();
    let args = util::get_args();
    let brightness = util::get_brightness(util::string_to_u16(&args[3])?)?;
    
    let tm_handler : JoinHandle<Result<(),&'static str>> = thread::spawn(move || {
        let sys_args = util::get_args();
        let mut cpu : u16 = util::get_rasperry_pi_temp()?;
        let mut fahrenheit = false;
        let mut tm1637display = setup_gpio_cdev(
            util::string_to_u32(&sys_args[2])?,
            util::string_to_u32(&sys_args[1])?,
            Box::from(|| sleep(Duration::from_micros(100))),
            "/dev/gpiochip0"
        );
        tm1637display.set_brightness(brightness);
        loop {
            let received = rx.recv_timeout(Duration::new(1,0));
            if received.is_ok() {
                fahrenheit = !fahrenheit;
            }
            if fahrenheit {
                cpu = (cpu * 9/5 + 32) as u16;
            }
            tm1637display.write_segments_raw(&TM1637Adapter::encode_number(cpu), 0);
            cpu = util::get_rasperry_pi_temp()?;
        }
    });

    let btn_handler : JoinHandle<Result<(),&'static str>> = thread::spawn(move || {
        let sys_args = util::get_args();
        let mut button = Button::new(util::string_to_u8(&sys_args[4])?);
        loop {
            button.wait_for_press(None);
            tx.send(1).unwrap();
        }
    });
    
    tm_handler.join().unwrap()?;
    btn_handler.join().unwrap()?;

	Ok(())
}