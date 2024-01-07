use core::cmp::Ordering;
use core::fmt::Debug;
use std::fmt::Display;

#[derive(Debug)]
pub struct Clock<T> {
    hours: T,
    minutes: T,
}

impl<T: Display> ToString for Clock<T> {
    fn to_string(&self) -> String {
        let h = &self.hours;
        let m = &self.minutes;
        format!("{h:02}:{m:02}")
    }
}

impl<T: Display + Debug> PartialOrd for Clock<T> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.to_string().partial_cmp(&other.to_string())
    }
}

impl<T: Display + Debug> PartialEq for Clock<T> {
    fn eq(&self, other: &Self) -> bool {
        self.to_string() == other.to_string()
    }
}

impl Clock<i32> {
    pub fn new(hours: i32, minutes: i32) -> Self {
        Self { hours, minutes }
    }

    pub fn add_minutes(&self, minutes: i32) -> Self {
        Self {
            hours: self.hours,
            minutes: self.minutes + minutes,
        }
    }
}
