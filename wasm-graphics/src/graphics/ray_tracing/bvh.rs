use std::usize::MAX;

use web_sys::console;

use crate::{console_log, graphics::mesh::Mesh, utils::{math::Vec3, utils::random_int}};

use super::{hittable::{self, Hittable}, material::Material, rt::{HitRecord, Ray}};

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
        

        return t_min <= t_max;

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

    #[inline(always)]
    pub fn hit<'a>(&'a self, ray: &Ray, t_min: f32, t_max: f32, hit_record: &mut HitRecord<'a>) -> bool {
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
    fn get_bounding_box(&self) -> &AABoundingBox {
        match self {
            BVHNode::Leaf { bounding_box, .. } => &bounding_box,
            BVHNode::Internal { bounding_box, .. } => &bounding_box
        }
    }
}


#[derive(Debug)]
pub struct FlattenedBVH {
    nodes: Vec<FlattenedBVHNode>,
    hittables: Vec<Box<dyn Hittable>>,
}

impl FlattenedBVH {
    pub fn new(mut hittables: Vec<Box<dyn Hittable>>) -> Self {
        let mut flattened_bvh = FlattenedBVH {
            nodes: Vec::with_capacity(2 * hittables.len()),
            hittables: Vec::with_capacity(hittables.len()),
        };

        flattened_bvh.build(&mut hittables, 0);
        flattened_bvh.hittables = hittables;
        console_log!("flattened bvh built with {} nodes, {} hittables", flattened_bvh.nodes.len(), flattened_bvh.hittables.len());
        return flattened_bvh;
    }

    fn build(&mut self, hittables: &mut [Box<dyn Hittable>], base_index: usize) -> usize {

        // empty node
        if hittables.is_empty() {
            return MAX;
        }

        // leaf node
        if hittables.len() == 1 {
            let mut bounding_box = hittables[0].get_bounding_box().clone();
            bounding_box.pad_to_minimums();
            // self.hittables.push(hittables[0].clone_box());
            // let idx = self.hittables.len() - 1;
            let node = FlattenedBVHNode::Leaf { 
                bounding_box, 
                hittable_index: base_index
            };
            self.nodes.push(node);
            return self.nodes.len() - 1;
        }

        // internal node
        let mut bounding_box = AABoundingBox::empty();
        for hittable in hittables.iter() {
            bounding_box = AABoundingBox::from_sub_boxes(&bounding_box, hittable.get_bounding_box());
        }
        bounding_box.pad_to_minimums();

        let axis = bounding_box.get_longest_axis();
        hittables.sort_by(|obj1, obj2| {
            let c1 = obj1.get_bounding_box().get_center();
            let c2 = obj2.get_bounding_box().get_center();
            match axis {
                0 => (c1.x).partial_cmp(&c2.x).unwrap(),
                1 => (c1.y).partial_cmp(&c2.y).unwrap(),
                2 => (c1.z).partial_cmp(&c2.z).unwrap(),
                _ => unreachable!("invalid axis to sort by"),
            }
        });
        
        let mid = hittables.len() / 2;

        let (left_vec, right_vec) = hittables.split_at_mut(mid);

        let left_child_index = self.build(left_vec, base_index);
        let right_child_index = self.build(right_vec, base_index + mid);

        let node = FlattenedBVHNode::Internal { 
            bounding_box, 
            left_index: left_child_index, 
            right_index: right_child_index 
        };
        self.nodes.push(node);
        return self.nodes.len() - 1;
    }

    // #[inline(always)]
    pub fn hit<'a>(&'a self, ray: &Ray, t_min: f32, mut t_max: f32, hit_record: &mut HitRecord<'a>) -> bool {
        let mut stack = Vec::with_capacity(64);
        stack.push(self.nodes.len() - 1); // start with the root node
        let mut hit_anything = false;

        while let Some(node_index) = stack.pop() {
            if node_index >= self.nodes.len() {
                continue;
            }
            let node = &self.nodes[node_index];
            match node {
                FlattenedBVHNode::Leaf { bounding_box, hittable_index } => {
                    if self.hittables[*hittable_index].hit(ray, t_min, t_max, hit_record) {
                        t_max = hit_record.t; // update closest hit so far
                        hit_anything = true;
                    }
                },
                FlattenedBVHNode::Internal { bounding_box, left_index, right_index } => {
                    if bounding_box.hit(ray, t_min, t_max) {
                        stack.push(*left_index);
                        stack.push(*right_index);
                    }
                }
            }
        }
        return hit_anything;
    }
}

#[derive(Debug)]
pub enum FlattenedBVHNode {
    Leaf {
        bounding_box: AABoundingBox,
        hittable_index: usize,
    },
    Internal {
        bounding_box: AABoundingBox,
        left_index: usize,
        right_index: usize,
    }
}