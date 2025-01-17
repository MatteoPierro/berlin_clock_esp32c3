use berlin_clock::{LightState, Time};
use chrono::{Local, Timelike};
use chrono_tz::Tz;
use esp_idf_svc::eventloop::EspSystemEventLoop;
use esp_idf_svc::hal::gpio::{Gpio0, Gpio1, Gpio10, Gpio11, Gpio12, Gpio2, Gpio20, Gpio21, Gpio3, Gpio4, Gpio5, Gpio6, Gpio7, Gpio8, Gpio9, InputOutput, Pin, PinDriver};
use esp_idf_svc::hal::modem::Modem;
use esp_idf_svc::nvs::EspDefaultNvsPartition;
use esp_idf_svc::sntp;
use esp_idf_svc::sntp::EspSntp;
use esp_idf_svc::wifi::{AuthMethod, BlockingWifi, ClientConfiguration, Configuration, EspWifi};
use log::info;
pub struct Seconds<'a> {
    pub first: PinDriver<'a, Gpio0, InputOutput>,
}

impl<'a> Seconds<'a> {
    pub fn display(&mut self, seconds: LightState) {
        toggle(&mut self.first, seconds)
    }
}

pub struct FiveMinutesPins<'a> {
    pub first: PinDriver<'a, Gpio10, InputOutput>,
    pub second: PinDriver<'a, Gpio8, InputOutput>,
    pub third: PinDriver<'a, Gpio3, InputOutput>,
    pub forth: PinDriver<'a, Gpio4, InputOutput>,
    pub fifth: PinDriver<'a, Gpio9, InputOutput>,
    pub sixth: PinDriver<'a, Gpio1, InputOutput>,
    pub seventh: PinDriver<'a, Gpio2, InputOutput>,
    pub eighth: PinDriver<'a, Gpio0, InputOutput>,
    pub ninth: PinDriver<'a, Gpio7, InputOutput>,
    pub tenth: PinDriver<'a, Gpio6, InputOutput>,
    pub eleventh: PinDriver<'a, Gpio5, InputOutput>,
}

impl FiveMinutesPins<'_> {
    pub fn display(&mut self, five_minutes_row: Vec<LightState>) {
        toggle(&mut self.first, five_minutes_row[0]);
        toggle(&mut self.second, five_minutes_row[1]);
        toggle(&mut self.third, five_minutes_row[2]);
        toggle(&mut self.forth, five_minutes_row[3]);
        toggle(&mut self.fifth, five_minutes_row[4]);
        toggle(&mut self.sixth, five_minutes_row[5]);
        toggle(&mut self.seventh, five_minutes_row[6]);
        toggle(&mut self.eighth, five_minutes_row[7]);
        toggle(&mut self.ninth, five_minutes_row[8]);
        toggle(&mut self.tenth, five_minutes_row[9]);
        toggle(&mut self.eleventh, five_minutes_row[10]);
    }
}

pub struct MinutesPins<'a> {
    pub first: PinDriver<'a, Gpio20, InputOutput>,
    pub second: PinDriver<'a, Gpio21, InputOutput>,
    pub third: PinDriver<'a, Gpio10, InputOutput>,
    pub forth: PinDriver<'a, Gpio9, InputOutput>,
}

impl MinutesPins<'_> {
    pub fn display(&mut self, minutes_row: Vec<LightState>) {
        toggle(&mut self.first, minutes_row[0]);
        toggle(&mut self.second, minutes_row[1]);
        toggle(&mut self.third, minutes_row[2]);
        toggle(&mut self.forth, minutes_row[3]);
    }
}

pub struct HoursPins<'d> {
    pub first: PinDriver<'d, Gpio7, InputOutput>,
    pub second: PinDriver<'d, Gpio8, InputOutput>,
    pub third: PinDriver<'d, Gpio6, InputOutput>,
    pub forth: PinDriver<'d, Gpio5, InputOutput>
}

impl <'d> HoursPins<'d> {
    pub fn display(&mut self, minutes_row: Vec<LightState>) {
        toggle(&mut self.first, minutes_row[0]);
        toggle(&mut self.second, minutes_row[1]);
        toggle(&mut self.third, minutes_row[2]);
        toggle(&mut self.forth, minutes_row[3]);
    }
}

pub struct FiveHoursPins<'d> {
    pub first: PinDriver<'d, Gpio4, InputOutput>,
    pub second: PinDriver<'d, Gpio3, InputOutput>,
    pub third: PinDriver<'d, Gpio2, InputOutput>,
    pub forth: PinDriver<'d, Gpio1, InputOutput>
}

impl<'d> FiveHoursPins<'d> {
    pub fn display(&mut self, minutes_row: Vec<LightState>) {
        toggle(&mut self.first, minutes_row[0]);
        toggle(&mut self.second, minutes_row[1]);
        toggle(&mut self.third, minutes_row[2]);
        toggle(&mut self.forth, minutes_row[3]);
    }
}

fn toggle<T: Pin>(one: &mut PinDriver<T, InputOutput>, value: LightState) {
    if value == LightState::On {
        one.set_high().unwrap();
    } else {
        one.set_low().unwrap();
    }
}

const SSID: &str = env!("SSID");
const PASSWORD: &str = env!("PASSWORD");

pub fn fetch_time(modem: Modem) -> anyhow::Result<(BlockingWifi<EspWifi<'static>>, EspSntp<'static>)> {
    let sys_loop = EspSystemEventLoop::take()?;
    let nvs = EspDefaultNvsPartition::take()?;

    let mut wifi = BlockingWifi::wrap(
        EspWifi::new(modem, sys_loop.clone(), Some(nvs)).unwrap(),
        sys_loop,
    )?;

    connect_wifi(&mut wifi)?;

    // Keep it around or else the SNTP service will stop
    let sntp = sntp::EspSntp::new_default()?;
    info!("SNTP initialized");
    Ok((wifi, sntp))
}

pub fn connect_wifi(wifi: &mut BlockingWifi<EspWifi<'static>>) -> anyhow::Result<()> {
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

pub fn current_time() -> Time {
    let now = Local::now().with_timezone(&Tz::Europe__Helsinki);
    info!("time {}", now);
    Time {
        hours: now.hour() as usize,
        minutes: now.minute() as usize,
        seconds: now.second() as usize,
    }
}
