use std::fmt::Debug;

use crate::{console_log, utils::{math::Vec3, utils::{random_float, random_range, sample_square}}};

use super::{game::Game, scene::{SceneObject, Sphere}};

#[derive(Debug, Clone, Default)]
pub struct Ray {
    pub origin: Vec3,
    pub direction: Vec3,
    pub index_of_refrac: f32,
}

impl Ray {
    pub fn new(origin: Vec3, direction: Vec3, index_of_refrac: f32) -> Self {
        Self { origin, direction, index_of_refrac }
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
    pub surface_color: Vec3,
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

            let successful_scatter;
            let attenuation;
            let scattered_ray;

            if let Some(material) = &hit_record.material {
                (successful_scatter, attenuation, scattered_ray) = material.scatter(&ray, &hit_record);
                if !successful_scatter {
                    return Vec3::zero(); // no scatter, return black
                }
            } else {
                return Vec3::zero(); // no material, return black
            }
            
            let next_ray_color = self.ray_trace(scattered_ray, depth-1);
            return attenuation.element_mul_with(&next_ray_color);

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

        Ray::new(origin, direction, 1.0)
    }

    fn get_rand_ray_at_pixel(&self, x: usize, y: usize) -> Ray {
        let origin = self.camera.pos;
        let offset = sample_square();
        let mut v = Vec3::new(x as f32 + offset.x, y as f32 + offset.y, 1.0);
        self.camera.vertex_screen_to_world_space(&mut v);
        let direction = (v - origin).normalized();

        Ray::new(origin, direction, 1.0)
    }
}


pub trait Material: Debug + Send + Sync {
    /// scatters the inbound ray and returns a tuple of the the attenuation color and the new ray.
    fn scatter(&self, ray: &Ray, hit_record: &HitRecord) -> (bool, Vec3, Ray);

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
    fn scatter(&self, ray: &Ray, hit_record: &HitRecord) -> (bool, Vec3, Ray) {
        let reflected_dir = hit_record.normal + Vec3::random_on_hemisphere(&hit_record.normal);

        if reflected_dir.near_zero() {
            console_log!("reflected_dir near zero");
        }

        let reflected_ray = Ray::new(hit_record.pos, reflected_dir, ray.index_of_refrac);
        let attenuation = hit_record.surface_color;
        return (true, attenuation, reflected_ray)
    }

    fn clone_box(&self) -> Box<dyn Material> {
        Box::new(self.clone())
    }
}

#[derive(Debug, Clone, Default)]
pub struct Metal {
    fuzz: f32,
}

impl Metal {
    pub fn new(fuzz: f32) -> Self {
        Self { fuzz }
    }
}

impl Material for Metal {
    fn scatter(&self, ray: &Ray, hit_record: &HitRecord) -> (bool, Vec3, Ray) {
        let mut reflected_dir = ray.direction.reflect(&hit_record.normal);
        reflected_dir.normalize();
        reflected_dir += self.fuzz * Vec3::random_on_unit_sphere();

        if reflected_dir.dot(&hit_record.normal) < 0.0 {
            return (false, Vec3::zero(), Ray::default());
        } else  {
            let reflected_ray = Ray::new(hit_record.pos, reflected_dir, ray.index_of_refrac);
            let attenuation = hit_record.surface_color;
            return (true, attenuation, reflected_ray)
        }
    }

    fn clone_box(&self) -> Box<dyn Material> {
        Box::new(self.clone())
    }
}


#[derive(Debug, Clone, Default)]
pub struct Dielectric {
    index_of_refrac: f32,
    // TODO: should i add color here? Doesn't really make sense for dielectric to have multiple colors
}

impl Dielectric {
    pub fn new(index_of_refrac: f32) -> Dielectric {
        return Dielectric { index_of_refrac };
    }
}

impl Material for Dielectric {
    fn scatter(&self, ray: &Ray, hit_record: &HitRecord) -> (bool, Vec3, Ray) {
        let attenuation = hit_record.surface_color;

        // default to index of refraction of air (1.0) if exiting a dielectric
        let n2 = if hit_record.front_face { self.index_of_refrac } else { 1.0 };
        let n1 = ray.index_of_refrac;
        let n1_over_n2 = n1 / n2;
        
        let ray_dir = ray.direction.normalized();

        let mut cos_theta = -ray_dir.dot(&hit_record.normal);
        cos_theta = cos_theta.min(1.0);
        let sin_theta = (1.0 - cos_theta * cos_theta).sqrt();

        // total internal reflection
        let cannot_refract = n1_over_n2 * sin_theta > 1.0;

        let refracted_dir = if cannot_refract {
            ray_dir.reflect(&hit_record.normal)
        } else {
            ray_dir.refract(&hit_record.normal, n1_over_n2)
        };

        let refracted_ray = Ray::new(hit_record.pos, refracted_dir, n2);
        return (true, attenuation, refracted_ray);
    }
    fn clone_box(&self) -> Box<dyn Material> {
        return Box::new(self.clone());
    }
}