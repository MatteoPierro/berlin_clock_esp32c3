use berlin_clock::{berlin_clock, LightState, Time};
use chrono::{Local, Timelike};
use esp_idf_svc::hal::delay::FreeRtos;
use esp_idf_svc::hal::gpio::{Gpio0, Gpio1, Gpio2, Gpio3, Gpio4, InputOutput, Pin, PinDriver};
use esp_idf_svc::hal::peripherals::Peripherals;

struct CircuitClock<'a> {
    seconds: PinDriver<'a, Gpio0, InputOutput>,
    minutes: MinutesPins<'a>,
}

impl CircuitClock<'_> {
    fn display_minutes(&mut self, minutes_row: Vec<LightState>) {
        self.minutes.display(minutes_row)
    }

    fn display_seconds(&mut self, val: LightState) {
        toggle(&mut self.seconds, val)
    }
}

struct MinutesPins<'a> {
    first: PinDriver<'a, Gpio1, InputOutput>,
    second: PinDriver<'a, Gpio2, InputOutput>,
    third: PinDriver<'a, Gpio3, InputOutput>,
    forth: PinDriver<'a, Gpio4, InputOutput>,
}

impl MinutesPins<'_> {
    fn display(&mut self, minutes_row: Vec<LightState>) {
        toggle(&mut self.first, minutes_row[0]);
        toggle(&mut self.second, minutes_row[1]);
        toggle(&mut self.third, minutes_row[2]);
        toggle(&mut self.forth, minutes_row[3]);
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

    let mut minutes = MinutesPins {
        first: one,
        second: two,
        third: three,
        forth: four,
    };

    let mut circuit_clock = CircuitClock {
        seconds: zero,
        minutes,
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
        circuit_clock.display_minutes(minutes_row);

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
