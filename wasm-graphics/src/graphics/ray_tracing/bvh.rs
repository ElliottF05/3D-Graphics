use crate::{console_log, graphics::mesh::Mesh, utils::{math::Vec3, utils::random_int}};

use super::{hittable::Hittable, material::Material, rt::{HitRecord, Ray}};

/// Axis-Aligned Bounding Box (AABB)
/// A bounding box defined by two points: the minimum and maximum corners.
#[derive(Clone, Debug)]
pub struct AABoundingBox {
    pub min: Vec3,
    pub max: Vec3,
}

impl AABoundingBox {
    /// Assumes input vectors are already min and max
    pub fn new_from_sorted(min: Vec3, max: Vec3) -> Self {
        AABoundingBox { min, max }
    }
    pub fn new_from_unsorted(v1: Vec3, v2: Vec3) -> Self {
        let (min, max) = (v1.min_elementwise(v2), v1.max_elementwise(v2));
        return Self::new_from_sorted(min, max);
    }
    pub fn from_sub_boxes(box1: &AABoundingBox, box2: &AABoundingBox) -> Self {
        return AABoundingBox::new_from_sorted(
            box1.min.min_elementwise(box2.min),
            box1.max.max_elementwise(box2.max),
        )
    }
    pub fn universe() -> Self {
        AABoundingBox { 
            min: Vec3::new(-99999.0, -99999.0, -99999.9), 
            max: Vec3::new(99999.0, 99999.0, 99999.0),
        }
    }
    pub fn empty() -> Self {
        AABoundingBox { 
            min: Vec3::new(99999.0, 99999.0, 99999.0),
            max: Vec3::new(-99999.0, -99999.0, -99999.9), 
        }
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
    pub fn get_x_size(&self) -> f32 {
        return self.max.x - self.min.x;
    }
    pub fn get_y_size(&self) -> f32 {
        return self.max.y - self.min.y;
    }
    pub fn get_z_size(&self) -> f32 {
        return self.max.z - self.min.z;
    }
    pub fn get_center(&self) -> Vec3 {
        return 0.5 * (self.min + self.max);
    }

    pub fn get_longest_axis(&self) -> i32 {
        if self.max.x - self.min.x > self.max.y - self.min.y {
            if self.max.x - self.min.x > self.max.z - self.min.z {
                return 0;
            } else {
                return 2;
            }
        } else {
            if self.max.y - self.min.y > self.max.z - self.min.z {
                return 1;
            } else {
                return 2;
            }
        }
    }

    pub fn pad_to_minimums(&mut self) {
        let delta = 0.0001;
        if self.get_x_size() < delta {
            self.min.x -= 0.5 * delta;
            self.max.x += 0.5 * delta;
        }
        if self.get_y_size() < delta {
            self.min.y -= 0.5 * delta;
            self.max.y += 0.5 * delta;
        }
        if self.get_z_size() < delta {
            self.min.z -= 0.5 * delta;
            self.max.z += 0.5 * delta;
        }
    }

    #[inline(always)]
    fn hit(&self, ray: &Ray, mut t_min: f32, mut t_max: f32) -> bool {

        // ray.origin + t * ray.direction = pos
        // t = (pos - ray.origin) / ray.direction
        
        let ray_dir_inv = 1.0 / ray.direction.x;
        let mut t0 = (self.min.x - ray.origin.x) * ray_dir_inv;
        let mut t1 = (self.max.x - ray.origin.x) * ray_dir_inv;
        (t0, t1) = (t0.min(t1), t0.max(t1));
        (t_min, t_max) = (t_min.max(t0), t_max.min(t1));

        if t_min > t_max {
            return false;
        }

        let ray_dir_inv = 1.0 / ray.direction.y;
        let mut t0 = (self.min.y - ray.origin.y) * ray_dir_inv;
        let mut t1 = (self.max.y - ray.origin.y) * ray_dir_inv;
        (t0, t1) = (t0.min(t1), t0.max(t1));
        (t_min, t_max) = (t_min.max(t0), t_max.min(t1));

        if t_min > t_max {
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


#[derive(Debug)]
pub enum BVHNode {
    Leaf {
        bounding_box: AABoundingBox,
        object: Box<dyn Hittable>,
    },
    Internal {
        bounding_box: AABoundingBox,
        left: Box<BVHNode>,
        right: Box<BVHNode>,
    }
}

impl BVHNode {
    pub fn new(mut objects: Vec<Box<dyn Hittable>>) -> Self {

        // leaf node
        if objects.len() == 1 {
            return Self::Leaf { 
                bounding_box: objects[0].get_bounding_box().clone(), 
                object: objects.pop().unwrap() 
            };
        } 
    
        // internal node
        let mut bounding_box = AABoundingBox::empty();
        for obj in objects.iter() {
            bounding_box = AABoundingBox::from_sub_boxes(&bounding_box, obj.get_bounding_box());
        }

        let axis = bounding_box.get_longest_axis();
        objects.sort_by(|obj1, obj2| {
            let c1 = obj1.get_bounding_box().get_center();
            let c2 = obj2.get_bounding_box().get_center();
            match axis {
                0 => (c1.x).partial_cmp(&c2.x).unwrap(),
                1 => (c1.y).partial_cmp(&c2.y).unwrap(),
                2 => (c1.z).partial_cmp(&c2.z).unwrap(),
                _ => unreachable!("invalid axis to sort by"),
            }
        });
        
        let mid = objects.len() / 2;
        let right_vec = objects.split_off(mid);
        let left_vec = objects;

        let left = Box::new(BVHNode::new(left_vec));
        let right = Box::new(BVHNode::new(right_vec));

        let bounding_box = AABoundingBox::from_sub_boxes(
            &left.get_bounding_box(), 
            &right.get_bounding_box()
        );

        return BVHNode::Internal { 
            bounding_box, 
            left,
            right,
        };

    }
}

impl Hittable for BVHNode {
    #[inline(always)]
    fn hit<'a>(&'a self, ray: &Ray, t_min: f32, t_max: f32, hit_record: &mut HitRecord<'a>) -> bool {
        match self {
            BVHNode::Leaf { object, .. } => {
                return object.hit(ray, t_min, t_max, hit_record)
            },
            BVHNode::Internal { bounding_box, left, right } => {
                if !bounding_box.hit(ray, t_min, t_max) {
                    return false;
                }
                let hit_left = left.hit(ray, t_min, t_max, hit_record);
                let new_tmax = if hit_left {hit_record.t} else {t_max};
                let hit_right = right.hit(ray, t_min, new_tmax, hit_record);
                return hit_left || hit_right;
            }
        }
    }
    fn sample_random_point(&self) -> Vec3 {
        console_log!("BVHNode::sample_random_point() should NEVER be called, this is a mistake");
        return Vec3::zero();
    }
    fn get_area(&self) -> f32 {
        console_log!("BVHNode::get_area() should NEVER be called, this is a mistake");
        return 1.0;
    }
    fn get_normal(&self, p: Vec3) -> Vec3 {
        console_log!("BVHNode::get_normal() should NEVER be called, this is a mistake");
        return Vec3::new(1.0, 0.0, 0.0);
    }
    fn get_color(&self) -> Vec3 {
        console_log!("BVHNode::get_color() should NEVER be called, this is a mistake");
        return Vec3::zero();
    }
    fn set_color(&mut self, color: Vec3) {
        console_log!("BVHNode::set_color() should NEVER be called, this is a mistake");
    }
    fn get_bounding_box(&self) -> &AABoundingBox {
        match self {
            BVHNode::Leaf { bounding_box, .. } => &bounding_box,
            BVHNode::Internal { bounding_box, .. } => &bounding_box
        }
    }
    fn get_material(&self) -> &dyn Material {
        console_log!("BVHNode::get_material() should NEVER be called, this is a mistake");
        unreachable!();
    }
    fn set_material(&mut self, material: Box<dyn Material>) {
        console_log!("BVHNode::set_material() should NEVER be called, this is a mistake");
        unreachable!();
    }
    fn translate_by(&mut self, offset: Vec3) {
        console_log!("BVHNode::translate_by() should NEVER be called, this is a mistake");
    }
    fn rotate_around(&mut self, center_of_rotation: Vec3, theta_z: f32, theta_y: f32) {
        console_log!("BVHNode::rotate_around() should NEVER be called, this is a mistake");
    }
    fn scale_around(&mut self, center_of_scale: Vec3, scale_factor: f32) {
        console_log!("BVHNode::scale_around() should NEVER be called, this is a mistake");
    }
    fn clone_box(&self) -> Box<dyn Hittable> {
        console_log!("BVHNode::clone_box() should NEVER be called, this is a mistake");
        unreachable!();
    }
}