use std::f64::consts::PI;

use sfml::{
    cpp::FBox,
    graphics::{Color, Drawable, PrimitiveType, Vertex, VertexBuffer, VertexBufferUsage},
};

use crate::{View, sterejec, sterejec_to_screen};

pub struct Grid {
    horizon_vb: FBox<VertexBuffer>,
    points: Vec<(f64, f64)>,
    vertices: Vec<Vertex>,
}

impl Grid {
    pub fn new(vertex_count: usize) -> sfml::SfResult<Grid> {
        let vb = VertexBuffer::new(
            PrimitiveType::LINE_STRIP,
            vertex_count + 1,
            VertexBufferUsage::STREAM,
        )?;

        let mut points = Vec::with_capacity(vertex_count + 1);

        for i in 0..vertex_count + 1 {
            points.push((0., i as f64 / vertex_count as f64 * 2. * PI));
        }

        Ok(Grid {
            horizon_vb: vb,
            points,
            vertices: vec![Vertex::default(); vertex_count + 1],
        })
    }

    pub fn update(&mut self, view: &View) -> sfml::SfResult<()> {
        for (v, pos) in self.vertices.iter_mut().zip(self.points.iter().copied()) {
            *v = Vertex::with_pos_color(sterejec_to_screen(sterejec(pos, view), view), Color::WHITE)
        }

        self.horizon_vb.update(&self.vertices, 0)?;

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
        target.draw_vertex_buffer(&self.horizon_vb, rs);
    }
}
