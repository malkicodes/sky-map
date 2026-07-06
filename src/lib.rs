use std::f32::consts::PI;

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

pub mod settings;
pub mod view;
