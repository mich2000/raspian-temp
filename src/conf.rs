use crate::util::get_brightness;
use crate::util::{string_to_u16, string_to_u32, string_to_u8};
use crate::RaspianError;
use std::env::args;
use tm1637_gpio_driver::Brightness;

pub struct RaspianConfig {
    dio_pin: u32,
    clk_pin: u32,
    brightness: Brightness,
    btn_pin: u8,
}

impl RaspianConfig {
    pub fn get_from_env() -> Result<Self, RaspianError> {
        let mut env_args = args();
        env_args.next();
        Ok(Self {
            dio_pin: string_to_u32(&env_args.next().ok_or(RaspianError::ArgumentIsEmpty)?)?,
            clk_pin: string_to_u32(&env_args.next().ok_or(RaspianError::ArgumentIsEmpty)?)?,
            brightness: get_brightness(string_to_u16(
                &env_args.next().ok_or(RaspianError::ArgumentIsEmpty)?,
            )?)?,
            btn_pin: string_to_u8(&env_args.next().ok_or(RaspianError::ArgumentIsEmpty)?)?,
        })
    }

    pub fn get_dio_pin(&self) -> u32 {
        self.dio_pin
    }

    pub fn get_clk_pin(&self) -> u32 {
        self.clk_pin
    }

    pub fn get_brightness(&self) -> Brightness {
        match self.brightness {
            Brightness::L0 => Brightness::L0,
            Brightness::L1 => Brightness::L1,
            Brightness::L2 => Brightness::L2,
            Brightness::L3 => Brightness::L3,
            Brightness::L4 => Brightness::L4,
            Brightness::L5 => Brightness::L5,
            Brightness::L6 => Brightness::L6,
            Brightness::L7 => Brightness::L7,
        }
    }

    pub fn get_btn_pin(&self) -> u8 {
        self.btn_pin
    }
}
