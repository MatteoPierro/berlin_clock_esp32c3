use crate::LightState::{Off, On};

pub struct Time {
    hours: usize,
    minutes: usize,
    seconds: usize,
}

#[derive(PartialEq, Debug)]
pub struct BerlinClock {
    five_hours: Vec<LightState>,
    hours: Vec<LightState>,
    five_minutes: Vec<LightState>,
    pub minutes: Vec<LightState>,
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum LightState {
    On,
    Off,
}

pub fn time(time: &str) -> Time {
    let parts = time.split(":").collect::<Vec<&str>>();

    Time {
        hours: parts[0].parse::<usize>().unwrap(),
        minutes: parts[1].parse::<usize>().unwrap(),
        seconds: parts[2].parse::<usize>().unwrap(),
    }
}

fn minutes_row(time: &Time) -> Vec<LightState> {
    let lights_in_minute_row = 4;
    let lights_on = time.minutes % 5;

    build_lights_row(lights_in_minute_row, lights_on)
}

fn five_minutes_row(time: &Time) -> Vec<LightState> {
    let lights_in_five_minutes_row = 11;
    let lights_on = time.minutes / 5;

    build_lights_row(lights_in_five_minutes_row, lights_on)
}

fn hours_row(time: &Time) -> Vec<LightState> {
    let lights_in_hours_row = 4;
    let lights_on = time.hours % 5;

    build_lights_row(lights_in_hours_row, lights_on)
}

fn five_hours_row(time: &Time) -> Vec<LightState> {
    let lights_in_five_hours_row = 4;
    let lights_on = time.hours / 5;

    build_lights_row(lights_in_five_hours_row, lights_on)
}

fn build_lights_row(lights_in_row: usize, lights_on: usize) -> Vec<LightState> {
    let lights_off = lights_in_row - lights_on;
    [vec![On; lights_on], vec![Off; lights_off]].concat()
}

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

pub fn berlin_clock(time: Time) -> BerlinClock {
    BerlinClock {
        five_hours: five_hours_row(&time),
        hours: hours_row(&time),
        five_minutes: five_minutes_row(&time),
        minutes: minutes_row(&time),
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    use LightState::{On, Off};

    #[test]
    fn it_displays_the_berlin_clock() {
        assert_eq!(
            berlin_clock(time("00:00:00")),
            BerlinClock {
                five_hours: vec![Off, Off, Off, Off],
                hours: vec![Off, Off, Off, Off],
                five_minutes: vec![Off, Off, Off, Off, Off, Off, Off, Off, Off, Off, Off],
                minutes: vec![Off, Off, Off, Off],
            }
        );

        assert_eq!(
            berlin_clock(time("07:19:00")),
            BerlinClock {
                five_hours: vec![On, Off, Off, Off],
                hours: vec![On, On, Off, Off],
                five_minutes: vec![On, On, On, Off, Off, Off, Off, Off, Off, Off, Off],
                minutes: vec![On, On, On, On],
            }
        )
    }

    #[test]
    fn it_displays_the_five_hours_row() {
        assert_eq!(five_hours_row(&time("00:00:00")), vec![Off, Off, Off, Off]);
        assert_eq!(five_hours_row(&time("01:00:00")), vec![Off, Off, Off, Off]);
        assert_eq!(five_hours_row(&time("04:00:00")), vec![Off, Off, Off, Off]);
        assert_eq!(five_hours_row(&time("05:00:00")), vec![On, Off, Off, Off]);
        assert_eq!(five_hours_row(&time("06:00:00")), vec![On, Off, Off, Off]);
        assert_eq!(five_hours_row(&time("10:00:00")), vec![On, On, Off, Off]);
        assert_eq!(five_hours_row(&time("11:00:00")), vec![On, On, Off, Off]);
        assert_eq!(five_hours_row(&time("15:00:00")), vec![On, On, On, Off]);
        assert_eq!(five_hours_row(&time("20:00:00")), vec![On, On, On, On]);
    }

    #[test]
    fn it_displays_the_hours_row() {
        assert_eq!(hours_row(&time("00:00:00")), vec![Off, Off, Off, Off]);
        assert_eq!(hours_row(&time("01:00:00")), vec![On, Off, Off, Off]);
        assert_eq!(hours_row(&time("02:00:00")), vec![On, On, Off, Off]);
        assert_eq!(hours_row(&time("03:00:00")), vec![On, On, On, Off]);
        assert_eq!(hours_row(&time("04:00:00")), vec![On, On, On, On]);
        assert_eq!(hours_row(&time("15:00:00")), vec![Off, Off, Off, Off]);
    }

    #[test]
    fn it_displays_the_five_minutes_row() {
        assert_eq!(
            five_minutes_row(&time("00:00:00")),
            vec![Off, Off, Off, Off, Off, Off, Off, Off, Off, Off, Off]
        );
        assert_eq!(
            five_minutes_row(&time("00:05:00")),
            vec![On, Off, Off, Off, Off, Off, Off, Off, Off, Off, Off]
        );
        assert_eq!(
            five_minutes_row(&time("00:10:00")),
            vec![On, On, Off, Off, Off, Off, Off, Off, Off, Off, Off]
        );
        assert_eq!(
            five_minutes_row(&time("00:15:00")),
            vec![On, On, On, Off, Off, Off, Off, Off, Off, Off, Off]
        );
        assert_eq!(
            five_minutes_row(&time("00:16:00")),
            vec![On, On, On, Off, Off, Off, Off, Off, Off, Off, Off]
        );
        assert_eq!(
            five_minutes_row(&time("00:20:00")),
            vec![On, On, On, On, Off, Off, Off, Off, Off, Off, Off]
        );
        assert_eq!(
            five_minutes_row(&time("00:59:00")),
            vec![On, On, On, On, On, On, On, On, On, On, On]
        );
    }

    #[test]
    fn it_displays_minutes_row() {
        assert_eq!(minutes_row(&time("00:00:00")), vec![Off, Off, Off, Off]);
        assert_eq!(minutes_row(&time("00:01:00")), vec![On, Off, Off, Off]);
        assert_eq!(minutes_row(&time("00:07:00")), vec![On, On, Off, Off]);
        assert_eq!(minutes_row(&time("00:08:00")), vec![On, On, On, Off]);
        assert_eq!(minutes_row(&time("00:14:00")), vec![On, On, On, On]);
        assert_eq!(minutes_row(&time("00:59:00")), vec![On, On, On, On]);
    }

    #[test]
    fn exploration() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
