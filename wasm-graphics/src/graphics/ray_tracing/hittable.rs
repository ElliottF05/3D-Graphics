use std::{f32::consts::PI, fmt::Debug};

use crate::{console_log, graphics::mesh::{Mesh, PhongProperties}, utils::{math::Vec3, utils::random_float}};

use super::{bvh::AABoundingBox, material::{Dielectric, DiffuseLight, Lambertian, Material, Metal}, rt::{HitRecord, Ray}};

pub trait Hittable: Debug + Send + Sync {
    fn hit<'a>(&'a self, ray: &Ray, t_min: f32, t_max: f32, hit_record: &mut HitRecord<'a>) -> bool;
    fn sample_random_point(&self) -> Vec3;
    fn get_area(&self) -> f32;
    fn get_normal(&self, p: Vec3) -> Vec3;
    fn get_color(&self) -> Vec3;
    fn set_color(&mut self, color: Vec3);
    fn get_bounding_box(&self) -> &AABoundingBox;
    fn get_material(&self) -> &dyn Material;
    fn get_mut_material(&mut self) -> &mut dyn Material;
    fn set_material(&mut self, material: Box<dyn Material>);
    /// used ONLY for interaction with JS
    fn set_material_type(&mut self, mat_type: u32) {
        self.set_material(match mat_type {
            0 => Box::new(Lambertian::default()),
            1 => Box::new(Metal::default()),
            2 => Box::new(Dielectric::new(1.5)),
            3 => Box::new(DiffuseLight::default()),
            _ => panic!("Invalid material type"),
        });
    }
    fn translate_by(&mut self, offset: Vec3);
    fn rotate_around(&mut self, center_of_rotation: Vec3, theta_z: f32, theta_y: f32);
    fn scale_around(&mut self, center_of_scale: Vec3, scale_factor: f32);

