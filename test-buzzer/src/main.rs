use rust_gpiozero::Buzzer;

fn main() {
    let mut buzzer = Buzzer::new(2);
    buzzer.set_beep_count(5);
    buzzer.beep(0.5, 1);
}
