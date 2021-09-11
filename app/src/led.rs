use std::thread;
use std::time::Duration;

use rppal::gpio::OutputPin;

const BLINK_DURATION: Duration = Duration::from_millis(100);

pub fn flash(led: &mut OutputPin, times: u8) {
    (0..times).for_each(|_| {
        led.set_high();
        thread::sleep(BLINK_DURATION);
        led.set_low();
        thread::sleep(BLINK_DURATION);
    })
}
