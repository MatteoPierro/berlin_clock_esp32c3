use berlin_clock::{berlin_clock, Time};
use chrono::{Local, Timelike};
use esp_idf_svc::hal::delay::FreeRtos;

use berlin_clock_hardware::{CircuitPins};

fn main() {
    esp_idf_svc::log::EspLogger::initialize_default();
    let mut circuit_pins = CircuitPins::new();

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

        circuit_pins.display_seconds(clock.seconds);
        FreeRtos::delay_ms(1000);
    }
}
