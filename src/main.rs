use std::thread::JoinHandle;

use std::sync::mpsc;

use std::error::Error;

mod util;
mod thread_handlers;

/**
 * Arguments application:
 * app dio_pin_tm clk_pin_tm brightness button_pin
*/
fn main() -> Result<(), Box<dyn Error>> {
    let (tx,rx) = mpsc::channel();
    let args : Vec<String> = std::env::args().collect();
    let brightness = util::get_brightness(util::string_to_u16(&args[3])?)?;
    
    let tm_handler : JoinHandle<Result<(),&'static str>> = thread_handlers::get_tm_1637_thread(util::string_to_u32(&args[1])?,util::string_to_u32(&args[2])?,brightness,rx);

    let btn_handler : JoinHandle<Result<(),&'static str>> = thread_handlers::get_button_thread_handler(util::string_to_u8(&args[4])?, tx);
    
    tm_handler.join().unwrap()?;
    btn_handler.join().unwrap()?;

	Ok(())
}