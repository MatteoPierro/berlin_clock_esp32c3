use berlin_clock::{berlin_clock, LightState, Time};
use chrono::{Local, Timelike};
use esp_idf_svc::hal::delay::FreeRtos;
use esp_idf_svc::hal::gpio::{Gpio0, InputOutput, Pin, PinDriver};
use esp_idf_svc::hal::peripherals::Peripherals;

struct CircuitClock<'a> {
    seconds: PinDriver<'a, Gpio0, InputOutput>
}

impl CircuitClock<'_> {
    fn display_seconds(&mut self, val :LightState) {
        toggle(&mut self.seconds, val)
    }
}

fn main() {
    esp_idf_svc::log::EspLogger::initialize_default();

    let mut peripherals = Peripherals::take().unwrap();
    let mut zero = PinDriver::input_output(&mut peripherals.pins.gpio0).unwrap();
    let mut one = PinDriver::input_output(&mut peripherals.pins.gpio1).unwrap();
    let mut two = PinDriver::input_output(&mut peripherals.pins.gpio2).unwrap();
    let mut three = PinDriver::input_output(&mut peripherals.pins.gpio3).unwrap();
    let mut four = PinDriver::input_output(&mut peripherals.pins.gpio4).unwrap();

    let mut circuit_clock = CircuitClock {
        seconds: zero
    };

    loop {
        let now = Local::now();
        let time = Time {
            hours: now.hour() as usize,
            minutes: now.minute() as usize,
            seconds: now.second() as usize,
        };

        log::info!("time, {}", now);

        let clock = berlin_clock(time);

        let minutes_row = clock.minutes.clone();
        toggle(&mut one, minutes_row[0]);
        toggle(&mut two, minutes_row[1]);
        toggle(&mut three, minutes_row[2]);
        toggle(&mut four, minutes_row[3]);

        circuit_clock.display_seconds(clock.seconds);
        FreeRtos::delay_ms(1000);
    }
}

fn toggle<T: Pin>(one: &mut PinDriver<T, InputOutput>, value: LightState) {
    if value == LightState::On {
        one.set_high().unwrap();
    } else {
        one.set_low().unwrap();
    }
}
