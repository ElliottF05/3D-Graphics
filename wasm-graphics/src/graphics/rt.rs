use crate::{console_log, utils::math::Vec3};

use super::game::Game;

#[derive(Debug, Clone)]
struct Ray {
    origin: Vec3,
    direction: Vec3,
}

impl Ray {
    fn new(origin: Vec3, direction: Vec3) -> Self {
        Self { origin, direction }
    }
    fn at(&self, t: f32) -> Vec3 {
        self.origin + self.direction * t
    }
}

impl Game {

    pub fn render_ray_tracing(&mut self) {
        self.running = false;
        console_log!("Rendering ray tracing");

        for x in 0..self.camera.width {
            for y in 0..self.camera.height {
                let ray = self.get_ray_at_pixel(x, y);
                let color = self.ray_trace(ray);
                self.pixel_buf.set_pixel(x, y, color)
            }
        }
    }

    fn ray_trace(&self, ray: Ray) -> Vec3 {
        return self.get_sky_color(&ray.direction);
    }

    fn get_ray_at_pixel(&self, x: usize, y: usize) -> Ray {
        
        let origin = self.camera.pos;

        let mut v = Vec3::new(x as f32, y as f32, 1.0);
        self.camera.vertex_screen_to_world_space(&mut v);
        let direction = (v - origin).normalized();

        Ray::new(origin, direction)
    }
}