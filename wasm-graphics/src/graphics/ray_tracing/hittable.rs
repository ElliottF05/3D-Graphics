use std::fmt::Debug;

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
        let bounding_box = AABoundingBox::new_from_sorted(center - r_vec, center + r_vec);
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

#[derive(Clone, Debug)]
pub struct Triangle {
    pub origin: Vec3,
    pub u: Vec3,
    pub v: Vec3,
    pub w: Vec3,
    pub normal: Vec3,
    pub d: f32,
    pub color: Vec3,
    pub material: Box<dyn Material>,
    pub bounding_box: AABoundingBox,
}

impl Triangle {
    pub fn new_from_directions(origin: Vec3, u: Vec3, v: Vec3, color: Vec3, material: &dyn Material) -> Triangle {
        let normal_unnormalized = u.cross(v);
        let normal_normalized = normal_unnormalized.normalized();
        let d = normal_normalized.dot(origin);
        let w = normal_unnormalized / normal_unnormalized.len_squared();

        let min = origin.min_elementwise(origin + u).min_elementwise(origin + v);
        let max = origin.max_elementwise(origin + u).max_elementwise(origin + v);
        let mut bounding_box = AABoundingBox::new_from_sorted(min, max);
        bounding_box.pad_to_minimums();

        return Triangle {
            origin,
            u,
            v,
            w,
            normal: normal_normalized,
            d,
            color,
            material: material.clone_box(),
            bounding_box
        }
    }
    pub fn new_from_vertices(v1: Vec3, v2: Vec3, v3: Vec3, color: Vec3, material: &dyn Material) -> Triangle {
        let u = v3 - v1;
        let v = v2 - v1;
        return Triangle::new_from_directions(v1, u, v, color, material);
    }

    #[inline(always)]
    fn intersection_is_interior(alpha: f32, beta: f32) -> bool {
        return alpha > 0.0 && beta > 0.0 && alpha + beta < 1.0;
    }
    
}

pub trait Hittable: Debug {
    fn hit<'a>(&'a self, ray: &Ray, t_min: f32, t_max: f32, hit_record: &mut HitRecord<'a>) -> bool;
    fn get_bounding_box(&self) -> &AABoundingBox;
}

impl Hittable for Sphere {
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

impl Hittable for Triangle {
    #[inline(always)]
    fn hit<'a>(&'a self, ray: &Ray, t_min: f32, t_max: f32, hit_record: &mut HitRecord<'a>) -> bool {
        let norm_dot_dir = self.normal.dot(ray.direction);

        // denominator is 0, occurs when ray direction is parallel to plane
        if norm_dot_dir.abs() < 1e-8 {
            return false;
        }

        let t = (self.d - self.normal.dot(ray.origin)) / norm_dot_dir;
        if t < t_min || t > t_max {
            return false;
        }

        let intersection = ray.at(t);

        let p = intersection - self.origin;
        let alpha = self.w.dot(p.cross(self.v));
        let beta = self.w.dot(self.u.cross(p));

        // now, alpha and beta give us the coordinates in uv space
        // in the plane the triangle lies in, with its origin as the plane's origin

        // testing if intersection is inside triangle
        if !Self::intersection_is_interior(alpha, beta) {
            return false;
        }

        hit_record.t = t;
        hit_record.pos = intersection;
        hit_record.material = Some(self.material.as_ref());
        hit_record.set_face_normal(ray, self.normal);
        hit_record.surface_color = self.color;

        return true;
    }

    fn get_bounding_box(&self) -> &AABoundingBox {
        &self.bounding_box
    }
}