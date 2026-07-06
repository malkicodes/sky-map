use std::f32::consts::PI;

use sfml::system::Vector2f;

use crate::view::View;

pub mod drawables;
pub mod star;

pub const SCREEN_SIZE: u32 = 1000;
pub const HALF_SCREEN_SIZE: u32 = SCREEN_SIZE / 2;

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

    let (o_lat, o_lng) = view.latlng();

    let k = 2. / (1. + o_lat.sin() * lat.sin() + o_lat.cos() * lat.cos() * (lng - o_lng).cos());

    (
        (k * lat.cos() * (lng - o_lng).sin()) as f32,
        (k * (o_lat.cos() * lat.sin() - o_lat.sin() * lat.cos() * (lng - o_lng).cos())) as f32,
    )
}

pub fn sterejec_to_screen(pos: (f32, f32), view: &View) -> Vector2f {
    Vector2f::new(pos.0, pos.1) * HALF_SCREEN_SIZE as f32 * view.zoom_v()
        + Vector2f::new(HALF_SCREEN_SIZE as f32, HALF_SCREEN_SIZE as f32)
}

pub mod settings;
pub mod view;
