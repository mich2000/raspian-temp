use crate::RaspianError;
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

pub fn get_rasperry_pi_temp(buffer: &mut [u8;4]]) -> Result<u16, RaspianError> {
    let file_pi_temp = std::fs::File::open(CPU_TEMP_PATH).or(Err(RaspianError::CpuTempFileFail))?;
    let mut handle = file_pi_temp.take(4);
    handle.read_exact(buffer).or(Err(RaspianError::CpuTempCannotBeRead))?;
    // Converts '"450' as a example buffer and takes 
    let string_buffer = str::from_utf8(&buffer[1..]).unwrap();
    Ok(string_to_u16(string_buffer)? / 10)
}

#[test]
fn test_out_file() {
    let mut temp_vector = [0;4];
    let file_pi_temp = std::fs::File::open("foo.txt").unwrap();
    let mut handle = file_pi_temp.take(7);
    handle.read_exact(temp_vector).unwrap();
    // Converts '"450' as a example buffer and takes 
    let string_buffer = str::from_utf8(temp_vector[1..]).unwrap();
    let temp = string_to_u16(temp_vector).unwrap() / 10;
    assert!(45, temp);
}
