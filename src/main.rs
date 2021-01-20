use std::thread::JoinHandle;

use std::sync::mpsc;

use std::error::Error;

mod handlers;
mod util;

/**
 * Arguments application:
 * app dio_pin_tm clk_pin_tm brightness button_pin
*/
fn main() -> Result<(), Box<dyn Error>> {
    let (button_sender, tm1637_receiver) = mpsc::channel();
    let mut args = std::env::args().skip(1);
    let tm_handler: JoinHandle<Result<(), &'static str>> = handlers::four_digit::get_tm_1637_thread(
        util::string_to_u32(
            &args
                .next()
                .expect("1st argument for the dio_pin is not there")
                .as_ref(),
        )?,
        util::string_to_u32(
            &args
                .next()
                .expect("2th argument clk_pin is not there")
                .as_ref(),
        )?,
        util::get_brightness(util::string_to_u16(
            &args
                .next()
                .expect("3th argument for the brightness is not there")
                .as_ref(),
        )?)?,
        tm1637_receiver,
    );
    let btn_handler: JoinHandle<Result<(), &'static str>> =
        handlers::button::get_button_thread_handler(
            util::string_to_u8(
                &args
                    .next()
                    .expect("4th agument of the button pin is not there")
                    .as_ref(),
            )?,
            button_sender,
        );
    tm_handler.join().unwrap()?;
    btn_handler.join().unwrap()?;

    Ok(())
}
