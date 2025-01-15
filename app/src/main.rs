use berlin_clock::{berlin_clock, Time};
use chrono::{Local, Timelike};
use esp_idf_svc::hal::delay::FreeRtos;
use esp_idf_svc::hal::gpio::PinDriver;
use esp_idf_svc::hal::peripherals::Peripherals;
use berlin_clock_hardware::{ClockPins, MinutesPins};

fn main() {
    esp_idf_svc::log::EspLogger::initialize_default();
    let peripherals = Peripherals::take().unwrap();

    let minutes = MinutesPins {
        first: PinDriver::input_output(peripherals.pins.gpio1).unwrap(),
        second: PinDriver::input_output(peripherals.pins.gpio2).unwrap(),
        third: PinDriver::input_output(peripherals.pins.gpio3).unwrap(),
        forth: PinDriver::input_output(peripherals.pins.gpio4).unwrap(),
    };

    let mut circuit_pins = ClockPins {
        // seconds: zero,
        five_minutes,
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
        circuit_pins.display_minutes(minutes_row);

        // circuit_pins.display_seconds(clock.seconds);
        FreeRtos::delay_ms(1000);
    }
}
