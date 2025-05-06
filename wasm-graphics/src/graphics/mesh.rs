use gltf::json::extensions::material;

use crate::{console_log, utils::math::Vec3, graphics::ray_tracing::rt::Ray};
use std::{collections::HashMap, fmt::Debug, io::Cursor, sync::atomic::{AtomicUsize, Ordering}, vec};

use super::ray_tracing::{bvh::AABoundingBox, hittable::{Hittable, Triangle}, material::Material, rt::HitRecord};

#[derive(Debug, Clone, Copy)]
pub struct PhongProperties {
    pub alpha: f32,
    pub ambient: f32,
    pub diffuse: f32,
    pub specular: f32,
    pub shininess: i32,
    pub is_light: bool,
    pub cull_faces: bool,
}

impl PhongProperties {
    pub fn new(alpha: f32, ambient: f32, diffuse: f32, specular: f32, shininess: i32, is_light: bool, cull_faces: bool) -> PhongProperties {
        PhongProperties {
            alpha,
            ambient,
            diffuse,
            specular,
            shininess,
            is_light,
            cull_faces
        }
    }
    pub fn new_light() -> PhongProperties {
        let mut phong_properties = PhongProperties::default();
        phong_properties.is_light = true;
        return phong_properties;
    }
}

impl Default for PhongProperties {
    fn default() -> Self {
        PhongProperties::new(1.0, 1.0, 0.8, 1.0, 32, false, true)
    }
}

#[derive(Debug, Clone)]
pub struct Mesh {
    pub vertices: Vec<Vec3>,
    pub indices: Vec<usize>,
    pub colors: Vec<Vec3>,
    pub normals: Vec<Vec3>,

    pub properties: PhongProperties,

    pub center: Vec3,
    pub radius: f32,
}


impl Mesh {

    // CONSTRUCTORS
    pub fn new(vertices: Vec<Vec3>, indices: Vec<usize>, colors: Vec<Vec3>, properties: PhongProperties) -> Mesh {

        let mut normals = Vec::with_capacity(indices.len() / 3);
        for i in (0..indices.len()).step_by(3) {
            let v1 = vertices[indices[i]];
            let v2 = vertices[indices[i+1]];
            let v3 = vertices[indices[i+2]];
            let norm = (v3 - v1).cross(v2 - v1).normalized();
            normals.push(norm);
        }

        let center = vertices
            .iter()
            .copied()
            .reduce(|acc ,v| acc + v)
            .expect("mesh has no vertices, can't be constructed")
            / vertices.len() as f32;

        let radius = vertices
            .iter()
            .map(|v| (*v - center).len_squared())
            .max_by(|a, b| a.total_cmp(b))
            .expect("invalid float found for comparison when constructing mesh")
            .sqrt();

        Mesh {
            vertices,
            indices,
            colors,
            normals, 
            properties,
            center: center,
            radius,
        }
    }

    pub fn new_with_color(vertices: Vec<Vec3>, indices: Vec<usize>, color: Vec3, properties: PhongProperties) -> Mesh {
        let colors = vec![color; indices.len() / 3];
        return Mesh::new(vertices, indices, colors, properties);
    }
    pub fn new_from_non_indexed(vertices: Vec<Vec3>, colors: Vec<Vec3>, properties: PhongProperties) -> Mesh {
        let indices = (0..vertices.len() as usize).collect();
        return Mesh::new(vertices, indices, colors, properties);
    }
    pub fn new_from_non_indexed_with_color(vertices: Vec<Vec3>, color: Vec3, properties: PhongProperties) -> Mesh {
        let colors = vec![color; vertices.len() / 3];
        return Mesh::new_from_non_indexed(vertices, colors, properties);
    }

