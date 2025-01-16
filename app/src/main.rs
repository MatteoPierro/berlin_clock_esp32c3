use std::thread::sleep;
use std::time::Duration;
use berlin_clock::{berlin_clock, Time};
use chrono::{Local, Timelike};
use esp_idf_svc::hal::delay::FreeRtos;
use esp_idf_svc::hal::gpio::{Gpio10, Gpio11, Gpio12, Gpio5, Gpio6, Gpio7, Gpio8, Gpio9, InputOutput, PinDriver};
use esp_idf_svc::hal::peripherals::Peripherals;
use log::info;
use berlin_clock_hardware::{current_time, fetch_time, FiveHoursPins, HoursPins, MinutesPins, Seconds};

fn main() -> anyhow::Result<()> {
    esp_idf_svc::log::EspLogger::initialize_default();
    let peripherals = Peripherals::take()?;
    let _s = fetch_time(peripherals.modem)?;

    let mut seconds = Seconds {
        first: PinDriver::input_output(peripherals.pins.gpio0)?
    };

    let mut minutes = MinutesPins {
        first: PinDriver::input_output(peripherals.pins.gpio20)?,
        second: PinDriver::input_output(peripherals.pins.gpio21)?,
        third: PinDriver::input_output(peripherals.pins.gpio9)?,
        forth: PinDriver::input_output(peripherals.pins.gpio10)?,
    };

    let mut hours = HoursPins {
        first: PinDriver::input_output(peripherals.pins.gpio7)?,
        second: PinDriver::input_output(peripherals.pins.gpio8)?,
        third: PinDriver::input_output(peripherals.pins.gpio5)?,
        forth: PinDriver::input_output(peripherals.pins.gpio6)?,
    };

    let mut five_hours = FiveHoursPins {
        first: PinDriver::input_output(peripherals.pins.gpio4)?,
        second: PinDriver::input_output(peripherals.pins.gpio3)?,
        third: PinDriver::input_output(peripherals.pins.gpio2)?,
        forth: PinDriver::input_output(peripherals.pins.gpio1)?,
    };

    loop {
        let now = current_time();

        info!("time, {:?}", now);

        let clock = berlin_clock(now);
        seconds.display(clock.seconds);
        minutes.display(clock.minutes.clone());
        hours.display(clock.hours.clone());
        five_hours.display(clock.five_hours.clone());

        sleep(Duration::from_secs(1));
    }
}
