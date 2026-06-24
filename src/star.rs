use arrayvec::ArrayString;
use serde::{Deserialize, Serialize};
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
}

impl Star {
    pub fn star_name(&self) -> String {
        self.name
            .clone()
            .or_else(|| self.bayer_name())
            .or_else(|| self.flamsteed_name())
            .unwrap_or_else(|| self.hr_name())
    }

    pub fn bayer_name(&self) -> Option<String> {
        self.bayer.as_ref().map(|b| b.to_string())
    }

    pub fn flamsteed_name(&self) -> Option<String> {
        self.flamsteed
            .map(|(number, constellation)| format!("{number} {constellation}"))
    }

    pub fn hr_name(&self) -> String {
        format!("HR {}", self.hr)
    }

    pub fn graphical_size(&self) -> f32 {
        const FALLOFF: f32 = 0.7;
        const MULTIPLIER: f32 = 3.;
        const MIN_SIZE: f32 = 0.5;

        (MULTIPLIER * FALLOFF.powf(self.vmag)).max(MIN_SIZE)
    }

    pub fn spherical_coordinates(&self) -> (f64, f64) {
        (
            self.ra * std::f64::consts::PI / 12.,
            self.dec * std::f64::consts::PI / 180.,
        )
    }
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

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
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
