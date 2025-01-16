use std::thread::sleep;
use std::time::Duration;
use berlin_clock::{berlin_clock, Time};
use chrono::{Local, Timelike};
use esp_idf_svc::hal::delay::FreeRtos;
use esp_idf_svc::hal::gpio::{Gpio10, Gpio11, Gpio12, Gpio5, Gpio6, Gpio7, Gpio8, Gpio9, InputOutput, PinDriver};
use esp_idf_svc::hal::peripherals::Peripherals;
use log::info;
use berlin_clock_hardware::{current_time, fetch_time, MinutesPins, Seconds};

struct HoursPins<'d> {
    first: PinDriver<'d, Gpio5, InputOutput>,
    second: PinDriver<'d, Gpio6, InputOutput>,
    third: PinDriver<'d, Gpio7, InputOutput>,
    forth: PinDriver<'d, Gpio8, InputOutput>
}

struct FiveHoursPins<'d> {
    first: PinDriver<'d, Gpio9, InputOutput>,
    second: PinDriver<'d, Gpio10, InputOutput>,
    third: PinDriver<'d, Gpio11, InputOutput>,
    forth: PinDriver<'d, Gpio12, InputOutput>
}

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

    let mut hours = HoursPins {
        first: PinDriver::input_output(peripherals.pins.gpio5)?,
        second: PinDriver::input_output(peripherals.pins.gpio6)?,
        third: PinDriver::input_output(peripherals.pins.gpio7)?,
        forth: PinDriver::input_output(peripherals.pins.gpio8)?,
    };

    let mut five_hours = FiveHoursPins {
        first: PinDriver::input_output(peripherals.pins.gpio9)?,
        second: PinDriver::input_output(peripherals.pins.gpio10)?,
        third: PinDriver::input_output(peripherals.pins.gpio11)?,
        forth: PinDriver::input_output(peripherals.pins.gpio12)?,
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
