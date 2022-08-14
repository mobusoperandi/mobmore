use std::{fmt::Display, ops::Add};
const MINUTES_PER_HOUR: i32 = 60;
const HOURS_PER_DAY: i32 = 24;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct Clock(i32);

impl Add for Clock {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self::from_minutes(self.0 + rhs.0)
    }
}
impl Display for Clock {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let hours = self.0 / MINUTES_PER_HOUR;
        let minutes = self.0 % MINUTES_PER_HOUR;
        write!(f, "{hours:02}:{minutes:02}")
    }
}

impl Clock {
    pub fn new(hours: i32, minutes: i32) -> Self {
        Self::from_hours(hours) + Self::from_minutes(minutes)
    }

    pub fn add_minutes(&self, minutes: i32) -> Self {
        *self + Self::from_minutes(minutes)
    }

    fn from_hours(hours: i32) -> Self {
        Self::from_minutes(hours * MINUTES_PER_HOUR)
    }

    fn from_minutes(minutes: i32) -> Self {
        Self(minutes.rem_euclid(HOURS_PER_DAY * MINUTES_PER_HOUR))
    }
}
