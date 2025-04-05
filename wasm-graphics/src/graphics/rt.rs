use std::fmt::Debug;

use crate::{console_log, utils::{math::{degrees_to_radians, Vec3}, utils::{random_float, random_range, sample_circle, sample_square}}};

use super::{game::Game, scene::{SceneObject, Sphere}};

const SAMPLES: usize = 100;
const MAX_DEPTH: usize = 50;

const DEFOCUS_ANGLE: f32 = degrees_to_radians(10.0);
const FOCUS_DIST: f32 = 3.4;

#[derive(Debug, Clone, Default)]
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

        for x in 0..self.camera.width {
            for y in 0..self.camera.height {

                let mut pixel_color = Vec3::zero();
                for _ in 0..SAMPLES {
                    let ray = self.get_rand_ray_at_pixel_with_defocus(x, y);
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

        Ray::new(origin, direction)
    }

    fn get_rand_ray_at_pixel(&self, x: usize, y: usize) -> Ray {
        let origin = self.camera.pos;
        let (offset_x, offset_y) = sample_square(1.0);
        let mut v = Vec3::new(x as f32 + offset_x, y as f32 + offset_y, 1.0);
        self.camera.vertex_screen_to_world_space(&mut v);
        let direction = (v - origin).normalized();

        Ray::new(origin, direction)
    }

    fn get_rand_ray_at_pixel_with_defocus(&self, x: usize, y: usize) -> Ray {
        let defocus_disk_radius = (0.5 * DEFOCUS_ANGLE).tan() * FOCUS_DIST;
        let (disk_x, disk_y) = sample_circle(defocus_disk_radius);
        let (offset_x, offset_y) = sample_square(1.0);


        let mut point_on_focus_plane = Vec3::new(x as f32 + offset_x, y as f32 + offset_y, FOCUS_DIST);
        self.camera.vertex_screen_to_camera_space(&mut point_on_focus_plane);

        let mut point_on_defocus_disk = Vec3::new(0.0, disk_x, disk_y);

        self.camera.vertex_camera_to_world_space(&mut point_on_focus_plane);
        self.camera.vertex_camera_to_world_space(&mut point_on_defocus_disk);

        let ray_dir = (point_on_focus_plane - point_on_defocus_disk).normalized();

        return Ray::new(point_on_defocus_disk, ray_dir);

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

        let reflected_ray = Ray::new(hit_record.pos, reflected_dir);
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
            let reflected_ray = Ray::new(hit_record.pos, reflected_dir);
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

    fn reflectance(&self, cos_theta: f32, n1: f32, n2: f32) -> f32 {
        // use Schlick's approximation: https://en.wikipedia.org/wiki/Schlick%27s_approximation
        let mut r_0 = (n1 - n2) / (n1 + n2);
        r_0 = r_0 * r_0;

        return r_0 + (1.0 - r_0) * (1.0 - cos_theta).powi(5);
    }
}

impl Material for Dielectric {
    fn scatter(&self, ray: &Ray, hit_record: &HitRecord) -> (bool, Vec3, Ray) {

        let attenuation;
        let n1;
        let n2;

        if hit_record.front_face {
            n1 = 1.0;
            n2 = self.index_of_refrac;
            attenuation = hit_record.surface_color;
        } else {
            // default to index of refraction of air (1.0) if exiting a dielectric
            // also default to no attenuation (check this)
            n1 = self.index_of_refrac;
            n2 = 1.0;
            attenuation = Vec3::new(1.0, 1.0, 1.0);
        }

        let n1_over_n2 = n1 / n2;
        
        let ray_dir = ray.direction.normalized();

        let mut cos_theta = -ray_dir.dot(&hit_record.normal);
        cos_theta = cos_theta.min(1.0);
        let sin_theta = (1.0 - cos_theta * cos_theta).sqrt();

        // total internal reflection
        let cannot_refract = n1_over_n2 * sin_theta > 1.0;
        let reflectance = self.reflectance(cos_theta, n1, n2);

        let refracted_dir = if cannot_refract || reflectance > random_float()  {
            ray_dir.reflect(&hit_record.normal)
        } else {
            ray_dir.refract(&hit_record.normal, n1_over_n2)
        };

        let refracted_ray = Ray::new(hit_record.pos, refracted_dir);
        return (true, attenuation, refracted_ray);
    }
    fn clone_box(&self) -> Box<dyn Material> {
        return Box::new(self.clone());
    }
}