use std::f64::consts::FRAC_PI_2;

use crate::star::Star;

pub mod star;

pub fn sterejec(star: &Star, view: &View) -> (f32, f32) {
    let (mut lng, mut lat) = star.spherical_coordinates();

    lat = -lat; // ??
    lng = -lng;

    let k = 2.
        / (1.
            + view.o_lat.sin() * lat.sin()
            + view.o_lat.cos() * lat.cos() * (lng - view.o_lng).cos());

    (
        (k * lat.cos() * (lng - view.o_lng).sin()) as f32,
        (k * (view.o_lat.cos() * lat.sin()
            - view.o_lat.sin() * lat.cos() * (lng - view.o_lng).cos())) as f32,
    )
}

#[derive(Clone, Copy, Debug, Default)]
pub struct View {
    o_lat: f64,
    o_lng: f64,

    zoom: f32,
}

impl View {
    pub fn latlng(&self) -> (f64, f64) {
        (self.o_lat, self.o_lng)
    }

    pub fn change_latlng(&mut self, delta: (f64, f64)) {
        self.o_lng += delta.1;

        self.o_lat = (self.o_lat + delta.0).clamp(-FRAC_PI_2, FRAC_PI_2);
    }

    pub fn zoom(&self) -> f32 {
        self.zoom
    }

    pub fn set_zoom(&mut self, v: f32) {
        self.zoom = v
    }
}

#[derive(Clone, Debug, Default)]
pub struct DisplaySettings {
    names: NameSetting,
}

impl DisplaySettings {
    pub fn names(&self) -> NameSetting {
        self.names
    }

    pub fn cycle_names(&mut self) {
        self.names = match self.names {
            NameSetting::Proper => NameSetting::BayerFlamsteed,
            NameSetting::BayerFlamsteed => NameSetting::HR,
            NameSetting::HR => NameSetting::HD,
            NameSetting::HD => NameSetting::Hidden,
            NameSetting::Hidden => NameSetting::Proper,
        }
    }
}

#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub enum NameSetting {
    #[default]
    Proper,
    BayerFlamsteed,
    HR,
    HD,
    Hidden,
}