    pub fn new_from_stl_bytes(stl_bytes: &Vec<u8>, color: Vec3, properties: PhongProperties) -> Mesh {
        let mut reader = Cursor::new(stl_bytes);
        let triangle_iter = stl_io::create_stl_reader(&mut reader).expect("Failed to create TriangleIterator from stl bytes");
        let mut vertices = Vec::with_capacity(triangle_iter.size_hint().0 * 3);
        for t in triangle_iter {
            let mut triangle = t.expect("Failed to unwrap a triangle in TriangleIterator");
            (triangle.vertices[1], triangle.vertices[2]) = (triangle.vertices[2], triangle.vertices[1]);
            for v in triangle.vertices {
                vertices.push(Vec3::new(v[0], v[1], v[2]));
            }
        }
        console_log!("stl_object num triangles: {}", vertices.len() / 3);
        return Mesh::new_from_non_indexed_with_color(vertices, color, properties);
    }

    
    // BUILDING DIFFERENT TYPES OF MESHES
    pub fn build_box_from_side_lengths(center: Vec3, x_length: f32, y_length: f32, z_length: f32, color: Vec3, properties: PhongProperties) -> Mesh {
        let x_half = 0.5 * x_length;
        let y_half = 0.5 * y_length;
        let z_half = 0.5 * z_length;
    
        let a = center - Vec3::new(x_half, y_half, z_half);
        let b = a + Vec3::new(0.0, y_length, 0.0);
        let c = a + Vec3::new(x_length, y_length, 0.0);
        let d = a + Vec3::new(x_length, 0.0, 0.0);
    
        let e = a + Vec3::new(0.0, 0.0, z_length);
        let f = b + Vec3::new(0.0, 0.0, z_length);
        let g = c + Vec3::new(0.0, 0.0, z_length);
        let h = d + Vec3::new(0.0, 0.0, z_length);
    
        let vertices = vec![a,b,c,d,e,f,g,h];
    
        let indices = vec![
            0,3,2,2,1,0,
            0,1,5,5,4,0,
            1,2,6,6,5,1,
            3,7,6,6,2,3,
            0,4,7,7,3,0,
            4,5,6,6,7,4,
        ];
    
        let colors = vec![color; indices.len() / 3];
        let mut mesh = Mesh::new(vertices, indices, colors, properties);
        mesh.center = center;
        return mesh;
    }

    pub fn build_box_from_corners(corner1: Vec3, corner2: Vec3, color: Vec3, properties: PhongProperties) -> Mesh {
        let center = 0.5 * (corner1 + corner2);
        let x_length = (corner1.x - corner2.x).abs();
        let y_length = (corner1.y - corner2.y).abs();
        let z_length = (corner1.z - corner2.z).abs();
        return Mesh::build_box_from_side_lengths(center, x_length, y_length, z_length, color, properties);
    }

    pub fn build_cube(center: Vec3, side_length: f32, color: Vec3, properties: PhongProperties) -> Mesh {
        return Mesh::build_box_from_side_lengths(center, side_length, side_length, side_length, color, properties)
    }

    pub fn build_rectangle(origin: Vec3, u: Vec3, v: Vec3, color: Vec3, properties: PhongProperties, cull_faces: bool) -> Mesh {
        let vertices = vec![origin, origin+v, origin+u, origin+u+v];
        let indices = vec![0,1,2,2,1,3];
        let mut new_props = properties.clone();
        new_props.cull_faces = cull_faces;
        return Mesh::new_with_color(vertices, indices, color, new_props);
    }
    pub fn build_checkerboard(center: Vec3, radius: i32, color1: Vec3, color2: Vec3, properties: PhongProperties, cull_faces: bool) -> Mesh {
        let mut vertices = Vec::new();
        for x in -radius..=radius {
            for y in -radius..=radius {
                vertices.push(Vec3::new(x as f32, y as f32, 0.0) + center);
            }
        }
        let mut indices = Vec::new();
        let mut colors = Vec::new();
        for x in 0..(radius as usize * 2) {
            for y in 0..(radius as usize * 2) {
                let i = (x * (radius as usize * 2 + 1) + y) as usize;
                indices.push(i);
                indices.push(i + radius as usize * 2 + 2);
                indices.push(i + radius as usize * 2 + 1);
                indices.push(i);
                indices.push(i + 1);
                indices.push(i + radius as usize * 2 + 2);
    
                let color = if (x + y) % 2 == 0 { color1 } else { color2 };
                colors.push(color);
                colors.push(color);
            }
        }
        
        let mut new_props = properties.clone();
        new_props.cull_faces = cull_faces;
        let mut mesh = Mesh::new(vertices, indices, colors, new_props);
        mesh.center = center;
        return mesh;
    }

    pub fn build_icosahedron(center: Vec3, t: f32, color: Vec3, properties: PhongProperties) -> Mesh {
        let (mut vertices, indices) = get_icosahedron_vertices_and_indices(t);
        for v in vertices.iter_mut() {
            *v += center;
        }
        let colors = vec![color; indices.len() / 3];
        let mut mesh = Mesh::new(vertices, indices, colors, properties);
        mesh.center = center;
        return mesh;
    }

    pub fn build_sphere(center: Vec3, radius: f32, subdivisions: u32, color: Vec3, properties: PhongProperties) -> Mesh {
        let (mut vertices, mut indices) = get_icosahedron_vertices_and_indices(1.0);

        for _ in 0..subdivisions {

            let mut new_indices = Vec::new();
            let mut midpoint_cache = HashMap::new();

            for i in (0..indices.len()).step_by(3) {

                let i1 = indices[i];
                let i2 = indices[i + 1];
                let i3 = indices[i + 2];

                // Get or create middle point indices
                let j1 = get_or_create_midpoint(&mut vertices, &mut midpoint_cache, i1, i2);
                let j2 = get_or_create_midpoint(&mut vertices, &mut midpoint_cache, i2, i3);
                let j3 = get_or_create_midpoint(&mut vertices, &mut midpoint_cache, i3, i1);

                new_indices.extend_from_slice(&[
                    i1, j1, j3,
                    i2, j2, j1,
                    i3, j3, j2,
                    j1, j2, j3,
                ]); 
            }
            indices = new_indices;
        }

        for v in vertices.iter_mut() {
            v.normalize();
            *v *= radius;
            *v += center;
        }

        let colors = vec![color; indices.len() / 3];
        let mut mesh =  Mesh::new(vertices, indices, colors, properties);
        mesh.center = center;
        mesh.radius = radius;
        return mesh;
    }

