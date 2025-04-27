use std::{f32::consts::PI, fmt::Debug};

use crate::{console_log, utils::{math::Vec3, utils::{random_float, random_int, random_range}}};

use super::{bvh::BVHNode, hittable::Hittable, rt::{HitRecord, Ray}};

pub trait Material: Debug + Send + Sync {
    /// scatters the inbound ray and returns a tuple of the the attenuation color and the new ray.
    fn scatter(&self, ray: &Ray, hit_record: &HitRecord) -> (bool, Vec3, Ray);

    /// returns (successful_scatter, attenuation, scattered_ray, Option(light_dist, light_color)).
    /// to use the results, attenuation must be multiplied by L_i, the incoming radiance 
    /// calculated via the next ray. This next ray is either a full ray-tracing ray
    /// if the option is None, or it is a simple shadow ray if the option is Some (direct light sampling).
    fn scatter_mis(&self, ray: &Ray, hit_record: &HitRecord, lights: &Vec<Box<dyn Hittable>>) -> (bool, Vec3, Ray, Option<(f32, Vec3)>) {
        return (false, Vec3::zero(), Ray::new(Vec3::zero(), Vec3::new(1.0, 0.0, 0.0)), None);
    }

    fn emitted(&self, hit_record: &HitRecord) -> Vec3;
    fn clone_box(&self) -> Box<dyn Material>;
}

impl Clone for Box<dyn Material> {
    fn clone(&self) -> Box<dyn Material> {
        self.as_ref().clone_box()
    }
}

#[derive(Debug, Clone, Default)]
pub struct Lambertian {
}

impl Lambertian {
}

impl Material for Lambertian {
    fn scatter(&self, ray: &Ray, hit_record: &HitRecord) -> (bool, Vec3, Ray) {
        let reflected_dir = hit_record.normal + Vec3::random_on_unit_sphere();

        if reflected_dir.near_zero() {
            console_log!("reflected_dir near zero");
        }

        let reflected_ray = Ray::new(hit_record.pos, reflected_dir);
        let attenuation = hit_record.surface_color;
        return (true, attenuation, reflected_ray)
    }

    /// returns (successful_scatter, attenuation, scattered_ray, Option(light_dist, light_color)).
    /// to use the results, attenuation must be multiplied by L_i, the incoming radiance 
    /// calculated via the next ray. This next ray is either a full ray-tracing ray
    /// if the option is None, or it is a simple shadow ray if the option is Some (direct light sampling).
    fn scatter_mis(&self, ray: &Ray, hit_record: &HitRecord, lights: &Vec<Box<dyn Hittable>>) -> (bool, Vec3, Ray, Option<(f32, Vec3)>) {
        let light = &lights[random_int(0, lights.len() as i32 - 1) as usize];
        let light_sample_point = light.sample_random_point();
        let threshold = 1.0;
        let use_cosine = random_float() < threshold;

        let cosine_dir = hit_record.normal + Vec3::random_on_unit_sphere();
        let light_dir = light_sample_point - hit_record.pos;

        let sample_dir = if use_cosine {
            cosine_dir.normalized()
        } else {
            light_dir.normalized()
        };

        let cosine_pdf = hit_record.normal.dot(sample_dir).clamp(0.0, 1.0) / PI; // dot(n,w) / pi

        let r_squared = light_dir.len_squared();
        let area = light.get_area();
        let cos_theta_light = light.get_normal(light_sample_point).dot(-sample_dir).abs().clamp(0.0, 1.0);
        let light_pdf = (1.0 / lights.len() as f32) * (1.0 / area) * r_squared / cos_theta_light;

        let pdf_mix = (threshold * cosine_pdf + (1.0-threshold) * light_pdf).max(1e-6);

        let cos_ray = hit_record.normal.dot(sample_dir).max(0.0);
        let brdf = hit_record.surface_color / PI;

        let attenuation = brdf * cos_ray / pdf_mix;
        
        let scattered_ray = Ray::new(hit_record.pos, sample_dir);

        let light_option = if use_cosine {
            None
        } else {
            Some((light_dir.len(), light.get_color()))
        };

        return (true, attenuation, scattered_ray, light_option);
    }
    fn emitted(&self, hit_record: &HitRecord) -> Vec3 {
        return Vec3::zero();
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
        let mut reflected_dir = ray.direction.reflect(hit_record.normal);
        reflected_dir.normalize();
        reflected_dir += self.fuzz * Vec3::random_on_unit_sphere();

        if reflected_dir.dot(hit_record.normal) < 0.0 {
            return (false, Vec3::zero(), Ray::default());
        } else  {
            let reflected_ray = Ray::new(hit_record.pos, reflected_dir);
            let attenuation = hit_record.surface_color;
            return (true, attenuation, reflected_ray)
        }
    }
    fn emitted(&self, hit_record: &HitRecord) -> Vec3 {
        return Vec3::zero();
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

        let mut cos_theta = -ray_dir.dot(hit_record.normal);
        cos_theta = cos_theta.min(1.0);
        let sin_theta = (1.0 - cos_theta * cos_theta).sqrt();

        // total internal reflection
        let cannot_refract = n1_over_n2 * sin_theta > 1.0;
        let reflectance = self.reflectance(cos_theta, n1, n2);

        let refracted_dir = if cannot_refract || reflectance > random_float()  {
            ray_dir.reflect(hit_record.normal)
        } else {
            ray_dir.refract(hit_record.normal, n1_over_n2)
        };

        let refracted_ray = Ray::new(hit_record.pos, refracted_dir);
        return (true, attenuation, refracted_ray);
    }
    fn emitted(&self, hit_record: &HitRecord) -> Vec3 {
        return Vec3::zero();
    }

    fn clone_box(&self) -> Box<dyn Material> {
        Box::new(self.clone())
    }
}

#[derive(Clone, Debug, Default)]
pub struct DiffuseLight {
}

impl DiffuseLight {
}

impl Material for DiffuseLight {
    fn scatter(&self, ray: &Ray, hit_record: &HitRecord) -> (bool, Vec3, Ray) {
        return (false, Vec3::zero(), ray.clone());
    }
    fn emitted(&self, hit_record: &HitRecord) -> Vec3 {
        return hit_record.surface_color;
    }

    fn clone_box(&self) -> Box<dyn Material> {
        return Box::new(self.clone());
    }
}

