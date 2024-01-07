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
    let mut new_minutes = &minutes % 60;
    if minutes < 0 {
        new_minutes = 60 + new_minutes;
    }
    if new_minutes > 59 {
        new_minutes = new_minutes % 60;
    }

    let excess = minutes / 60;
    new_hours = parse_hours(new_hours + excess);

    return vec![new_hours, new_minutes];
}

fn parse_hours(hours: i32) -> i32 {
    let mut new_hours = &hours % 24;
    if new_hours < 0 {
        new_hours = 24 + new_hours;
    }
    if new_hours > 23 {
        new_hours = new_hours % 24;
    }
    new_hours
}
