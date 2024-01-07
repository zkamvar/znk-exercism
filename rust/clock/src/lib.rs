use core::fmt::Debug;
use core::fmt::Formatter;
use core::fmt::Result;
use std::fmt::Display;

#[derive(Debug, PartialOrd, PartialEq)]
pub struct Clock {
    hours: i32,
    minutes: i32,
}

impl Display for Clock {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        let h = &self.hours;
        let m = &self.minutes;
        write!(f, "{h:02}:{m:02}")
    }
}

impl Clock {
    pub fn new(hours: i32, minutes: i32) -> Self {
        let time = parse_time(hours, minutes);
        Self {
            hours: time[0],
            minutes: time[1],
        }
    }

    pub fn add_minutes(&self, minutes: i32) -> Self {
        let time = parse_time(self.hours, &self.minutes + minutes);
        Self {
            hours: time[0],
            minutes: time[1],
        }
    }
}

fn parse_time(hours: i32, minutes: i32) -> Vec<i32> {
    let mut new_hours = parse_hours(hours);
    let mut new_minutes = &minutes.abs() % 60;
    let mut offset = 0;
    if minutes < 0 {
        new_minutes = 60 - new_minutes;
        if new_minutes > 0 {
            offset -= 1;
        }
    }
    if new_minutes > 59 {
        new_minutes = new_minutes % 60;
        if new_minutes == 0 {
            offset += 1;
        }
    }

    let excess = minutes / 60;
    new_hours = parse_hours(new_hours + excess + offset);

    return vec![new_hours, new_minutes];
}

fn parse_hours(hours: i32) -> i32 {
    let mut new_hours = &hours.abs() % 24;
    if hours < 0 {
        new_hours = 24 - new_hours;
    }
    if new_hours > 23 {
        new_hours = new_hours % 24;
    }
    new_hours
}
