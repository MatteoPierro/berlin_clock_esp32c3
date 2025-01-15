use berlin_clock_hardware::fetch_time;
use chrono::Local;
use esp_idf_svc::hal::prelude::Peripherals;
use esp_idf_svc::log::EspLogger;
use log::info;
use std::thread::sleep;
use std::time::Duration;

fn main() -> anyhow::Result<()> {
    EspLogger::initialize_default();
    let peripherals = Peripherals::take()?;
    fetch_time(peripherals.modem)?;

    loop {
        info!("Current time: {:?}", Local::now());
        sleep(Duration::from_secs(1));
    }
}
