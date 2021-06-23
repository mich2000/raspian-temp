use std::error::Error;
use std::fmt::{self, Formatter};

pub type IoError = std::io::Error;

#[derive(Debug)]
pub enum RaspianError {
    ParsingNumFailed,
    CpuTempFileFail,
    CpuTempCannotBeRead,
    OutOfBrightnessRange,
    ArgumentIsEmpty,
    Io(IoError),
}

impl From<IoError> for RaspianError {
    fn from(io_error : IoError) -> Self {
        RaspianError::Io(io_error)
    }
}

impl Error for RaspianError {}

impl fmt::Display for RaspianError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match &self {
            RaspianError::ParsingNumFailed => write!(f, "Could not parse the string to u16."),
            RaspianError::CpuTempCannotBeRead => {
                write!(f, "Could not read from the raspberry pi file temperature")
            }
            RaspianError::CpuTempFileFail => write!(
                f,
                "Could not read the cpu temperature from the raspberry pi."
            ),
            RaspianError::OutOfBrightnessRange => write!(
                f,
                "Number was not between 0 and 7, these are the brightness levels."
            ),
            RaspianError::ArgumentIsEmpty => write!(f, "Argument is empty"),
            RaspianError::Io(e) => write!(f, "An IO error: {}", e),
        }
    }
}
