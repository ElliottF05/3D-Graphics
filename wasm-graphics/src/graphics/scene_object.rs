use std::sync::atomic::{AtomicUsize, Ordering};

use crate::utils::math::Vec3;

use super::{lighting::Light, mesh::{Mesh, PhongProperties}, ray_tracing::{hittable::{self, Hittable, Sphere}, material::{Dielectric, DiffuseLight, Lambertian, Material, Metal}}};

static NEXT_ID: AtomicUsize = AtomicUsize::new(0);

pub struct SceneObject {
    pub mesh: Mesh,
    pub hittables: Vec<Box<dyn Hittable>>,
    pub lights: Vec<Light>,
    pub is_selected: bool,
    id: usize,
}

impl SceneObject {
    pub fn set_material(mut self, unified_mat: (PhongProperties, Box<dyn Material>)) -> Self {
        let material = unified_mat.1;
        for h in self.hittables.iter_mut() {
            h.set_material(material.clone());
        }
        self.mesh.properties = unified_mat.0;
        return self;
    }
    pub fn new(mesh: Mesh, hittables: Vec<Box<dyn Hittable>>, lights: Vec<Light>) -> SceneObject {
        let id = NEXT_ID.fetch_add(1, Ordering::Relaxed);
        return SceneObject { mesh, hittables, lights, is_selected: false, id };
    }
    pub fn new_from_mesh(mesh: Mesh, material: Box<dyn Material>) -> SceneObject {
        let hittables = mesh.to_rt_hittables(material.as_ref());
        return SceneObject::new(mesh, hittables, Vec::new());
    }

    /// Converts the mesh to a list of hittables, each with a DiffuseLight material.
    /// Creates an omnidirectional light at the center of the mesh, input min_dist as
    /// None for the default (radius of mesh).
    pub fn new_from_mesh_with_omni_light(mesh: Mesh, color: Vec3, min_dist: Option<f32>, buf_width: usize) -> SceneObject {
        let min_d = if let Some(min_d) = min_dist {
            min_d
        } else {
            mesh.radius + 0.01
        };
        let lights = Light::new_omnidirectional(mesh.center, color, min_d, buf_width);
        let hittables = mesh.to_rt_hittables(&DiffuseLight::default());
        return SceneObject::new(mesh, hittables, lights);
    }

    pub fn new_sphere_custom(center: Vec3, radius: f32, color: Vec3, subdivisions: u32, properties: PhongProperties, material: Box<dyn Material>) -> SceneObject {
        let sphere = Sphere::new(center, radius, color, material.clone_box());
        let mesh = Mesh::build_sphere(center, radius, subdivisions, color, properties);
        return SceneObject::new(mesh, vec![Box::new(sphere)], vec![]);
    }
    pub fn new_sphere(center: Vec3, radius: f32, color: Vec3, subdivisions: u32, unified_mat: (PhongProperties, Box<dyn Material>)) -> SceneObject {
        return SceneObject::new_sphere_custom(center, radius, color, subdivisions, unified_mat.0, unified_mat.1);
    }
    pub fn new_sphere_light(center: Vec3, radius: f32, color: Vec3, subdivisions: u32, buf_width: usize) -> SceneObject {
        let sphere = Sphere::new(center, radius, color, DiffuseLight::default().clone_box());
        let mut properties = PhongProperties::default();
        properties.is_light = true;
        let mesh = Mesh::build_sphere(center, radius, subdivisions, color, properties);
        let lights = Light::new_omnidirectional(center, color, radius + 0.01, buf_width);
        return SceneObject::new(mesh, vec![Box::new(sphere)], lights);
    }

    pub fn new_rectangle(origin: Vec3, u: Vec3, v: Vec3, color: Vec3, unified_mat: (PhongProperties, Box<dyn Material>), cull_faces: bool) -> SceneObject {
        let mesh = Mesh::build_rectangle(origin, u, v, color, unified_mat.0, cull_faces);
        return SceneObject::new_from_mesh(mesh, unified_mat.1);
    }
    pub fn new_rectangle_light(origin: Vec3, u: Vec3, v: Vec3, color: Vec3, min_dist: f32, buf_width: usize) -> SceneObject {
        let phong = PhongProperties::new_light();
        let mesh = Mesh::build_rectangle(origin, u, v, color, phong, false);
        return SceneObject::new_from_mesh_with_omni_light(mesh, color, Some(min_dist), buf_width);
    }

