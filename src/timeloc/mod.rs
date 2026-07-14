mod time;

use std::{f64::consts::PI, time::Duration};

pub use time::Time;

mod location;

pub use location::Location;

#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct TimeLocation {
    time: Time,
    location: Location,

    lst: f64,
}

impl TimeLocation {
    pub fn new(time: Time, location: Location) -> Self {
        let mut output = TimeLocation {
            time,
            location,
            lst: 0.,
        };

        output.recalculate_lst();

        output
    }

    /// Returns the local sidereal time in radians
    fn recalculate_lst(&mut self) {
        self.lst = gmst(&self.time) + self.location.lng()
    }

    pub fn lst(&self) -> f64 {
        self.lst
    }

    pub fn set_time(&mut self, time: Time) {
        self.time = time;
    }
}

// source: https://ssd.jpl.nasa.gov/planets/phys_par.html
pub const SIDEREAL_DAY: Duration = Duration::from_micros(86164100352);
pub const SIDEREAL_YEAR: Duration = Duration::from_micros(31558149102240);

// source: https://web.archive.org/web/20190430134555/http://aa.usno.navy.mil/publications/docs/c15_usb_online.pdf
pub const TROPICAL_YEAR: Duration = Duration::from_micros(31556925187471);

/// Returns GMST (Greenwich Mean Sidereal Time) for a certain Time in radians
pub fn gmst(time: &Time) -> f64 {
    // source: auass.com/wp-content/uploads/2021/01/Astronomical-Algorithms.pdf Chapter 12
    let days_since_j2000 = time.days_since(&Time::J2000);
    let centuries_since_j2000 = days_since_j2000 / 36525.;

    let gmst_0ut = 100.460_618_37
        + 36_000.770_053_608 * centuries_since_j2000
        + 0.000_387_933 * centuries_since_j2000.powi(2)
        + centuries_since_j2000.powi(3) / 38_710_000.;

    let instant_st = time.fract() * 360. * (86400. / SIDEREAL_DAY.as_secs_f64());

    loop_angle((gmst_0ut + instant_st) * PI / 180.)
}

fn modulo(x: f64, n: f64) -> f64 {
    x - n * (x / n).floor()
}

fn loop_angle(theta: f64) -> f64 {
    modulo(theta, 2. * PI)
}
