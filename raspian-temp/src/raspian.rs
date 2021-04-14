use rust_gpiozero::Button;
use tm1637_gpio_driver::gpio_api::setup_gpio_cdev;
use tm1637_gpio_driver::{Brightness, TM1637Adapter};
use raspian_lib::ArduinoLooping;

pub struct RaspianTemp {
    tm1637 : TM1637Adapter,
    button : Button
}