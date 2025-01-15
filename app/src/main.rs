use berlin_clock::{berlin_clock, time, LightState};
use chrono::Local;
use esp_idf_svc::hal::delay::FreeRtos;
use esp_idf_svc::hal::gpio::{InputOutput, Pin, PinDriver};
use esp_idf_svc::hal::peripherals::Peripherals;

fn main() {
    esp_idf_svc::log::EspLogger::initialize_default();

    let now = Local::now();
    log::info!("Hello, {}", now);

    let peripherals = Peripherals::take().unwrap();
    let mut zero = PinDriver::input_output(peripherals.pins.gpio0).unwrap();
    let mut one = PinDriver::input_output(peripherals.pins.gpio1).unwrap();
    let mut two = PinDriver::input_output(peripherals.pins.gpio2).unwrap();
    let mut three = PinDriver::input_output(peripherals.pins.gpio3).unwrap();
    let mut four = PinDriver::input_output(peripherals.pins.gpio4).unwrap();

    let clock = berlin_clock(time("00:08:00"));
    let minutes_row = clock.minutes.clone();
    toggle(&mut one, minutes_row[0]);
    toggle(&mut two, minutes_row[1]);
    toggle(&mut three, minutes_row[2]);
    toggle(&mut four, minutes_row[3]);

    let seconds = clock.seconds;
    toggle(&mut zero, seconds);

    loop {
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
