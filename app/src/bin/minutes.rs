use esp_idf_svc::hal::delay::FreeRtos;
use esp_idf_svc::hal::gpio::PinDriver;
use esp_idf_svc::hal::peripherals::Peripherals;

fn main() {
    esp_idf_svc::log::EspLogger::initialize_default();

    let peripherals = Peripherals::take().unwrap();
    let mut one = PinDriver::input_output(peripherals.pins.gpio1).unwrap();

    loop {
        one.toggle().unwrap();
        FreeRtos::delay_ms(1000);
    }
}
