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
    let mut zero = PinDriver::input_output(peripherals.pins.gpio0).unwrap();
    let mut one = PinDriver::input_output(peripherals.pins.gpio1).unwrap();
    let mut two = PinDriver::input_output(peripherals.pins.gpio2).unwrap();
    let mut three = PinDriver::input_output(peripherals.pins.gpio3).unwrap();
    let mut four = PinDriver::input_output(peripherals.pins.gpio4).unwrap();

    loop {
        zero.toggle().unwrap();
        one.toggle().unwrap();
        two.toggle().unwrap();
        three.toggle().unwrap();
        four.toggle().unwrap();
        log::info!("level {:?}", &zero.get_level());
        FreeRtos::delay_ms(1000);
    }
}