    pub fn to_rt_triangles(&self, material: &dyn Material) -> Vec<Triangle> {
        let mut triangles = Vec::with_capacity(self.indices.len() / 3);
        for i in (0..self.indices.len()).step_by(3) {
            let v1 = self.vertices[self.indices[i]];
            let v2 = self.vertices[self.indices[i+1]];
            let v3 = self.vertices[self.indices[i+2]];
            let color = self.colors[i / 3];

            let triangle = Triangle::new_from_vertices(v1, v2, v3, color, material);
            triangles.push(triangle);
        }
        // console_log!("{:?}", self.colors.len());
        // console_log!("{:?}", triangles.len());
        return triangles;
    }
    pub fn to_rt_hittables(&self, material: &dyn Material) -> Vec<Box<dyn Hittable>> {
        return self
            .to_rt_triangles(material)
            .iter()
            .map(|t| Box::new(t.clone()) as Box<dyn Hittable>)
            .collect()
    }


    pub fn translate_by(&mut self, offset: Vec3) {
        for v in self.vertices.iter_mut() {
            *v += offset;
        }
    }
    pub fn translate_to(&mut self, destination: Vec3) {
        let offset = destination - self.center;
        self.translate_by(offset);
    }
    pub fn set_center(&mut self, new_center: Vec3) {
        self.translate_to(new_center);
    }
    
    /// Rotates in the z direction first, then y direction
    pub fn rotate_around(&mut self, center_of_rotation: Vec3, theta_z: f32, theta_y: f32) {
        for v in self.vertices.iter_mut() {
            *v -= center_of_rotation;
            let (sin_z, cos_z) = theta_z.sin_cos();
            let (sin_y, cos_y) = theta_y.sin_cos();
            v.rotate_z_fast(sin_z, cos_z);
            v.rotate_y_fast(sin_y, cos_y);
            *v += center_of_rotation;
        }
    }
    /// Rotates in the z direction first, then y direction
    pub fn rotate_around_center(&mut self, theta_z: f32, theta_y: f32) {
        self.rotate_around(self.center, theta_z, theta_y);
    }

    pub fn scale_around(&mut self, center_of_scale: Vec3, scale_factor: f32) {
        for v in self.vertices.iter_mut() {
            *v -= center_of_scale;
            *v *= scale_factor;
            *v += center_of_scale;
        }
    }
    pub fn scale_by(&mut self, scale_factor: f32) {
        self.scale_around(self.center, scale_factor);
    }
}

fn get_icosahedron_vertices_and_indices(t: f32) -> (Vec<Vec3>, Vec<usize>) {
    let vertices = vec![
        Vec3::new(-1.0, t, 0.0),
        Vec3::new(1.0, t, 0.0),
        Vec3::new(-1.0, -t, 0.0),
        Vec3::new(1.0, -t, 0.0),

        Vec3::new(0.0, -1.0, t),
        Vec3::new(0.0, 1.0, t),
        Vec3::new(0.0, -1.0, -t),
        Vec3::new(0.0, 1.0, -t),

        Vec3::new(t, 0.0, -1.0),
        Vec3::new(t, 0.0, 1.0),
        Vec3::new(-t, 0.0, -1.0),
        Vec3::new(-t, 0.0, 1.0),
    ];

    let indices = vec![
        11,0,5,
        5,0,1,
        1,0,7,
        7,0,10,
        10,0,11,

        5,1,9,
        11,5,4,
        10,11,2,
        7,10,6,
        1,7,8,

        9,3,4,
        4,3,2,
        2,3,6,
        6,3,8,
        8,3,9,

        9,4,5,
        4,2,11,
        2,6,10,
        6,8,7,
        8,9,1,
    ];

    return (vertices, indices);
}

fn get_or_create_midpoint(vertices: &mut Vec<Vec3>, midpoint_cache: &mut HashMap<(usize, usize), usize>, i1: usize, i2: usize) -> usize {
    let key = if i1 < i2 { (i1, i2) } else { (i2, i1) };
    if let Some(&index) = midpoint_cache.get(&key) {
        return index;
    }

    let v1 = vertices[i1];
    let v2 = vertices[i2];
    let midpoint = v1.midpoint_with(v2);
    let index = vertices.len();
    vertices.push(midpoint);
    midpoint_cache.insert(key, index);
    return index;
}