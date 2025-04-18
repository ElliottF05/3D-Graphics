use crate::{console_log, graphics::mesh::{Mesh, PhongProperties}, utils::math::Vec3};

use super::{bvh::AABoundingBox, material::Material, rt::{HitRecord, Ray}};

#[derive(Clone, Debug)]
pub struct Sphere {
    pub center: Vec3,
    pub radius: f32,
    pub color: Vec3,
    pub material: Box<dyn Material>,
    pub bounding_box: AABoundingBox,
}

impl Sphere {
    pub fn new(center: Vec3, radius: f32, color: Vec3, material: Box<dyn Material>) -> Sphere {
        let r_vec = Vec3::new(radius, radius, radius);
        let bounding_box = AABoundingBox::new(center - r_vec, center + r_vec);
        return Sphere {
            center, 
            radius, 
            color,
            material,
            bounding_box,
        }
    }

    pub fn to_mesh(&self, subdivisions: u32, properties: PhongProperties) -> Mesh {
        return Mesh::build_sphere(self.center, self.radius, subdivisions, self.color, properties)
    }
}

struct Triangle {
    pub origin: Vec3,
    pub u: Vec3,
    pub v: Vec3,
    pub normal: Vec3,
    pub color: Vec3,
    pub material: Box<dyn Material>,
    pub bounding_box: AABoundingBox,
}

pub trait Hittable {
    fn hit<'a>(&'a self, ray: &Ray, t_min: f32, t_max: f32, hit_record: &mut HitRecord<'a>) -> bool;
    fn get_bounding_box(&self) -> &AABoundingBox;
}

impl Hittable for Sphere {
    // RAY TRACING FUNCTIONS
    #[inline(always)]
    fn hit<'a>(&'a self, ray: &Ray, t_min: f32, t_max: f32, hit_record: &mut HitRecord<'a>) -> bool {
        let oc = self.center - ray.origin;
        let a = ray.direction.len_squared();
        let h = oc.dot(ray.direction);
        let c = oc.len_squared() - self.radius * self.radius;

        let discriminant = h * h - a * c;

        if discriminant < 0.0 {
            return false;
        } else {
            let sqrtd = discriminant.sqrt();
            let mut t = (h - sqrtd) / a; // want smaller (closer) value of t first

            if t < t_min || t > t_max { // Check if t is in range [t_min, t_max]
                t = (h + sqrtd) / a; // use larger value of t

                if t < t_min || t > t_max {
                    return false; // none of the t values are in range
                }
            }

            hit_record.t = t;
            hit_record.pos = ray.at(t);
            // normal points from center of sphere to intersection point on surface
            let outward_normal = (hit_record.pos - self.center).normalized();
            hit_record.set_face_normal(ray, outward_normal);
            hit_record.material = Some(self.material.as_ref());
            hit_record.surface_color = self.color; // assuming sphere is one color

            return true;
        }

    }

    fn get_bounding_box(&self) -> &AABoundingBox {
        return &self.bounding_box;
    }
}