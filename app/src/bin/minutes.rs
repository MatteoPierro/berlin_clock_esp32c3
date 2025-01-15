use berlin_clock::{berlin_clock, time};
use berlin_clock_hardware::{ClockPins, FiveMinutesPins};
use esp_idf_svc::hal::delay::FreeRtos;
use esp_idf_svc::hal::gpio::PinDriver;
use esp_idf_svc::hal::peripherals::Peripherals;

fn main() {
    esp_idf_svc::log::EspLogger::initialize_default();
    let peripherals = Peripherals::take().unwrap();

    let mut five_minutes = FiveMinutesPins {
        first: PinDriver::input_output(peripherals.pins.gpio10).unwrap(),
        second: PinDriver::input_output(peripherals.pins.gpio8).unwrap(),
        third: PinDriver::input_output(peripherals.pins.gpio3).unwrap(),
        forth: PinDriver::input_output(peripherals.pins.gpio4).unwrap(),
        fifth: PinDriver::input_output(peripherals.pins.gpio9).unwrap(),
        sixth: PinDriver::input_output(peripherals.pins.gpio1).unwrap(),
        seventh: PinDriver::input_output(peripherals.pins.gpio2).unwrap(),
        eighth: PinDriver::input_output(peripherals.pins.gpio0).unwrap(),
        ninth: PinDriver::input_output(peripherals.pins.gpio7).unwrap(),
        tenth: PinDriver::input_output(peripherals.pins.gpio6).unwrap(),
        eleventh: PinDriver::input_output(peripherals.pins.gpio5).unwrap()
    };

    let clock = berlin_clock(time("23:59:59"));
    five_minutes.display(clock.five_minutes);

    loop {
        FreeRtos::delay_ms(1000);
    }
}
