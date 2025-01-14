use chrono::Local;
use esp_idf_svc::hal::delay::FreeRtos;
use esp_idf_svc::hal::gpio::PinDriver;
use esp_idf_svc::hal::peripherals::Peripherals;
use berlin_clock::add;

fn main() {
    esp_idf_svc::log::EspLogger::initialize_default();

    let now = Local::now();
    log::info!("Hello, {}", now);
    log::info!("Adding {}", add(2, 3));

    let peripherals = Peripherals::take().unwrap();
    let mut led = PinDriver::input_output(peripherals.pins.gpio7).unwrap();

    loop {
        led.set_high().unwrap();
        log::info!("level {:?}", &led.get_level());
        FreeRtos::delay_ms(1000);
        led.set_low().unwrap();
        log::info!("level {:?}", &led.get_level());
        FreeRtos::delay_ms(3000);
    }
}
