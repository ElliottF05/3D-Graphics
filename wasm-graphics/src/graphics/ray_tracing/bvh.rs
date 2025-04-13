use crate::utils::math::Vec3;

use super::rt::Ray;

/// Axis-Aligned Bounding Box (AABB)
/// A bounding box defined by two points: the minimum and maximum corners.
pub struct AABoundingBox {
    pub min: Vec3,
    pub max: Vec3,
}

impl AABoundingBox {
    pub fn new(min: Vec3, max: Vec3) -> Self {
        AABoundingBox { min, max }
    }
    pub fn from_sub_boxes(box1: &AABoundingBox, box2: &AABoundingBox) -> Self {
        return AABoundingBox::new(
            box1.min.min_elementwise(box2.min),
            box1.max.max_elementwise(box2.max),
        )
    }

    pub fn get_x_bounds(&self) -> (f32, f32) {
        return (self.min.x, self.max.x);
    }
    pub fn get_y_bounds(&self) -> (f32, f32) {
        return (self.min.y, self.max.y);
    }
    pub fn get_z_bounds(&self) -> (f32, f32) {
        return (self.min.z, self.max.z);
    }

    fn hit(&self, ray: &Ray, mut t_min: f32, mut t_max: f32) -> bool {

        // ray.origin + t * ray.direction = pos
        // t = (pos - ray.origin) / ray.direction
        
        let ray_dir_inv = 1.0 / ray.direction.x;
        let mut t0 = (self.min.x - ray.origin.x) * ray_dir_inv;
        let mut t1 = (self.max.x - ray.origin.x) * ray_dir_inv;
        (t0, t1) = (t0.min(t1), t0.max(t1));
        (t_min, t_max) = (t_min.max(t0), t_max.min(t1));

        if t_max <= t_min {
            return false;
        }

        let ray_dir_inv = 1.0 / ray.direction.y;
        let mut t0 = (self.min.y - ray.origin.y) * ray_dir_inv;
        let mut t1 = (self.max.y - ray.origin.y) * ray_dir_inv;
        (t0, t1) = (t0.min(t1), t0.max(t1));
        (t_min, t_max) = (t_min.max(t0), t_max.min(t1));

        if t_max <= t_min {
            return false;
        }

        let ray_dir_inv = 1.0 / ray.direction.z;
        let mut t0 = (self.min.z - ray.origin.z) * ray_dir_inv;
        let mut t1 = (self.max.z - ray.origin.z) * ray_dir_inv;
        (t0, t1) = (t0.min(t1), t0.max(t1));
        (t_min, t_max) = (t_min.max(t0), t_max.min(t1));
        

        return t_max > t_min;

    }
}

impl Default for AABoundingBox {
    /// Empty intervals
    fn default() -> Self {
        Self { 
            min: Vec3::new(1.0, 1.0, 1.0), 
            max: Vec3::zero() 
        }
    }
}