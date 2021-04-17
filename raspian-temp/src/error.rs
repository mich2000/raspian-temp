use std::error::Error;
use std::fmt::{self, Display, Formatter};

#[derive(Debug, Error)]
pub enum RaspianError {
    JsonParsingFailed,
    JsonConfigFaulthy,
    ParsingNumFailed,
    CpuTempFileFail,
    OutOfBrightnessRange,
}

impl fmt::Display for RaspianError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match &self {
            RaspianError::JsonParsingFailed => write!(f, "Could not parse the given json"),
            RaspianError::JsonConfigFaulthy => write!(f, "Give in a json configuration."),
            RaspianError::ParsingNumFailed => write!(f, "Could not parse the string to u16."),
            RaspianError::CpuTempFileFail => write!(
                f,
                "Could not read the cpu temperature from the raspberry pi."
            ),
            RaspianError::OutOfBrightnessRange => write!(
                f,
                "Number was not between 0 and 7, these are the brightness levels."
            ),
        }
    }
}
