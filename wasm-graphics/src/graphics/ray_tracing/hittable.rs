use crate::{graphics::scene::{SceneObject, Sphere, VertexObject}, utils::math::Vec3};

use super::{bvh::AABoundingBox, rt::{HitRecord, Ray}};



pub trait Hittable {
    fn hit<'a>(&'a self, ray: &Ray, t_min: f32, t_max: f32, hit_record: &mut HitRecord<'a>) -> bool;
    fn get_bounding_box(&self) -> &AABoundingBox;
}

impl Hittable for VertexObject {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32, hit_record: &mut HitRecord) -> bool {
        return false;
    }
    fn get_bounding_box(&self) -> &AABoundingBox {
        return &self.bounding_box;
    }
}

impl Hittable for Sphere {
    // RAY TRACING FUNCTIONS

    #[inline(always)]
    fn hit<'a>(&'a self, ray: &Ray, t_min: f32, t_max: f32, hit_record: &mut HitRecord<'a>) -> bool {
        let oc = self.mesh.center - ray.origin;
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
            let outward_normal = (hit_record.pos - self.mesh.center).normalized();
            hit_record.set_face_normal(ray, outward_normal);
            hit_record.material = Some(self.mesh.material.as_ref());
            hit_record.surface_color = self.mesh.colors[0]; // assuming sphere is one color

            return true;
        }

    }

    fn get_bounding_box(&self) -> &AABoundingBox {
        return &self.bounding_box;
    }
}