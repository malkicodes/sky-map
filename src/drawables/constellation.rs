use std::{collections::HashSet, fs::File};

use sfml::{
    SfResult,
    cpp::FBox,
    graphics::{Color, Drawable, PrimitiveType, Vertex, VertexBuffer, VertexBufferUsage},
    system::Vector3,
};

use crate::{
    View,
    settings::{ConstellationSetting, DisplaySettings},
    star::Star,
};

#[derive(Debug, Clone)]
pub struct Constellation {
    vb: FBox<VertexBuffer>,
    vertices: Vec<Vertex>,
    /// Index for each star from original star data list
    star_info: Vec<(usize, (f64, f64))>,

    centroid: (f64, f64),
    opacity: u8,

    name: String,
}

impl Constellation {
    pub fn new(lines: Vec<[usize; 2]>, star_data: &[Star], name: String) -> SfResult<Self> {
        let mut star_info: Vec<(usize, (f64, f64))> = Vec::with_capacity(lines.len() * 2);

        let mut coords_sum = Vector3::<f64>::default();
        let mut seen_stars = HashSet::with_capacity(lines.len());

        for star_i in lines.iter().flatten().copied() {
            let (dec, ra) = star_data[star_i].spherical_coordinates();
            star_info.push((star_i, (dec, ra)));

            if !seen_stars.contains(&star_i) {
                seen_stars.insert(star_i);
                coords_sum += Vector3::new(ra.cos() * dec.cos(), ra.sin() * dec.cos(), dec.sin());
            }
        }

        let coords_center = coords_sum / seen_stars.len() as f64;
        let sphere_center = coords_center / coords_center.length_sq().sqrt();

        let centroid_ra = sphere_center.y.atan2(sphere_center.x);
        let centroid_dec = sphere_center.z.asin();

        let centroid = (centroid_dec, centroid_ra);

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

            centroid,
            opacity: 0,

            name,
        })
    }

    const LINE_COLOR: Color = crate::colors::CONSTELLATION_COLOR;
    const OPACITY_CHANGE_PER_FRAME: u8 = 8; // approx 32 frames

    pub fn is_hovered(&self, view: &View) -> bool {
        let centroid_projected = view.project(self.centroid());

        centroid_projected.length_sq() < 0.25
    }

    pub fn should_render(&self) -> bool {
        self.opacity != 0
    }

    pub fn opacity(&self) -> u8 {
        self.opacity
    }

    pub fn update(&mut self, view: &View, settings: &DisplaySettings) -> SfResult<()> {
        if settings.constellations() != ConstellationSetting::None
            && (self.is_hovered(view) || settings.constellations() == ConstellationSetting::All)
        {
            self.opacity = self.opacity.saturating_add(Self::OPACITY_CHANGE_PER_FRAME);
        } else {
            self.opacity = self.opacity.saturating_sub(Self::OPACITY_CHANGE_PER_FRAME);
        }

        for (v, (_i, coords)) in self.vertices.iter_mut().zip(self.star_info.iter().copied()) {
            *v = Vertex::with_pos_color(
                view.project_to_screen(coords),
                Color {
                    a: self.opacity,
                    ..Self::LINE_COLOR
                },
            );
        }
        self.vb.update(&self.vertices, 0)
    }

    pub fn name(&self) -> &str {
        self.name.as_str()
    }

    pub fn centroid(&self) -> (f64, f64) {
        self.centroid
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
        pub(super) constellations: Vec<ConstellationData>,
    }

    #[derive(Debug, Clone, Deserialize)]
    pub(super) struct ConstellationData {
        pub(super) id: String,
        pub(super) name: ConstellationName,
        /// List of line strips to draw between HIP stars
        pub(super) lines: Vec<Vec<Option<u32>>>,
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
            for line in path.array_windows::<2>() {
                if let [Some(target_a), Some(target_b)] = line {
                    if let [Some(a), Some(b)] = [target_a, target_b].map(|target_hip| {
                        star_data
                            .iter()
                            .enumerate()
                            .find(|(_, star)| star.hip.eq(target_hip))
                            .map(|(i, _)| i)
                    }) {
                        lines.push([a, b]);
                    } else {
                        eprintln!(
                            "WARNING: Could not find stars {:>6} - {:<6} for {}",
                            target_a, target_b, constellation.name.native
                        );
                    }
                } else {
                    eprintln!(
                        "WARNING: Line {line:?} for constellation {} contains null",
                        constellation.id
                    )
                }
            }
        }

        let constellation =
            Constellation::new(lines, star_data, constellation.name.native).expect("sfml error");

        constellations.push(constellation);
    }

    constellations
}
