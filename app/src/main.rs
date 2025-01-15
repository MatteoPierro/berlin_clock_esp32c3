use std::thread::sleep;
use std::time::Duration;
use berlin_clock::{berlin_clock, Time};
use chrono::{Local, Timelike};
use esp_idf_svc::hal::delay::FreeRtos;
use esp_idf_svc::hal::gpio::PinDriver;
use esp_idf_svc::hal::peripherals::Peripherals;
use berlin_clock_hardware::{fetch_time, ClockPins, MinutesPins};

fn main() -> anyhow::Result<()> {
    esp_idf_svc::log::EspLogger::initialize_default();
    let peripherals = Peripherals::take()?;
    fetch_time(peripherals.modem)?;

    let mut minutes = MinutesPins {
        first: PinDriver::input_output(peripherals.pins.gpio1)?,
        second: PinDriver::input_output(peripherals.pins.gpio2)?,
        third: PinDriver::input_output(peripherals.pins.gpio3)?,
        forth: PinDriver::input_output(peripherals.pins.gpio4)?,
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
        minutes.display(minutes_row);

        sleep(Duration::from_secs(1));
    }
}
