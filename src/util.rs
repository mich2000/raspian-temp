use tm1637_gpio_driver::Brightness;
use std::env;
use std::fs;

static CPU_TEMP_PATH : &str = "/sys/class/thermal/thermal_zone0/temp";

pub fn get_args() -> Vec<String> {
    env::args().collect()
}

pub fn string_to_u32(string : &str) -> Result<u32,&'static str> {
    match string.parse::<u32>() {
        Ok(num) => Ok(num),
        Err(_) => Err("Could not parse the string to u32.")
    }
}

pub fn get_brightness(num : u32) -> Result<Brightness,&'static str> {
    match num {
        0 => Ok(Brightness::L0),
        1 => Ok(Brightness::L1),
        2 => Ok(Brightness::L2),
        3 => Ok(Brightness::L3),
        4 => Ok(Brightness::L4),
        5 => Ok(Brightness::L5),
        6 => Ok(Brightness::L6),
        7 => Ok(Brightness::L7),
        _ => Err("Number was not between 0 and 7, these are the brightness levels.")
    }
}

pub fn get_rasperry_pi_temp() -> u32 {
    string_to_u32(fs::read_to_string(CPU_TEMP_PATH).expect("Could not read the file where the raspberry pi cpu temperature is.")
    .trim()).expect("could not get u32 of the result.") / 1000
}
