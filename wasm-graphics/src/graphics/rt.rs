use std::fmt::Debug;

use crate::{console_log, utils::{math::Vec3, utils::{random_float, random_range, sample_square}}};

use super::{game::Game, scene::{SceneObject, Sphere}};

#[derive(Debug, Clone)]
pub struct Ray {
    pub origin: Vec3,
    pub direction: Vec3,
}

impl Ray {
    pub fn new(origin: Vec3, direction: Vec3) -> Self {
        Self { origin, direction }
    }
    pub fn at(&self, t: f32) -> Vec3 {
        self.origin + self.direction * t
    }
}

#[derive(Debug, Default)]
pub struct HitRecord {
    pub t: f32,
    pub pos: Vec3,
    pub normal: Vec3,
    pub front_face: bool,
    pub material: Option<Box<dyn Material>>,
}

impl HitRecord {
    pub fn set_face_normal(&mut self, ray: &Ray, outward_normal: Vec3) {
        self.front_face = ray.direction.dot(&outward_normal) < 0.0;
        self.normal = if self.front_face { outward_normal } else { -outward_normal };
    }
}

impl Game {

    pub fn render_ray_tracing(&mut self) {
        self.running = false;
        console_log!("Rendering ray tracing");

        const SAMPLES: usize = 100;
        const MAX_DEPTH: usize = 50;
        for x in 0..self.camera.width {
            for y in 0..self.camera.height {

                let mut pixel_color = Vec3::zero();
                for _ in 0..SAMPLES {
                    let ray = self.get_rand_ray_at_pixel(x, y);
                    let ray_color = self.ray_trace(ray, MAX_DEPTH);
                    pixel_color += ray_color;
                }
                pixel_color /= SAMPLES as f32;
                self.pixel_buf.set_pixel(x, y, pixel_color);
            }
        }

        self.apply_post_processing_effects();
    }

    fn ray_trace(&self, ray: Ray, depth: usize) -> Vec3 {

        if depth == 0 {
            return Vec3::zero(); // no more bounces
        }

        // TODO: change interval max
        let mut hit_record = HitRecord::default();
        let mut hit_anything = false;
        let mut closest_so_far = 100.0;
        
        for obj in self.objects.borrow().iter() {
            if obj.hit(&ray, 0.001, closest_so_far, &mut hit_record) {
                closest_so_far = hit_record.t; // update closest hit
                hit_anything = true;
            }
        }

        if hit_anything { // recursively ray trace
            let reflected_dir = hit_record.normal + Vec3::random_on_hemisphere(&hit_record.normal);
            let reflected_ray = Ray::new(hit_record.pos, reflected_dir);
            return 0.5 * self.ray_trace(reflected_ray, depth-1);
        } else {
            // if no hit, return background color
            return self.get_sky_color(&ray.direction);
        }
    }

    fn get_ray_at_pixel(&self, x: usize, y: usize) -> Ray {
        let origin = self.camera.pos;

        let mut v = Vec3::new(x as f32, y as f32, 1.0);
        self.camera.vertex_screen_to_world_space(&mut v);
        let direction = (v - origin).normalized();

        Ray::new(origin, direction)
    }

    fn get_rand_ray_at_pixel(&self, x: usize, y: usize) -> Ray {
        let origin = self.camera.pos;
        let offset = sample_square();
        let mut v = Vec3::new(x as f32 + offset.x, y as f32 + offset.y, 1.0);
        self.camera.vertex_screen_to_world_space(&mut v);
        let direction = (v - origin).normalized();

        Ray::new(origin, direction)
    }
}


pub trait Material: Debug + Send + Sync {
    /// scatters the inbound ray and returns a tuple of the the attenuation color and the new ray.
    fn scatter(&self, ray: &Ray, hit_record: &HitRecord, surface_color: Vec3) -> (Vec3, Ray);

    /// util for cloning Box<dyn Material> trait objects
    fn clone_box(&self) -> Box<dyn Material>;
}

// implement the clone trait for Box<dyn Material>
impl Clone for Box<dyn Material> {
    fn clone(&self) -> Box<dyn Material> {
        self.clone_box()
    }
}

#[derive(Debug, Clone, Default)]
pub struct Lambertian {
}

impl Lambertian {
}

impl Material for Lambertian {
    fn scatter(&self, ray: &Ray, hit_record: &HitRecord, surface_color: Vec3) -> (Vec3, Ray) {
        let reflected_dir = hit_record.normal + Vec3::random_on_hemisphere(&hit_record.normal);
        let reflected_ray = Ray::new(hit_record.pos, reflected_dir);
        let attenuation = surface_color;
        return (attenuation, reflected_ray)
    }

    fn clone_box(&self) -> Box<dyn Material> {
        Box::new(self.clone())
    }
}
