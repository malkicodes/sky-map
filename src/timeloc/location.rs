use std::f64::consts::PI;

#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct Location {
    /// Latitude in degrees
    lat: f64,
    /// Longitude in degrees
    lng: f64,
}

impl Location {
    /// Returns `None` if latitude is not between -90 and 90 or if longitude is not between -180 and 180
    pub const fn new(lat: f64, lng: f64) -> Option<Self> {
        if !(lat.is_normal() && lng.is_normal()) {
            return None;
        }

        if lat.abs() > 90. {
            return None;
        }

        if lng.abs() > 180. {
            return None;
        }

        Some(Self { lat, lng })
    }

    pub const fn lat_degrees(&self) -> f64 {
        self.lat
    }

    pub const fn lng_degrees(&self) -> f64 {
        self.lng
    }

    pub const fn lat(&self) -> f64 {
        self.lat * PI / 180.
    }

    pub const fn lng(&self) -> f64 {
        self.lng * PI / 180.
    }
}
