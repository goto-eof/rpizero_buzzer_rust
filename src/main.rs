extern crate rust_gpiozero;
use rust_gpiozero::*;
use std::thread::sleep;
use std::time::Duration;

fn main() {
    // Tell the Pi which GPIO pin you are using
    let mut buzzer = Buzzer::new(17);
}
