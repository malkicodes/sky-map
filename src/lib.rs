use std::f32::consts::PI;

use phf::phf_map;

use crate::view::View;

pub mod drawables;
pub mod star;

pub mod colors {
    use sfml::graphics::Color;

    pub const BACKGROUND_COLOR: Color = Color::BLACK;
    pub const GRID_MAJOR_COLOR: Color = Color::rgba(255, 255, 255, 64);
    pub const GRID_MINOR_COLOR: Color = Color::rgba(255, 255, 255, 32);
    pub const CONSTELLATION_COLOR: Color = Color::rgb(64, 64, 128);
}

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

pub static GREEK_LETTERS: phf::Map<&'static str, char> = phf_map! {
    "alp" | "alf" => 'α',
    "bet" => 'β',
    "gam" => 'γ',
    "del" => 'δ',
    "eps" => 'ε',
    "zet" => 'ζ',
    "eta" => 'η',
    "the" | "tet" => 'θ',
    "iot" => 'ι',
    "kap" => 'κ',
    "lam" => 'λ',
    "mu" | "mu." => 'μ',
    "nu" | "nu." => 'ν',
    "xi" | "ksi" => 'ξ',
    "omi" => 'ο',
    "pi" | "pi." => 'π',
    "rho" => 'ρ',
    "sig" => 'σ',
    "tau" => 'τ',
    "ups" => 'υ',
    "phi" => 'φ',
    "chi" => 'χ',
    "psi" => 'ψ',
    "ome" => 'ω'
};

pub const SUPERSCRIPT_CHARS: [char; 10] = ['⁰', '¹', '²', '³', '⁴', '⁵', '⁶', '⁷', '⁸', '⁹'];
pub const SUBSCRIPT_CHARS: [char; 10] = ['₀', '₁', '₂', '₃', '₄', '₅', '₆', '₇', '₈', '₉'];

pub mod settings;
pub mod view;
