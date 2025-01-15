use esp_idf_svc::hal::gpio::{Gpio0, Gpio1, Gpio10, Gpio18, Gpio19, Gpio2, Gpio20, Gpio21, Gpio3, Gpio4, Gpio5, Gpio6, Gpio7, Gpio8, Gpio9, InputOutput, Pin, PinDriver};
use esp_idf_svc::hal::peripherals::Peripherals;
use berlin_clock::LightState;

pub struct ClockPins<'a> {
    // pub seconds: PinDriver<'a, Gpio0, InputOutput>,
    pub five_minutes: FiveMinutesPins<'a>,
    pub minutes: MinutesPins<'a>,
}

impl ClockPins<'_> {
    pub fn display_five_minutes(&mut self, five_minutes_row: Vec<LightState>) {
        self.five_minutes.display(five_minutes_row)
    }

    pub fn display_minutes(&mut self, minutes_row: Vec<LightState>) {
        self.minutes.display(minutes_row)
    }

    // pub fn display_seconds(&mut self, val: LightState) {
    //     toggle(&mut self.seconds, val)
    // }
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
    pub first: PinDriver<'a, Gpio1, InputOutput>,
    pub second: PinDriver<'a, Gpio2, InputOutput>,
    pub third: PinDriver<'a, Gpio3, InputOutput>,
    pub forth: PinDriver<'a, Gpio4, InputOutput>,
}

impl MinutesPins<'_> {

    // fn new() -> Self {
    //
    // }
    fn display(&mut self, minutes_row: Vec<LightState>) {
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