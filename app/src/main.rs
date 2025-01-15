use std::thread::sleep;
use std::time::Duration;
use berlin_clock::{berlin_clock, Time};
use chrono::{Local, Timelike};
use esp_idf_svc::hal::delay::FreeRtos;
use esp_idf_svc::hal::gpio::PinDriver;
use esp_idf_svc::hal::peripherals::Peripherals;
use log::info;
use berlin_clock_hardware::{current_time, fetch_time, MinutesPins, Seconds};

fn main() -> anyhow::Result<()> {
    esp_idf_svc::log::EspLogger::initialize_default();
    let peripherals = Peripherals::take()?;
    fetch_time(peripherals.modem)?;

    let mut seconds = Seconds {
        first: PinDriver::input_output(peripherals.pins.gpio0)?
    };

    let mut minutes = MinutesPins {
        first: PinDriver::input_output(peripherals.pins.gpio1)?,
        second: PinDriver::input_output(peripherals.pins.gpio2)?,
        third: PinDriver::input_output(peripherals.pins.gpio3)?,
        forth: PinDriver::input_output(peripherals.pins.gpio4)?,
    };

    loop {
        let now = current_time();

        info!("time, {:?}", now);

        let clock = berlin_clock(now);
        seconds.display(clock.seconds);
        minutes.display(clock.minutes.clone());

        sleep(Duration::from_secs(1));
    }
}
