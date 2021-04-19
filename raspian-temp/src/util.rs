use crate::RaspianError;
use std::fs;
use std::io::Read;
use tm1637_gpio_driver::Brightness;

static CPU_TEMP_PATH: &str = "/sys/class/thermal/thermal_zone0/temp";

pub fn string_to_u16(string: &str) -> Result<u16, RaspianError> {
    match string.parse::<u16>() {
        Ok(num) => Ok(num),
        Err(_) => Err(RaspianError::ParsingNumFailed),
    }
}

pub fn get_brightness(num: u16) -> Result<Brightness, RaspianError> {
    match num {
        0 => Ok(Brightness::L0),
        1 => Ok(Brightness::L1),
        2 => Ok(Brightness::L2),
        3 => Ok(Brightness::L3),
        4 => Ok(Brightness::L4),
        5 => Ok(Brightness::L5),
        6 => Ok(Brightness::L6),
        7 => Ok(Brightness::L7),
        _ => Err(RaspianError::OutOfBrightnessRange),
    }
}

pub fn get_rasperry_pi_temp(buffer: &mut String) -> Result<u16, RaspianError> {
    let mut file_pi_temp = std::fs::File::open(CPU_TEMP_PATH).unwrap();
    let mut handle = file_pi_temp.take(7);
    handle.read(&mut buffer);
    drop(file);
    Ok(string_to_u16(cpu_string.trim())? / 1000)
}
