use std::time::SystemTime;
use chrono::{Local, Timelike};
use berlin_clock::add;

fn main() {
    // It is necessary to call this function once. Otherwise some patches to the runtime
    // implemented by esp-idf-sys might not link properly. See https://github.com/esp-rs/esp-idf-template/issues/71
    esp_idf_svc::sys::link_patches();

    // Bind the log crate to the ESP Logging facilities
    esp_idf_svc::log::EspLogger::initialize_default();

    let now = Local::now();
    log::info!("Hello, {}", now);
    log::info!("Adding {}", add(2, 3));
}
