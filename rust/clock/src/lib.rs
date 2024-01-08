use core::fmt::Debug;
use core::fmt::Formatter;
use core::fmt::Result;
use std::fmt::Display;

#[derive(Debug, PartialOrd, PartialEq)]
pub struct Clock {
    minutes: i32,
}

impl Display for Clock {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        let h = get_hour(self.minutes).abs();
        let m = get_minutes(self.minutes).abs();
        write!(f, "{h:02}:{m:02}")
    }
}

impl Clock {
    pub fn new(hours: i32, minutes: i32) -> Self {
        let h = hours.rem_euclid(24) * 60;
        Self {
            minutes: (h + minutes).rem_euclid(1440),
        }
    }

    pub fn add_minutes(&self, minutes: i32) -> Self {
        Self {
            minutes: (&self.minutes + minutes).rem_euclid(1440),
        }
    }
}

fn get_hour(minutes: i32) -> i32 {
    minutes.div_euclid(60).rem_euclid(24)
}

fn get_minutes(minutes: i32) -> i32 {
    minutes.rem_euclid(60)
}
