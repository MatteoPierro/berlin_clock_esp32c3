use esp_idf_svc::hal::gpio::{Gpio0, Gpio1, Gpio2, Gpio3, Gpio4, InputOutput, Pin, PinDriver};
use esp_idf_svc::hal::peripherals::Peripherals;
use berlin_clock::LightState;

pub struct CircuitPins<'a> {
    pub seconds: PinDriver<'a, Gpio0, InputOutput>,
    pub minutes: MinutesPins<'a>,
}

impl CircuitPins<'_> {

    pub fn new() -> Self {
        let peripherals = Peripherals::take().unwrap();
        let zero = PinDriver::input_output(peripherals.pins.gpio0).unwrap();
        let one = PinDriver::input_output(peripherals.pins.gpio1).unwrap();
        let two = PinDriver::input_output(peripherals.pins.gpio2).unwrap();
        let three = PinDriver::input_output(peripherals.pins.gpio3).unwrap();
        let four = PinDriver::input_output(peripherals.pins.gpio4).unwrap();

        let minutes = MinutesPins {
            first: one,
            second: two,
            third: three,
            forth: four,
        };

        CircuitPins {
            seconds: zero,
            minutes,
        }
    }
    pub fn display_minutes(&mut self, minutes_row: Vec<LightState>) {
        self.minutes.display(minutes_row)
    }

    pub fn display_seconds(&mut self, val: LightState) {
        toggle(&mut self.seconds, val)
    }
}

pub struct MinutesPins<'a> {
    pub first: PinDriver<'a, Gpio1, InputOutput>,
    pub second: PinDriver<'a, Gpio2, InputOutput>,
    pub third: PinDriver<'a, Gpio3, InputOutput>,
    pub forth: PinDriver<'a, Gpio4, InputOutput>,
}

impl MinutesPins<'_> {
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