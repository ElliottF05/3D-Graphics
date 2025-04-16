use std::hint;

use crate::{graphics::scene::SceneObject, utils::{math::Vec3, utils::random_int}};

use super::{hittable::Hittable, rt::{HitRecord, Ray}};

/// Axis-Aligned Bounding Box (AABB)
/// A bounding box defined by two points: the minimum and maximum corners.
#[derive(Clone, Debug)]
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


pub enum BVHNode {
    Leaf {
        bounding_box: AABoundingBox,
        object: Box<dyn SceneObject>,
    },
    Internal {
        bounding_box: AABoundingBox,
        left: Box<BVHNode>,
        right: Box<BVHNode>,
    }
}

impl BVHNode {
    pub fn new(mut objects: Vec<Box<dyn SceneObject>>) -> Self {

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
            match axis {
                0 => (obj1.get_center().x).partial_cmp(&obj2.get_center().x).unwrap(),
                1 => (obj1.get_center().y).partial_cmp(&obj2.get_center().y).unwrap(),
                2 => (obj1.get_center().z).partial_cmp(&obj2.get_center().z).unwrap(),
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
    fn hit<'a>(&'a self, ray: &Ray, t_min: f32, t_max: f32, hit_record: &mut HitRecord<'a>) -> bool {
        match self {
            BVHNode::Leaf { object, .. } => object.hit(ray, t_min, t_max, hit_record),
            BVHNode::Internal { bounding_box, left, right } => {
                if !bounding_box.hit(ray, t_min, t_max) {
                    return false;
                }
                let hit_left = left.hit(ray, t_min, t_max, hit_record);
                let new_tmax = if hit_left {hit_record.t} else {t_max};
                let hit_right = right.hit(ray, t_min, new_tmax, hit_record);
                hit_left || hit_right
            }
        }
    }
    fn get_bounding_box(&self) -> &AABoundingBox {
        match self {
            BVHNode::Leaf { bounding_box, .. } => &bounding_box,
            BVHNode::Internal { bounding_box, .. } => &bounding_box
        }
    }
}