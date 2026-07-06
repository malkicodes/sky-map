use std::{f32::consts::PI, f64::consts::FRAC_PI_2};

use sfml::system::Vector2f;

pub mod drawables;
pub mod star;

pub const SCREEN_SIZE: u32 = 1000;
pub const HALF_SCREEN_SIZE: u32 = 500;

pub fn rad_to_hms(rad: f32) -> (i16, u16, f32) {
    let hours = (rad - 2. * PI * (rad / (2. * PI)).floor()) * 12. / PI;
    let minutes = hours.fract() * 60.;
    let seconds = minutes.fract() * 60.;

    (hours.floor() as i16, minutes.floor() as u16, seconds)
}

pub fn rad_to_dms(rad: f32) -> (i16, u16, f32) {
    let degrees = rad.abs() * 180. / PI;
    let arcminutes = degrees.fract() * 60.;
    let arcseconds = arcminutes.fract() * 60.;

    (
        degrees.floor() as i16 * if rad.is_sign_positive() { -1 } else { 1 },
        arcminutes.floor() as u16,
        arcseconds,
    )
}

pub fn sterejec(pos: (f64, f64), view: &View) -> (f32, f32) {
    let lat = -pos.0;
    let lng = -pos.1;

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

pub fn sterejec_to_screen(pos: (f32, f32), view: &View) -> Vector2f {
    Vector2f::new(pos.0, pos.1) * HALF_SCREEN_SIZE as f32 * view.zoom_v
        + Vector2f::new(HALF_SCREEN_SIZE as f32, HALF_SCREEN_SIZE as f32)
}

#[derive(Clone, Copy, Debug)]
pub struct View {
    o_lat: f64,
    o_lng: f64,

    zoom: f32,
    zoom_v: f32,
}

impl Default for View {
    fn default() -> Self {
        Self {
            o_lat: 0.,
            o_lng: 0.,
            zoom: 1.,
            zoom_v: 1.,
        }
    }
}

impl View {
    pub fn latlng(&self) -> (f64, f64) {
        (self.o_lat, -self.o_lng)
    }

    pub fn change_latlng(&mut self, delta: (f64, f64)) {
        self.o_lng += delta.1;

        self.o_lat = (self.o_lat + delta.0).clamp(-FRAC_PI_2, FRAC_PI_2);
    }

    pub fn zoom(&self) -> f32 {
        self.zoom
    }

    pub fn zoom_v(&self) -> f32 {
        self.zoom_v
    }

    pub fn set_zoom(&mut self, v: f32) {
        self.zoom = v;
        self.zoom_v = 2_f32.powf(v) * 0.5;
    }
}

#[derive(Clone, Debug, Default)]
pub struct DisplaySettings {
    names: NameSetting,
    grid: GridSetting,
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

    pub fn grid(&self) -> GridSetting {
        self.grid
    }

    pub fn cycle_grid(&mut self) {
        self.grid = match self.grid {
            GridSetting::MajorMinor => GridSetting::Major,
            GridSetting::Major => GridSetting::Horizon,
            GridSetting::Horizon => GridSetting::None,
            GridSetting::None => GridSetting::MajorMinor,
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

#[derive(Clone, Copy, Debug, Default, PartialEq, PartialOrd)]
pub enum GridSetting {
    None,
    Horizon,
    Major,
    #[default]
    MajorMinor,
}
