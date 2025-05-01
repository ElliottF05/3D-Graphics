use std::sync::atomic::AtomicUsize;

use crate::utils::math::Vec3;

use super::{lighting::Light, mesh::{Mesh, PhongProperties}, ray_tracing::{hittable::{self, Hittable, Sphere}, material::{Dielectric, DiffuseLight, Lambertian, Material, Metal}}};

static NEXT_ID: AtomicUsize = AtomicUsize::new(0);

pub struct SceneObject {
    pub mesh: Mesh,
    pub hittables: Vec<Box<dyn Hittable>>,
    pub lights: Vec<Light>,
    id: usize,
}

impl SceneObject {
    pub fn new(mesh: Mesh, hittables: Vec<Box<dyn Hittable>>, lights: Vec<Light>) -> SceneObject {
        let id = NEXT_ID.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
        return SceneObject { mesh, hittables, lights, id };
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
    pub fn new_sphere(center: Vec3, radius: f32, color: Vec3, subdivisions: u32, unified_properties: (PhongProperties, Box<dyn Material>)) -> SceneObject {
        return SceneObject::new_sphere_custom(center, radius, color, subdivisions, unified_properties.0, unified_properties.1);
    }
    pub fn new_sphere_light(center: Vec3, radius: f32, color: Vec3, subdivisions: u32, buf_width: usize) -> SceneObject {
        let sphere = Sphere::new(center, radius, color, DiffuseLight::default().clone_box());
        let mut properties = PhongProperties::default();
        properties.is_light = true;
        let mesh = Mesh::build_sphere(center, radius, subdivisions, color, properties);
        let lights = Light::new_omnidirectional(center, color, radius + 0.01, buf_width);
        return SceneObject::new(mesh, vec![Box::new(sphere)], lights);
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
    pub fn new_glossy_mat(metalness: f32) -> (PhongProperties, Box<dyn Material>) {
        let phong = PhongProperties::new(
            1.0, 
            0.2, 
            1.0 - metalness, 
            metalness, 
            metalness as i32 * 32, 
            false,
            true
        );
        let mat = Metal::new(1.0 - metalness);
        return (phong, Box::new(mat));
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
}