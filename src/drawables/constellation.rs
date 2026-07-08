use std::fs::File;

use sfml::{
    SfResult,
    cpp::FBox,
    graphics::{Color, Drawable, PrimitiveType, Vertex, VertexBuffer, VertexBufferUsage},
};

use crate::{View, settings::DisplaySettings, star::Star};

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

    const LINE_COLOR: Color = Color {
        a: 128,
        ..Color::BLUE
    };

    pub fn update(&mut self, view: &View, _settings: &DisplaySettings) -> SfResult<()> {
        for (v, (_i, coords)) in self.vertices.iter_mut().zip(self.star_info.iter().copied()) {
            *v = Vertex::with_pos_color(view.project_to_screen(coords), Self::LINE_COLOR);
        }
        self.vb.update(&self.vertices, 0)
    }

    pub fn name(&self) -> &str {
        self.name.as_str()
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

#[allow(dead_code, reason = "info panel for constellation will need this data")]
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
            for [target_a, target_b] in path.array_windows::<2>() {
                if let [Some(a), Some(b)] = [target_a, target_b].map(|target_simbad| {
                    star_data
                        .iter()
                        .enumerate()
                        .find(|(_, star)| star.simbad_id().eq(target_simbad))
                        .map(|(i, _)| i)
                }) {
                    lines.push([a, b]);
                } else {
                    eprintln!("WARNING: Could not find stars {} - {}", target_a, target_b);
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
