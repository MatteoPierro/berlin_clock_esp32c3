use berlin_clock::{berlin_clock, time};
use berlin_clock_hardware::CircuitPins;
use esp_idf_svc::hal::delay::FreeRtos;

fn main() {
    esp_idf_svc::log::EspLogger::initialize_default();
    let mut circuit_pins = CircuitPins::new();

    let clock = berlin_clock(time("23:59:59"));
    circuit_pins.display_minutes(clock.minutes.clone());

    loop {
        FreeRtos::delay_ms(1000);
    }
}