    fn clone_box(&self) -> Box<dyn Hittable>;
}

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

    fn sample_random_point(&self) -> Vec3 {
        return self.center + self.radius * Vec3::random_on_unit_sphere();
    }
    fn get_area(&self) -> f32 {
        return 4.0 * PI * self.radius * self.radius;
    }
    fn get_normal(&self, p: Vec3) -> Vec3 {
        return (p - self.center).normalized();
    }
    fn get_color(&self) -> Vec3 {
        return self.color;
    }
    fn set_color(&mut self, color: Vec3) {
        self.color = color;
    }
    fn get_bounding_box(&self) -> &AABoundingBox {
        return &self.bounding_box;
    }
    fn get_material(&self) -> &dyn Material {
        return self.material.as_ref();
    }
    fn get_mut_material(&mut self) -> &mut dyn Material {
        return self.material.as_mut();
    }
    fn set_material(&mut self, material: Box<dyn Material>) {
        self.material = material;
    }
    fn translate_by(&mut self, offset: Vec3) {
        self.center += offset;
        let r_vector = self.radius * Vec3::ones();
        self.bounding_box = AABoundingBox::new_from_sorted(self.center - r_vector, self.center + r_vector);
    }
    fn rotate_around(&mut self, center_of_rotation: Vec3, theta_z: f32, theta_y: f32) {
        self.center -= center_of_rotation;
        let (sin_z, cos_z) = theta_z.sin_cos();
        let (sin_y, cos_y) = theta_y.sin_cos();
        self.center.rotate_z_fast(sin_z, cos_z);
        self.center.rotate_y_fast(sin_y, cos_y);
        self.center += center_of_rotation;
        let r_vector = self.radius * Vec3::ones();
        self.bounding_box = AABoundingBox::new_from_sorted(self.center - r_vector, self.center + r_vector);
    }
    fn scale_around(&mut self, center_of_scale: Vec3, scale_factor: f32) {
        self.center -= center_of_scale;
        self.radius *= scale_factor;
        self.center += center_of_scale;
        let r_vector = self.radius * Vec3::ones();
        self.bounding_box = AABoundingBox::new_from_sorted(self.center - r_vector, self.center + r_vector);
    }

    fn clone_box(&self) -> Box<dyn Hittable> {
        return Box::new(self.clone());
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
        let mut triangle = Triangle {
            origin,
            u,
            v,
            w: Vec3::zero(),
            normal: Vec3::zero(),
            d: 0.0,
            color,
            material: material.clone_box(),
            bounding_box: AABoundingBox::empty(),
        };
        triangle.update_geometry();
        return triangle;
    }

    /// Sets various geometry members given that origin,u,v is known.
    /// Finds w, nomral, d, and bounding_box.
    pub fn update_geometry(&mut self) {
        let normal_unnormalized = self.u.cross(self.v);
        let normal_normalized = normal_unnormalized.normalized();
        let d = normal_normalized.dot(self.origin);
        let w = normal_unnormalized / normal_unnormalized.len_squared();

        self.update_bounding_box();

        self.w = w;
        self.normal = normal_normalized;
        self.d = d;
    }
    pub fn update_bounding_box(&mut self) {
        let min = self.origin.min_elementwise(self.origin + self.u).min_elementwise(self.origin + self.v);
        let max = self.origin.max_elementwise(self.origin + self.u).max_elementwise(self.origin + self.v);
        let mut bounding_box = AABoundingBox::new_from_sorted(min, max);
        bounding_box.pad_to_minimums();
        self.bounding_box = bounding_box;
    }
    pub fn new_from_vertices(v1: Vec3, v2: Vec3, v3: Vec3, color: Vec3, material: &dyn Material) -> Triangle {
        let u = v3 - v1;
        let v = v2 - v1;
        return Triangle::new_from_directions(v1, u, v, color, material);
    }
    pub fn new_quad(origin: Vec3, u: Vec3, v: Vec3, color: Vec3, material: &dyn Material) -> (Triangle, Triangle) {
        let t1 = Triangle::new_from_directions(origin, u, v, color, material);
        let t2 = Triangle::new_from_directions(origin+u+v, -u, -v, color, material);
        return (t1, t2);
    }

    #[inline(always)]
    fn intersection_is_interior(alpha: f32, beta: f32) -> bool {
        return alpha > 0.0 && beta > 0.0 && alpha + beta < 1.0;
    }

    pub fn to_mesh(&self, properties: PhongProperties) -> Mesh {
        let vertices = vec![self.origin, self.origin + self.v, self.origin + self.u];
        let indices = vec![0, 1, 2];
        let colors = vec![self.color];
        return Mesh::new(vertices, indices, colors, properties);
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

    fn sample_random_point(&self) -> Vec3 {
        let mut alpha = random_float();
        let mut beta = random_float();
        if alpha + beta > 1.0 {
            alpha = 1.0 - alpha;
            beta = 1.0 - beta;
        }
        return self.origin + alpha * self.u + beta * self.v;
    }
    fn get_area(&self) -> f32 {
        return 0.5 * self.u.cross(self.v).len();
    }
    fn get_normal(&self, p: Vec3) -> Vec3 {
        return self.normal;
    }
    fn get_color(&self) -> Vec3 {
        return self.color;
    }
    fn set_color(&mut self, color: Vec3) {
        self.color = color;
    }
    fn get_bounding_box(&self) -> &AABoundingBox {
        &self.bounding_box
    }
    fn get_material(&self) -> &dyn Material {
        self.material.as_ref()
    }
    fn get_mut_material(&mut self) -> &mut dyn Material {
        self.material.as_mut()
    }
    fn set_material(&mut self, material: Box<dyn Material>) {
        self.material = material;
    }
    fn translate_by(&mut self, offset: Vec3) {
        self.origin += offset;
        self.update_geometry();
        // self.bounding_box.max += offset;
        // self.bounding_box.min += offset;
    }
    fn rotate_around(&mut self, center_of_rotation: Vec3, theta_z: f32, theta_y: f32) {
        let (sin_z, cos_z) = theta_z.sin_cos();
        let (sin_y, cos_y) = theta_y.sin_cos();

        self.origin -= center_of_rotation;
        self.origin.rotate_z_fast(sin_z, cos_z);
        self.origin.rotate_y_fast(sin_y, cos_y);
        self.origin += center_of_rotation;

        self.u.rotate_z_fast(sin_z, cos_z);
        self.u.rotate_y_fast(sin_y, cos_y);

        self.v.rotate_z_fast(sin_z, cos_z);
        self.v.rotate_y_fast(sin_y, cos_y);

        self.update_geometry();
    }
    fn scale_around(&mut self, center_of_scale: Vec3, scale_factor: f32) {
        self.origin -= center_of_scale;
        self.origin *= scale_factor;
        self.origin += center_of_scale;
        
        self.u *= scale_factor;
        self.v *= scale_factor;

        self.update_geometry();
    }

    fn clone_box(&self) -> Box<dyn Hittable> {
        return Box::new(self.clone());
    }
}