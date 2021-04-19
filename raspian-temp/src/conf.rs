use crate::util::get_brightness;
use crate::RaspianError;
use serde::Deserialize;
use tm1637_gpio_driver::Brightness;

#[derive(Deserialize)]
pub struct RaspianConfig {
    dio_pin: u32,
    clk_pin: u32,
    brightness: u16,
    btn_pin: u8,
}

impl RaspianConfig {
    pub fn get_dio_pin(&self) -> u32 {
        self.dio_pin
    }

    pub fn get_clk_pin(&self) -> u32 {
        self.clk_pin
    }

    pub fn get_brightness(&self) -> Result<Brightness, RaspianError> {
        get_brightness(self.brightness)
    }

    pub fn get_btn_pin(&self) -> u8 {
        self.btn_pin
    }
}
