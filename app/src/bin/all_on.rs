use berlin_clock::{berlin_clock, time};
use berlin_clock_hardware::{fetch_time, FiveMinutesPins};
use esp_idf_svc::hal::gpio::PinDriver;
use esp_idf_svc::hal::peripherals::Peripherals;
use std::thread::sleep;
use std::time::Duration;

fn main() -> anyhow::Result<()> {
    esp_idf_svc::log::EspLogger::initialize_default();
    let peripherals = Peripherals::take()?;

    let mut five_minutes = FiveMinutesPins {
        first: PinDriver::input_output(peripherals.pins.gpio10)?,
        second: PinDriver::input_output(peripherals.pins.gpio8)?,
        third: PinDriver::input_output(peripherals.pins.gpio3)?,
        forth: PinDriver::input_output(peripherals.pins.gpio4)?,
        fifth: PinDriver::input_output(peripherals.pins.gpio9)?,
        sixth: PinDriver::input_output(peripherals.pins.gpio1)?,
        seventh: PinDriver::input_output(peripherals.pins.gpio2)?,
        eighth: PinDriver::input_output(peripherals.pins.gpio0)?,
        ninth: PinDriver::input_output(peripherals.pins.gpio7)?,
        tenth: PinDriver::input_output(peripherals.pins.gpio6)?,
        eleventh: PinDriver::input_output(peripherals.pins.gpio5)?
    };

    let mut tx =  PinDriver::input_output(peripherals.pins.gpio20)?;
    let mut rx = PinDriver::input_output(peripherals.pins.gpio21)?;

    tx.set_high()?;
    rx.set_high()?;

    let clock = berlin_clock(time("23:59:59"));
    five_minutes.display(clock.five_minutes);

    loop {
        sleep(Duration::from_secs(1));
    }
}
