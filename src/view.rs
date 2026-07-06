use std::f64::consts::FRAC_PI_2;

use sfml::system::Vector2f;

use crate::HALF_SCREEN_SIZE;

#[derive(Clone, Copy, Debug)]
pub struct View {
    o_lat: f64,
    o_lng: f64,

    zoom: f32,
    zoom_v: f32,
}

impl Default for View {
    fn default() -> Self {
        Self {
            o_lat: 0.,
            o_lng: 0.,
            zoom: 1.,
            zoom_v: 1.,
        }
    }
}

impl View {
    pub fn latlng(&self) -> (f64, f64) {
        (self.o_lat, self.o_lng)
    }

    pub fn change_latlng(&mut self, delta: (f64, f64)) {
        self.o_lng += delta.1;

        self.o_lat = (self.o_lat + delta.0).clamp(-FRAC_PI_2, FRAC_PI_2);
    }

    pub fn zoom(&self) -> f32 {
        self.zoom
    }

    pub fn zoom_v(&self) -> f32 {
        self.zoom_v
    }

    pub fn set_zoom(&mut self, v: f32) {
        self.zoom = v;
        self.zoom_v = 2_f32.powf(v) * 0.5;
    }

    pub fn project(&self, pos: (f64, f64)) -> (f64, f64) {
        let lat = -pos.0;
        let lng = -pos.1;

        let (o_lat, o_lng) = self.latlng();

        let k = 2. / (1. + o_lat.sin() * lat.sin() + o_lat.cos() * lat.cos() * (lng - o_lng).cos());

        (
            (k * lat.cos() * (lng - o_lng).sin()),
            (k * (o_lat.cos() * lat.sin() - o_lat.sin() * lat.cos() * (lng - o_lng).cos())),
        )
    }

    pub fn project_to_screen(&self, pos: (f64, f64)) -> Vector2f {
        let (proj_x, proj_y) = self.project(pos);

        Vector2f::new(proj_x as f32, proj_y as f32) * HALF_SCREEN_SIZE as f32 * self.zoom_v()
            + Vector2f::new(HALF_SCREEN_SIZE as f32, HALF_SCREEN_SIZE as f32)
    }
}