    pub fn new_box_from_side_lengths(center: Vec3, x_length: f32, y_length: f32, z_length: f32, color: Vec3, unified_mat: (PhongProperties, Box<dyn Material>)) -> SceneObject {
        let mesh = Mesh::build_box_from_side_lengths(center, x_length, y_length, z_length, color, unified_mat.0);
        return SceneObject::new_from_mesh(mesh, unified_mat.1);
    }
    pub fn new_box_from_corners(corner1: Vec3, corner2: Vec3, color: Vec3, unified_mat: (PhongProperties, Box<dyn Material>)) -> SceneObject {
        let mesh = Mesh::build_box_from_corners(corner1, corner2, color, unified_mat.0);
        return SceneObject::new_from_mesh(mesh, unified_mat.1);
    }
    pub fn new_box_light_from_side_lengths(center: Vec3, x_length: f32, y_length: f32, z_length: f32, color: Vec3, min_dist: f32, buf_width: usize) -> SceneObject {
        let phong = PhongProperties::new_light();
        let mesh = Mesh::build_box_from_side_lengths(center, x_length, y_length, z_length, color, phong);
        return SceneObject::new_from_mesh_with_omni_light(mesh, color, Some(min_dist), buf_width);
    }
    pub fn new_box_light_from_corners(corner1: Vec3, corner2: Vec3, color: Vec3, min_dist: f32, buf_width: usize) -> SceneObject {
        let phong = PhongProperties::new_light();
        let mesh = Mesh::build_box_from_corners(corner1, corner2, color, phong);
        return SceneObject::new_from_mesh_with_omni_light(mesh, color, Some(min_dist), buf_width);
    }

    pub fn new_checkerboard(center: Vec3, radius: i32, color1: Vec3, color2: Vec3, unified_mat: (PhongProperties, Box<dyn Material>), cull_faces: bool) -> SceneObject {
        let mesh = Mesh::build_checkerboard(center, radius, color1, color2, unified_mat.0, cull_faces);
        return SceneObject::new_from_mesh(mesh, unified_mat.1);
    }

    pub fn new_diffuse_mat() -> (PhongProperties, Box<dyn Material>) {
        let phong = PhongProperties::new(
            1.0, 
            1.0, 
            1.0, 
            0.0, 
            0, 
            false,
            true
        );
        let mat = Lambertian::default();
        return (phong, Box::new(mat));
    }
    pub fn new_glossy_mat(fuzz: f32) -> (PhongProperties, Box<dyn Material>) {
        let phong = PhongProperties::new(
            1.0, 
            0.2, 
            fuzz, 
            1.0 - fuzz, 
            ((1.0 - fuzz) * 32.0) as i32, 
            false,
            true
        );
        let mat = Metal::new(fuzz);
        return (phong, Box::new(mat));
    }
    pub fn new_metal_mat(fuzz: f32) -> (PhongProperties, Box<dyn Material>) {
        return SceneObject::new_glossy_mat(fuzz);
    }
    pub fn new_glass_mat(alpha: f32) -> (PhongProperties, Box<dyn Material>) {
        let phong = PhongProperties::new(
            alpha, 
            0.5, 
            0.5, 
            0.75, 
            16, 
            false,
            true
        );
        let mat = Dielectric::new(1.5);
        return (phong, Box::new(mat));
    }
    pub fn new_light_mat() -> (PhongProperties, Box<dyn Material>) {
        let phong = PhongProperties::new_light();
        let mat = DiffuseLight::default();
        return (phong, Box::new(mat));
    }

    pub fn translate_by(&mut self, offset: Vec3) {
        self.mesh.translate_by(offset);
        for h in self.hittables.iter_mut() {
            h.translate_by(offset);
        }
    }
    pub fn translate_to(&mut self, destination: Vec3) {
        let offset = destination - self.mesh.center;
        self.translate_by(offset);
    }
    pub fn set_center(&mut self, new_center: Vec3) {
        self.translate_to(new_center);
    }

    pub fn get_id(&self) -> usize {
        return self.id;
    }
    pub fn is_light(&self) -> bool {
        return !self.lights.is_empty();
    }

    /// Rotates in the z direction first, then y direction
    pub fn rotate_around(&mut self, center_of_rotation: Vec3, theta_z: f32, theta_y: f32) {
        self.mesh.rotate_around(center_of_rotation, theta_z, theta_y);
        for h in self.hittables.iter_mut() {
            h.rotate_around(center_of_rotation, theta_z, theta_y);
        }
    }
    /// Rotates in the z direction first, then y direction
    pub fn rotate_around_center(&mut self, theta_z: f32, theta_y: f32) {
        let center = self.mesh.center;
        self.rotate_around(center, theta_z, theta_y);
    }

    pub fn scale_around(&mut self, center_of_scale: Vec3, scale_factor: f32) {
        self.mesh.scale_around(center_of_scale, scale_factor);
        for h in self.hittables.iter_mut() {
            h.scale_around(center_of_scale, scale_factor);
        }
    }
    pub fn scale_by(&mut self, scale_factor: f32) {
        let center = self.mesh.center;
        self.scale_around(center, scale_factor);
    }
}

impl Clone for SceneObject {
    fn clone(&self) -> Self {
        SceneObject {
            mesh: self.mesh.clone(),
            hittables: self.hittables.iter().map(|h| h.clone_box()).collect(),
            lights: self.lights.clone(),
            is_selected: self.is_selected,
            id: NEXT_ID.fetch_add(1, Ordering::Relaxed),
        }
    }
}