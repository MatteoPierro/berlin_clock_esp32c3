use berlin_clock_hardware::{current_time, fetch_time};
use esp_idf_svc::hal::prelude::Peripherals;
use esp_idf_svc::log::EspLogger;
use log::info;
use std::thread::sleep;
use std::time::Duration;

fn main() -> anyhow::Result<()> {
    EspLogger::initialize_default();
    let peripherals = Peripherals::take()?;
    let _s = fetch_time(peripherals.modem)?;

    loop {
        info!("Current time: {:?}", current_time());
        sleep(Duration::from_secs(1));
    }
}
