use chrono::Local;
use core::convert::TryInto;
use esp_idf_svc::hal::prelude::Peripherals;
use esp_idf_svc::log::EspLogger;
use esp_idf_svc::wifi::{AuthMethod, BlockingWifi, ClientConfiguration, Configuration, EspWifi};
use esp_idf_svc::{eventloop::EspSystemEventLoop, nvs::EspDefaultNvsPartition, sntp};
use esp_idf_svc::hal::modem::Modem;
use log::info;

const SSID: &str = env!("SSID");
const PASSWORD: &str = env!("PASSWORD");

fn main() -> anyhow::Result<()> {
    EspLogger::initialize_default();
    let peripherals = Peripherals::take()?;
    fetch_time(peripherals.modem)?;

    loop {
        // To get a better formatting of the time, you can use the `chrono` or `time` Rust crates
        info!("Current time: {:?}", Local::now());
        std::thread::sleep(std::time::Duration::from_secs(1));
    }
}

fn fetch_time(modem: Modem) ->  anyhow::Result<()> {
    let sys_loop = EspSystemEventLoop::take()?;
    let nvs = EspDefaultNvsPartition::take()?;

    let mut wifi = BlockingWifi::wrap(
        EspWifi::new(modem, sys_loop.clone(), Some(nvs)).unwrap(),
        sys_loop,
    )?;

    connect_wifi(&mut wifi)?;

    // Keep it around or else the SNTP service will stop
    let _sntp = sntp::EspSntp::new_default()?;
    info!("SNTP initialized");
    Ok(())
}

fn connect_wifi(wifi: &mut BlockingWifi<EspWifi<'static>>) -> anyhow::Result<()> {
    let wifi_configuration: Configuration = Configuration::Client(ClientConfiguration {
        ssid: SSID.try_into().unwrap(),
        bssid: None,
        auth_method: AuthMethod::WPA2Personal,
        password: PASSWORD.try_into().unwrap(),
        channel: None,
        ..Default::default()
    });

    wifi.set_configuration(&wifi_configuration)?;

    wifi.start()?;
    info!("Wifi started");

    wifi.connect()?;
    info!("Wifi connected");

    wifi.wait_netif_up()?;
    info!("Wifi netif up");

    Ok(())
}
