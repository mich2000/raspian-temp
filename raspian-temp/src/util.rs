use crate::RaspianError;
use std::io::Read;
use tm1637_gpio_driver::Brightness;
use std::fs::File;

static CPU_TEMP_PATH: &str = "/sys/class/thermal/thermal_zone0/temp";

pub fn string_to_u8(string: &str) -> Result<u8, RaspianError> {
    match string.parse::<u8>() {
        Ok(num) => Ok(num),
        Err(_) => Err(RaspianError::ParsingNumFailed),
    }
}

pub fn string_to_u16(string: &str) -> Result<u16, RaspianError> {
    match string.parse::<u16>() {
        Ok(num) => Ok(num),
        Err(_) => Err(RaspianError::ParsingNumFailed),
    }
}

pub fn string_to_u32(string: &str) -> Result<u32, RaspianError> {
    match string.parse::<u32>() {
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

pub fn get_raspberry_pi_temp_file() -> Result<File, RaspianError> {
    File::open(CPU_TEMP_PATH).or(Err(RaspianError::CpuTempFileFail))
}

pub fn get_raspberry_pi_temp(buffer: &mut [u8; 2], temp_file : &File) -> Result<u16, RaspianError> {
    let mut handle = temp_file.take(2);
    handle
        .read_exact(buffer)
        .or(Err(RaspianError::CpuTempCannotBeRead))?;
    // Converts '"45' as a example buffer and takes
    let string_buffer = std::str::from_utf8(&buffer[..]).unwrap().trim();
    string_to_u16(string_buffer)
}
