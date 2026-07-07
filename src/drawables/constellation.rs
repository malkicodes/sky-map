use std::{
    collections::HashMap,
    fs::{self, File},
};

use arrayvec::ArrayString;
use sfml::{
    SfResult,
    cpp::FBox,
    graphics::{Color, Drawable, PrimitiveType, Vertex, VertexBuffer, VertexBufferUsage},
};

use crate::{
    GREEK_LETTERS, View,
    settings::DisplaySettings,
    star::{Bayer, Star},
};

#[derive(Debug, Clone)]
pub struct Constellation {
    vb: FBox<VertexBuffer>,
    vertices: Vec<Vertex>,
    /// Index for each star from original star data list
    star_info: Vec<(usize, (f64, f64))>,

    name: String,
}

impl Constellation {
    pub fn new(lines: Vec<[usize; 2]>, star_data: &[Star], name: String) -> SfResult<Self> {
        let mut star_info: Vec<(usize, (f64, f64))> = Vec::with_capacity(lines.len() * 2);

        for star_i in lines.iter().flatten().copied() {
            star_info.push((star_i, star_data[star_i].spherical_coordinates()));
        }

        let vertices = vec![Vertex::DEFAULT; star_info.len()];
        let vb = VertexBuffer::new(
            PrimitiveType::LINES,
            vertices.len(),
            VertexBufferUsage::STREAM,
        )?;

        Ok(Self {
            vb,
            vertices,
            star_info,

            name,
        })
    }
    
    const LINE_COLOR: Color = Color { a: 128, ..Color::BLUE };

    pub fn update(&mut self, view: &View, _settings: &DisplaySettings) -> SfResult<()> {
        for (v, (_i, coords)) in self.vertices.iter_mut().zip(self.star_info.iter().copied()) {
            *v = Vertex::with_pos_color(view.project_to_screen(coords), Self::LINE_COLOR);
        }
        self.vb.update(&self.vertices, 0)
    }
}

impl Drawable for Constellation {
    fn draw<'a: 'shader, 'texture, 'shader, 'shader_texture>(
        &'a self,
        target: &mut dyn sfml::graphics::RenderTarget,
        rs: &sfml::graphics::RenderStates<'texture, 'shader, 'shader_texture>,
    ) {
        target.draw_vertex_buffer(&self.vb, rs);
    }
}

mod constellation_data {
    use serde::Deserialize;

    #[derive(Debug, Clone, Deserialize)]
    pub(super) struct ConstellationDataset {
        pub(super) id: String,
        pub(super) name: String,
        pub(super) region: String,
        pub(super) period: String,
        pub(super) constellations: Vec<ConstellationData>,
    }

    #[derive(Debug, Clone, Deserialize)]
    pub(super) struct ConstellationData {
        pub(super) id: String,
        pub(super) names: Vec<ConstellationName>,
        pub(super) lines: Vec<Vec<String>>,
        pub(super) semantics: Vec<String>,
    }

    #[derive(Debug, Clone, Deserialize)]
    pub(super) struct ConstellationName {
        pub(super) english: String,
        pub(super) native: String,
    }
}

pub fn load_constellations(star_data: &[Star]) -> Vec<Constellation> {
    let dataset: constellation_data::ConstellationDataset =
        serde_json::from_reader(File::open("./assets/constellations.json").unwrap()).unwrap();

    let mut constellations = Vec::with_capacity(dataset.constellations.len());

    for constellation in dataset.constellations {
        let mut lines: Vec<[usize; 2]> = Vec::new();

        for path in constellation.lines {
            for [x, y] in path.array_windows::<2>() {
                let xbayer = match simbad_to_bayer(x) {
                    Some(v) => v,
                    None => {
                        eprintln!(
                            "WARNING: Constellation loader could not convert star {x}, skipping"
                        );
                        continue;
                    }
                };

                let ybayer = match simbad_to_bayer(y) {
                    Some(v) => v,
                    None => {
                        eprintln!(
                            "WARNING: Constellation loader could not convert star {y}, skipping"
                        );
                        continue;
                    }
                };

                let xstar_i = star_data
                    .iter()
                    .enumerate()
                    .find(|(i, star)| {
                        star.bayer.is_some_and(|star_bayer| {
                            star_bayer.constellation == xbayer.constellation
                                && star_bayer.letter == xbayer.letter
                                && xbayer.superscript.is_none_or(|xs| {
                                    star_bayer.superscript.is_some_and(|ss| ss == xs)
                                })
                        })
                    })
                    .map(|(i, _)| i);

                let ystar_i = star_data
                    .iter()
                    .enumerate()
                    .find(|(_i, star)| {
                        star.bayer.is_some_and(|star_bayer| {
                            star_bayer.constellation == ybayer.constellation
                                && star_bayer.letter == ybayer.letter
                                && ybayer.superscript.is_none_or(|ys| {
                                    star_bayer.superscript.is_some_and(|ss| ss == ys)
                                })
                        })
                    })
                    .map(|(i, _)| i);

                match [xstar_i, ystar_i] {
                    [Some(x), Some(y)] => lines.push([x, y]),
                    _ => eprintln!(
                        "WARNING: Constellation loader could not find stars {xbayer} and/or {ybayer}, skipping"
                    ),
                }
            }
        }

        let constellation = Constellation::new(
            lines,
            star_data,
            constellation
                .names
                .first()
                .expect("no constellation name")
                .english
                .clone(),
        )
        .expect("sfml error");

        constellations.push(constellation);
    }

    constellations
}

fn simbad_to_bayer(simbad: &str) -> Option<Bayer> {
    if !simbad.starts_with('*') {
        return None;
    }

    let mut words = simbad.split_ascii_whitespace().skip(1);

    let mut letter_string = words.next()?;
    let superscript = if letter_string.len() > 3 {
        let superscript = letter_string[3..].parse().ok()?;
        letter_string = &letter_string[0..3];
        Some(superscript)
    } else {
        None
    };
    let constellation = ArrayString::from(words.next()?).ok()?;

    Some(Bayer {
        letter: *GREEK_LETTERS.get(&letter_string)?,
        constellation,
        superscript,
    })
}
