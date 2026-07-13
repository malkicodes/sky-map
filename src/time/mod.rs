use std::{
    ops::{Add, Sub},
    time::Duration,
};

use chrono::{DateTime, TimeZone, Utc};

const SECONDS_PER_DAY: f64 = 86400.0;
const MICROSECONDS_PER_DAY: f64 = SECONDS_PER_DAY * 1_000_000.;

#[derive(Clone, Copy, Debug, PartialEq, PartialOrd)]
pub struct Time(f64);

impl Time {
    pub const JD_EPOCH: Self = Time(-2400000.5);
    pub const J2000: Self = Time(51544.5);
    pub const UNIX_EPOCH: Self = Time(40587.0);

    pub const fn from_mjd(mjd: f64) -> Time {
        Time(mjd)
    }

    pub const fn from_jd(jd: f64) -> Time {
        Time(jd + Self::JD_EPOCH.mjd())
    }

    pub const fn mjd(&self) -> f64 {
        self.0
    }

    pub const fn jd(&self) -> f64 {
        self.0 - Self::JD_EPOCH.mjd()
    }

    pub const fn days_since(&self, other: &Time) -> f64 {
        self.0 - other.0
    }

    pub const fn years_since(&self, other: &Time) -> f64 {
        self.days_since(other) / 365.25
    }

    /// Returns `None` if self < other
    pub const fn duration_since(&self, other: &Time) -> Option<Duration> {
        if self.0 < other.0 {
            return None;
        }

        let days = self.days_since(other);
        let secs = days * SECONDS_PER_DAY;
        let nanos = (secs.fract() * 1_000_000_000.).round() as u32;

        Some(Duration::new(secs as u64, nanos))
    }
}

impl Default for Time {
    fn default() -> Self {
        Self::J2000
    }
}

impl<Tz: TimeZone> Into<Time> for DateTime<Tz> {
    fn into(self) -> Time {
        Time(Time::UNIX_EPOCH.0 + self.timestamp_micros() as f64 / MICROSECONDS_PER_DAY)
    }
}

impl Into<DateTime<Utc>> for Time {
    fn into(self) -> DateTime<Utc> {
        DateTime::from_timestamp_micros(
            ((self.0 - Time::UNIX_EPOCH.0) * MICROSECONDS_PER_DAY).round() as i64,
        )
        .unwrap()
    }
}

impl Add<Duration> for Time {
    type Output = Time;

    fn add(self, rhs: Duration) -> Self::Output {
        Time::from_mjd(self.mjd() + rhs.as_secs_f64() / SECONDS_PER_DAY)
    }
}

impl Sub<Duration> for Time {
    type Output = Time;

    fn sub(self, rhs: Duration) -> Self::Output {
        Time::from_mjd(self.mjd() - rhs.as_secs_f64() / SECONDS_PER_DAY)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn into_chrono() {
        let j2000 = Time::J2000;

        let datetime: DateTime<Utc> = j2000.into();
        let target = DateTime::from_timestamp(946728000, 0).unwrap(); // source: https://www.epochconverter.com/

        assert_eq!(datetime, target)
    }

    #[test]
    fn durations() {
        let a = Time::J2000;
        let b = Time::UNIX_EPOCH;

        assert_eq!(a.years_since(&b), 30.);
        assert_eq!(
            dbg!(a.duration_since(&b).unwrap()),
            Duration::from_secs_f64(30. * 365.25 * SECONDS_PER_DAY)
        );
    }

    #[test]
    fn duration_ops() {
        assert_eq!(
            (Time::J2000 + Duration::new(86400, 0)).mjd(),
            Time::J2000.mjd() + 1.
        );

        assert_eq!(
            (Time::UNIX_EPOCH - Duration::new(86400 * 2, 0)).mjd(),
            Time::UNIX_EPOCH.mjd() - 2.
        );
    }
}
