use std::{array, f64::consts::PI};

use sfml::{
    cpp::FBox,
    graphics::{Color, Drawable, PrimitiveType, Vertex, VertexBuffer, VertexBufferUsage},
};

use crate::{
    View,
    settings::{DisplaySettings, GridSetting},
};

pub struct Grid {
    horizon_vb: FBox<VertexBuffer>,
    horizon_vertices: [Vertex; 181],

    lng_lines: [(FBox<VertexBuffer>, [Vertex; 181]); 12],
    lat_lines: [(FBox<VertexBuffer>, [Vertex; 181]); 10],

    setting: GridSetting,
}

impl Grid {
    const MAJOR_COLOR: Color = Color::rgb(96, 96, 96);
    const MINOR_COLOR: Color = Color::rgb(64, 64, 64);

    pub fn new() -> sfml::SfResult<Grid> {
        let vb = VertexBuffer::new(PrimitiveType::LINE_STRIP, 181, VertexBufferUsage::STREAM)?;

        let mut points = [(0., 0.); 181];

        for i in 0..181 {
            points[i] = (0., i as f64 * PI / 90.);
        }

        Ok(Grid {
            horizon_vb: vb,
            horizon_vertices: [Vertex::DEFAULT; 181],

            lng_lines: array::from_fn(|_| {
                (
                    VertexBuffer::new(PrimitiveType::LINE_STRIP, 181, VertexBufferUsage::STREAM)
                        .unwrap(),
                    [Vertex::DEFAULT; 181],
                )
            }),
            lat_lines: array::from_fn(|_| {
                (
                    VertexBuffer::new(PrimitiveType::LINE_STRIP, 181, VertexBufferUsage::STREAM)
                        .unwrap(),
                    [Vertex::DEFAULT; 181],
                )
            }),

            setting: Default::default(),
        })
    }

    pub fn update(&mut self, view: &View, settings: &DisplaySettings) -> sfml::SfResult<()> {
        for (pos, v) in self.horizon_vertices.iter_mut().enumerate() {
            *v = Vertex::with_pos_color(
                view.project_to_screen((0., pos as f64 * PI / 90.)),
                Self::MAJOR_COLOR,
            )
        }

        self.horizon_vb.update(&self.horizon_vertices, 0)?;

        for (i, (vb, vertices)) in self.lng_lines.iter_mut().enumerate() {
            let lng = i as f64 * PI / 12.;

            for (pos, v) in vertices.iter_mut().enumerate() {
                *v = Vertex::with_pos_color(
                    view.project_to_screen((pos as f64 * PI / 90., lng)),
                    if i == 0 || i == 6 {
                        Self::MAJOR_COLOR
                    } else {
                        Self::MINOR_COLOR
                    },
                );
            }

            vb.update(vertices, 0)?;
        }

        for (i, (vb, vertices)) in self.lat_lines.iter_mut().enumerate() {
            let lat = [75, 60, 45, 30, 15, -15, -30, -45, -60, -75]
                .map(|theta| theta as f64 * PI / 180.)[i];

            for (pos, v) in vertices.iter_mut().enumerate() {
                *v = Vertex::with_pos_color(
                    view.project_to_screen((lat, pos as f64 * PI / 90.)),
                    Self::MINOR_COLOR,
                );
            }

            vb.update(vertices, 0)?;
        }

        self.setting = settings.grid();

        Ok(())
    }

    pub fn vb(&self) -> &FBox<VertexBuffer> {
        &self.horizon_vb
    }
}

impl Drawable for Grid {
    fn draw<'a: 'shader, 'texture, 'shader, 'shader_texture>(
        &'a self,
        target: &mut dyn sfml::graphics::RenderTarget,
        rs: &sfml::graphics::RenderStates<'texture, 'shader, 'shader_texture>,
    ) {
        if self.setting == GridSetting::None {
            return;
        }

        target.draw_vertex_buffer(&self.horizon_vb, rs);

        match self.setting {
            GridSetting::None | GridSetting::Horizon => return,
            GridSetting::Major => {
                target.draw_vertex_buffer(&self.lng_lines[0].0, rs);
                target.draw_vertex_buffer(&self.lng_lines[6].0, rs);
            }
            GridSetting::MajorMinor => {
                for (vb, _) in self.lng_lines.iter().chain(self.lat_lines.iter()) {
                    target.draw_vertex_buffer(vb, rs);
                }
            }
        }
    }
}
