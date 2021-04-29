#![no_std]
#![no_main]
#![allow(dead_code)]

use panic_halt as _;
use wio_terminal as wio;

use wio::entry;
use wio::hal::gpio::{Floating, Input, Output, Pc26, Port, PushPull};
use wio::pac::Peripherals;
use wio::prelude::*;

#[entry]
fn main() -> ! {
    let peripherals = Peripherals::take().unwrap();
    let mut pins = wio::Pins::new(peripherals.PORT);
    let mut led = pins.user_led.into_push_pull_output(&mut pins.port);
    let button1 = Button1::new(pins.button1, &mut pins.port);

    loop {
        if button1.is_pressed() {
            led.set_high().unwrap();
        } else {
            led.set_low().unwrap();
        }
    }
}

struct Button1 {
    pin: Pc26<Input<Floating>>,
}
impl Button1 {
    fn new(pin: Pc26<Input<Floating>>, port: &mut Port) -> Button1 {
        Button1 {
            pin: pin.into_floating_input(port),
        }
    }

    fn is_pressed(&self) -> bool {
        self.pin.is_low().unwrap()
    }
    fn is_released(&self) -> bool {
        self.pin.is_high().unwrap()
    }
}
