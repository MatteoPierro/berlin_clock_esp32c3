use berlin_clock::{add, berlin_clock, time, LightState};
use chrono::Local;
use esp_idf_svc::hal::delay::FreeRtos;
use esp_idf_svc::hal::gpio::PinDriver;
use esp_idf_svc::hal::peripherals::Peripherals;

fn main() {
    esp_idf_svc::log::EspLogger::initialize_default();

    let now = Local::now();
    log::info!("Hello, {}", now);

    let peripherals = Peripherals::take().unwrap();
    let mut _zero = PinDriver::input_output(peripherals.pins.gpio0).unwrap();
    let mut one = PinDriver::input_output(peripherals.pins.gpio1).unwrap();
    let mut two = PinDriver::input_output(peripherals.pins.gpio2).unwrap();
    let mut three = PinDriver::input_output(peripherals.pins.gpio3).unwrap();
    let mut four = PinDriver::input_output(peripherals.pins.gpio4).unwrap();

    let clock = berlin_clock(time("00:07:00"));
    let val = clock.minutes.clone();
    if val[0] == LightState::On {
        one.set_high().unwrap();
    }

    if val[1] == LightState::On {
        two.set_high().unwrap();
    }

    if val[2] == LightState::On {
        three.set_high().unwrap();
    }

    if val[3] == LightState::On {
        four.set_high().unwrap();
    }

    loop {
        FreeRtos::delay_ms(1000);
    }
}
