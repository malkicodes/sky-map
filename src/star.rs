use arrayvec::ArrayString;
use phf::phf_ordered_map;
use serde::{Deserialize, Serialize};
use sfml::graphics::Color;
use std::{
    fmt::{Display, Write},
    str::FromStr,
};

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Star {
    pub name: Option<String>,

    pub hr: u16,

    pub bayer: Option<Bayer>,
    pub flamsteed: Option<(u8, ArrayString<3>)>,

    pub dm: (DurchmusterungSurvey, i8, u16, Option<char>),
    pub hd: u32,
    pub sao: Option<u32>,

    /// Right ascension in hours, equinox J2000, epoch 2000.0
    pub ra: f64,
    /// Declination in degrees, equinox J2000, epoch 2000.0
    pub dec: f64,

    pub vmag: f32,
    pub spectral_type: ArrayString<2>,
}

impl Star {
    pub fn star_name(&self) -> String {
        self.name
            .clone()
            .or_else(|| self.bayer_name())
            .or_else(|| self.flamsteed_name())
            .unwrap_or_else(|| self.hr_name())
    }

    pub fn bayerflamsteed_name(&self) -> String {
        self.bayer_name()
            .or_else(|| self.flamsteed_name())
            .unwrap_or_else(|| self.hr_name())
    }

    pub fn bayer_name(&self) -> Option<String> {
        self.bayer.as_ref().map(|b| b.to_string())
    }

    pub fn bayer(&self) -> Option<Bayer> {
        self.bayer
    }

    pub fn flamsteed_name(&self) -> Option<String> {
        self.flamsteed
            .map(|(number, constellation)| format!("{number} {constellation}"))
    }

    pub fn hr_name(&self) -> String {
        format!("HR {}", self.hr)
    }

    pub fn hd_name(&self) -> String {
        format!("HD {}", self.hd)
    }

    pub fn graphical_size(&self) -> f32 {
        const FALLOFF: f32 = 0.7;
        const MULTIPLIER: f32 = 3.;
        const MIN_SIZE: f32 = 0.5;

        (MULTIPLIER * FALLOFF.powf(self.vmag)).max(MIN_SIZE)
    }

    pub fn graphical_color(&self) -> Option<Color> {
        let temperature = spectral_to_temperature(self.spectral_type)?;

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

        Some(Color::rgb(red, green, blue))
    }

    pub fn spherical_coordinates(&self) -> (f64, f64) {
        (
            self.dec * std::f64::consts::PI / 180.,
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

fn spectral_to_temperature(spectral_type: ArrayString<2>) -> Option<f32> {
    let mut c = spectral_type.chars();
    let letter = c.next()?;
    let number = c.next()?.to_digit(10)?;

    let start = *(SPECTRAL_TEMPERATURES.get(&letter)?);
    let end = SPECTRAL_TEMPERATURES
        .index(SPECTRAL_TEMPERATURES.get_index(&letter)? + 1)
        .map(|x| *x.1)
        .unwrap_or(2400.);

    Some(start + (end - start) * (number as f32 / 10.))
}

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub enum DurchmusterungSurvey {
    BD,
    CD,
    CPD,
}

impl FromStr for DurchmusterungSurvey {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "BD" => Ok(Self::BD),
            "CD" => Ok(Self::CD),
            "CP" | "CPD" => Ok(Self::CPD),
            _ => Err(()),
        }
    }
}

#[derive(Deserialize, Serialize, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Bayer {
    pub letter: char,
    pub constellation: arrayvec::ArrayString<3>,
    pub superscript: Option<u8>,
}

impl Display for Bayer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_char(self.letter)?;

        if let Some(superscript) = self.superscript {
            f.write_char(match superscript {
                1 => '¹',
                2 => '²',
                3 => '³',
                n => char::from_u32(n as u32 + 0x2070).unwrap_or('ⁿ'),
            })?;
        }

        f.write_char(' ')?;

        f.write_str(&self.constellation)
    }
}
