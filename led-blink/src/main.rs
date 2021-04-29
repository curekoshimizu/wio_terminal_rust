#![no_std]
#![no_main]
#![allow(dead_code)]

use panic_halt as _;
use wio_terminal as wio;

use wio::entry;
use wio::hal::gpio::{Floating, Input, Output, Pa15, Pc26, Port, PushPull};
use wio::pac::Peripherals;
use wio::prelude::*;

#[entry]
fn main() -> ! {
    let peripherals = Peripherals::take().unwrap();
    let mut pins = wio::Pins::new(peripherals.PORT);
    let mut led = BlueLed::new(pins.user_led, &mut pins.port);
    let button1 = RightButton::new(pins.button1, &mut pins.port);

    loop {
        if button1.is_pressed() {
            led.turn_on();
        } else {
            led.turn_off();
        }
    }
}

struct RightButton {
    pin: Pc26<Input<Floating>>,
}
impl RightButton {
    fn new(pin: Pc26<Input<Floating>>, port: &mut Port) -> RightButton {
        RightButton {
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

struct BlueLed {
    pin: Pa15<Output<PushPull>>,
}
impl BlueLed {
    fn new(pin: Pa15<Input<Floating>>, port: &mut Port) -> BlueLed {
        BlueLed {
            pin: pin.into_push_pull_output(port),
        }
    }

    fn turn_on(&mut self) {
        self.pin.set_high().unwrap();
    }
    fn turn_off(&mut self) {
        self.pin.set_low().unwrap();
    }
}
