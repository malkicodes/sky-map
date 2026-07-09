use arrayvec::ArrayString;
use phf::phf_ordered_map;
use serde::Deserialize;
use sfml::graphics::Color;

use crate::{GREEK_LETTERS, SUBSCRIPT_CHARS, SUPERSCRIPT_CHARS};

#[derive(Deserialize, Debug, Clone, PartialEq)]
pub struct Star {
    pub(crate) hd: u32,

    /// Right ascension in hours, equinox J2000, epoch 2000.0
    pub(crate) ra: f64,
    /// Declination in degrees, equinox J2000, epoch 2000.0
    pub(crate) de: f64,

    pub(crate) mag: f32,
    pub(crate) spt: Option<(char, u8)>,

    names: Option<StarNames>,
}

#[derive(Deserialize, Debug, Default, Clone, PartialEq)]
struct StarNames {
    dm: ArrayString<12>,
    gc: Option<u16>,
    hr: Option<u16>,
    hip: Option<u32>,
    fl: Option<u16>,
    bayer: Option<ArrayString<5>>,
    prop: Option<ArrayString<28>>,
    cst: ArrayString<3>,
}

impl Star {
    pub const DEFAULT_COLOR: Color = Color::WHITE;

    pub fn apparent_magnitude(&self) -> f32 {
        self.mag
    }

    pub fn star_name(&self) -> String {
        self.simbad_id()
    }

    pub fn bayerflamsteed_name(&self) -> Option<String> {
        self.bayer_name().or_else(|| self.flamsteed_name())
    }

    fn format_bayer(bayer_str: &str) -> String {
        let segments = bayer_str.trim_ascii().split_at(
            bayer_str
                .char_indices()
                .find(|(_, c)| c.is_ascii_digit())
                .unwrap_or((bayer_str.len(), ' '))
                .0,
        );

        let first_segment = segments.0;
        let mut output = GREEK_LETTERS
            .get(first_segment)
            .map(|x| x.to_string())
            .unwrap_or_else(|| first_segment.to_owned());
        if (!segments.1.trim().is_empty())
            && let Ok(n) = segments.1.trim().parse::<u16>()
        {
            if first_segment.starts_with(|x: char| x.is_ascii_uppercase()) {
                for digit in n
                    .to_string()
                    .chars()
                    .filter_map(|c| c.to_digit(10))
                    .map(|d| d as usize)
                {
                    output.push(SUBSCRIPT_CHARS[digit]);
                }
            } else {
                for digit in n
                    .to_string()
                    .chars()
                    .filter_map(|c| c.to_digit(10))
                    .map(|d| d as usize)
                {
                    output.push(SUPERSCRIPT_CHARS[digit]);
                }
            }
        } else if !segments.1.is_empty() {
            eprintln!(
                "WARNING: meow :3 {} {:?}",
                segments.1,
                segments.1.trim().parse::<u16>()
            )
        }
        output
    }

    pub fn bayer_name(&self) -> Option<String> {
        match &self.names {
            Some(names) => names
                .bayer
                .map(|bayer| format!("{} {}", Self::format_bayer(&bayer), names.cst)),
            None => None,
        }
    }

    pub fn flamsteed_name(&self) -> Option<String> {
        match &self.names {
            Some(names) => names.fl.map(|number| format!("{number} {}", names.cst)),
            None => None,
        }
    }

    pub fn hd_name(&self) -> String {
        format!("HD {}", self.hd)
    }

    pub fn simbad_id(&self) -> String {
        match &self.names {
            Some(names) => names
                .bayer
                .map_or_else(|| names.fl.map(|n| n.to_string()), |b| Some(b.to_string()))
                .map_or_else(|| self.hd_name(), |s| format!("* {s} {}", names.cst)),
            None => self.hd_name(),
        }
    }

    pub fn proper_name(&self) -> Option<String> {
        (&self.names)
            .as_ref()?
            .prop
            .map(|x| String::from(x.as_str()))
    }

    pub fn graphical_size(&self) -> f32 {
        const FALLOFF: f32 = 0.65; // irl value: 5th root of 0.01 ~= 0.398
        const MULTIPLIER: f32 = 5.;
        const MIN_SIZE: f32 = 0.0;
        const MAX_SIZE: f32 = 5.0;

        (MULTIPLIER * FALLOFF.powf(self.mag)).clamp(MIN_SIZE, MAX_SIZE)
    }

    pub fn graphical_color(&self) -> Color {
        if let Some(col) = self.spt.and_then(|spt| spectral_to_rgb(spt)) {
            col
        } else {
            Self::DEFAULT_COLOR
        }
    }

    pub fn spherical_coordinates(&self) -> (f64, f64) {
        (
            self.de * std::f64::consts::PI / 180.,
            self.ra * std::f64::consts::PI / 12.,
        )
    }
}

static SPECTRAL_TEMPERATURES: phf::OrderedMap<char, f32> = phf_ordered_map! {
    // https://astro.unl.edu/naap/hr/hr_background1.html
    'O' => 40000.,
    'B' => 20000.,
    'A' => 10000.,
    'F' => 7500.,
    'G' => 5500.,
    'K' => 4000.,
    'M' => 3000.,
};

fn spectral_to_temperature(spectral_type: (char, u8)) -> Option<f32> {
    let (letter, number) = spectral_type;

    let start = *(SPECTRAL_TEMPERATURES.get(&letter)?);
    let end = SPECTRAL_TEMPERATURES
        .index(SPECTRAL_TEMPERATURES.get_index(&letter)? + 1)
        .map(|x| *x.1)
        .unwrap_or(2400.);

    Some(start + (end - start) * (number as f32 / 10.))
}

fn temperature_to_rgb(temperature: f32) -> Color {
    // https://tannerhelland.com/2012/09/18/convert-temperature-rgb-algorithm-code.html

    let temp = temperature / 100.;

    let red = if temp <= 66. {
        255
    } else {
        (329.698727446 * (temp - 60.).powf(-0.1332047592))
            .clamp(0., 255.)
            .round() as u8
    };

    let green = if temp <= 66. {
        (99.4708025861 * temp.ln() - 161.1195681661)
            .clamp(0., 255.)
            .round() as u8
    } else {
        (288.1221695283 * ((temp - 60.).powf(-0.0755148492)))
            .clamp(0., 255.)
            .round() as u8
    };

    let blue = if temp >= 66. {
        255
    } else if temp <= 19. {
        0
    } else {
        (138.5177312231 * (temp - 10.).ln() - 305.0447927307)
            .clamp(0., 255.)
            .round() as u8
    };

    Color::rgb(red, green, blue)
}

fn spectral_to_rgb(spectral_type: (char, u8)) -> Option<Color> {
    Some(temperature_to_rgb(spectral_to_temperature(spectral_type)?))
}

pub fn load_stars() -> Vec<Star> {
    let mut stars: Vec<Star> = serde_json::from_str(include_str!("../assets/stars.json")).unwrap();
    stars.sort_by(|a, b| a.apparent_magnitude().total_cmp(&b.apparent_magnitude()));

    stars
}
