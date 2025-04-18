use gltf::json::extensions::material;

use crate::{console_log, utils::math::Vec3, graphics::ray_tracing::rt::Ray};
use std::{collections::HashMap, fmt::Debug, io::Cursor, sync::atomic::{AtomicUsize, Ordering}, vec};

use super::ray_tracing::{bvh::AABoundingBox, hittable::{Hittable, Triangle}, material::Material, rt::HitRecord};

static NEXT_ID: AtomicUsize = AtomicUsize::new(0);

#[derive(Debug, Clone)]
pub struct PhongProperties {
    pub alpha: f32,
    pub ambient: f32,
    pub diffuse: f32,
    pub specular: f32,
    pub shininess: i32,
}

impl PhongProperties {
    pub fn new(alpha: f32, ambient: f32, diffuse: f32, specular: f32, shininess: i32) -> PhongProperties {
        PhongProperties {
            alpha,
            ambient,
            diffuse,
            specular,
            shininess,
        }
    }
}

impl Default for PhongProperties {
    fn default() -> Self {
        PhongProperties::new(1.0, 1.0, 0.8, 1.0, 32)
    }
}

#[derive(Debug)]
pub struct Mesh {
    pub vertices: Vec<Vec3>,
    pub indices: Vec<usize>,
    pub colors: Vec<Vec3>,
    pub normals: Vec<Vec3>,

    pub properties: PhongProperties,

    pub center: Vec3,
    pub radius: f32,

    id: usize,
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
            id: NEXT_ID.fetch_add(1, Ordering::Relaxed),
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
    pub fn build_cube(pos: Vec3, side_length: f32, color: Vec3, properties: PhongProperties) -> Mesh {
        let half = side_length / 2.0;
    
        let a = pos - Vec3::new(half, half, half);
        let b = a + Vec3::new(0.0, side_length, 0.0);
        let c = a + Vec3::new(side_length, side_length, 0.0);
        let d = a + Vec3::new(side_length, 0.0, 0.0);
    
        let e = a + Vec3::new(0.0, 0.0, side_length);
        let f = b + Vec3::new(0.0, 0.0, side_length);
        let g = c + Vec3::new(0.0, 0.0, side_length);
        let h = d + Vec3::new(0.0, 0.0, side_length);
    
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
        mesh.center = pos;
        return mesh;
    }
    
    pub fn build_checkerboard(center: Vec3, radius: i32, color1: Vec3, color2: Vec3, properties: PhongProperties) -> Mesh {
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
    
        let mut mesh = Mesh::new(vertices, indices, colors, properties);
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
        return triangles;
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
    pub fn scale(&mut self, scale_factor: f32) {
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

// pub trait SceneObject: Debug {
//     fn get_vertices(&self) -> &Vec<Vec3>;
//     fn get_vertices_mut(&mut self) -> &mut Vec<Vec3>;
//     fn get_indices(&self) -> &Vec<usize>;
//     fn get_colors(&self) -> &Vec<Vec3>;
//     fn get_normals(&self) -> &Vec<Vec3>;
//     fn get_properties(&self) -> &PhongProperties;
//     fn get_material(&self) -> &Box<dyn Material>;
//     fn get_id(&self) -> usize;
//     fn get_center(&self) -> Vec3;
//     fn set_center(&mut self, center: Vec3);

//     fn translate(&mut self, translation: Vec3) {
//         self.set_center(self.get_center() + translation);
//     }
//     fn rotate_z(&mut self, theta_z: f32) {
//         let center = self.get_center();
//         let (sin_z, cos_z) = theta_z.sin_cos();
//         for v in self.get_vertices_mut().iter_mut() {
//             *v -= center;
//             v.rotate_z_fast(sin_z, cos_z);
//             *v += center;
//         }
//     }
//     fn rotate_y(&mut self, theta_y: f32) {
//         let center = self.get_center();
//         let (sin_y, cos_y) = theta_y.sin_cos();
//         for v in self.get_vertices_mut().iter_mut() {
//             *v -= center;
//             v.rotate_y_fast(sin_y, cos_y);
//             *v += center;
//         }
//     }
//     fn scale_by(&mut self, scale: f32) {
//         let center = self.get_center();
//         for v in self.get_vertices_mut().iter_mut() {
//             *v -= center;
//             *v *= scale;
//             *v += center;
//         }
//     }
//     fn scale_to_radius(&mut self, radius: f32) {
//         let center = self.get_center();
//         let max_dist = self
//             .get_vertices()
//             .iter()
//             .map(|v| (*v - center).len_squared())
//             .max_by(|a,b| a.total_cmp(b))
//             .unwrap()
//             .sqrt();
//         let scale = radius / max_dist;
//         self.scale_by(scale);
//     }

//     fn clone_box(&self) -> Box<dyn SceneObject>;
// }

// impl Clone for Box<dyn SceneObject> {
//     fn clone(&self) -> Self {
//         self.as_ref().clone_box()
//     }
// }

// #[derive(Debug, Clone)]
// pub struct VertexObject {
//     pub vertices: Vec<Vec3>,
//     pub indices: Vec<usize>,
//     pub colors: Vec<Vec3>,
//     pub normals: Vec<Vec3>,

//     pub properties: PhongProperties,
//     pub material: Box<dyn Material>,

//     pub id: usize,

//     pub center: Vec3,
//     pub bounding_box: AABoundingBox,
// }

// impl SceneObject for VertexObject {
//     fn get_vertices(&self) -> &Vec<Vec3> {
//         &self.vertices
//     }
//     fn get_vertices_mut(&mut self) -> &mut Vec<Vec3> {
//         &mut self.vertices
//     }
//     fn get_indices(&self) -> &Vec<usize> {
//         return &self.indices;
//     }
//     fn get_colors(&self) -> &Vec<Vec3> {
//         return &self.colors;
//     }
//     fn get_normals(&self) -> &Vec<Vec3> {
//         return &self.normals;
//     }
//     fn get_properties(&self) -> &PhongProperties {
//         &self.properties
//     }
//     fn get_material(&self) -> &Box<dyn Material> {
//         return &self.material;
//     }
//     fn get_id(&self) -> usize {
//         return self.id;
//     }
//     fn get_center(&self) -> Vec3 {
//         return self.center;
//     }
//     fn set_center(&mut self, center: Vec3) {
//         let delta = center - self.center;
//         for v in self.get_vertices_mut().iter_mut() {
//             *v += delta;
//         }
//         self.center = center;
//     }

//     fn clone_box(&self) -> Box<dyn SceneObject> {
//         Box::new(self.clone())
//     }
// }


// impl VertexObject {
//     pub fn new(vertices: Vec<Vec3>, indices: Vec<usize>, colors: Vec<Vec3>, properties: PhongProperties, material: Box<dyn Material>) -> VertexObject {
//         let mut center = Vec3::new(0.0, 0.0, 0.0);
//         for v in &vertices {
//             center += *v;
//         }
//         center /= vertices.len() as f32;

//         let mut normals = Vec::with_capacity(colors.len());
//         let mut min_corner = vertices[0];
//         let mut max_corner = vertices[0];
//         for i in (0..indices.len()).step_by(3) {
//             let v1 = vertices[indices[i]];
//             let v2 = vertices[indices[i + 1]];
//             let v3 = vertices[indices[i + 2]];
//             let normal = (v3 - v1).cross(v2 - v1).normalized();
//             normals.push(normal);

//             min_corner.min_elementwise(v1);
//             min_corner.min_elementwise(v2);
//             min_corner.min_elementwise(v3);

//             max_corner.max_elementwise(v1);
//             max_corner.max_elementwise(v2);
//             max_corner.max_elementwise(v3);
//         }

//         let bounding_box = AABoundingBox::new(min_corner, max_corner);

//         VertexObject {
//             vertices,
//             indices,
//             colors,
//             normals,
//             properties,
//             material,
//             id: NEXT_ID.fetch_add(1,Ordering::Relaxed),
//             center,
//             bounding_box,
//         }
//     }
//     pub fn new_with_color(vertices: Vec<Vec3>, indices: Vec<usize>, color: Vec3, properties: PhongProperties, material: Box<dyn Material>) -> VertexObject {
//         let colors = vec![color; indices.len() / 3];
//         return VertexObject::new(vertices, indices, colors, properties, material);
//     }
//     pub fn new_from_non_indexed(vertices: Vec<Vec3>, colors: Vec<Vec3>, properties: PhongProperties, material: Box<dyn Material>) -> VertexObject {
//         let indices = (0..vertices.len() as usize).collect();
//         return VertexObject::new(vertices, indices, colors, properties, material);
//     }
//     pub fn new_from_non_indexed_with_color(vertices: Vec<Vec3>, color: Vec3, properties: PhongProperties, material: Box<dyn Material>) -> VertexObject {
//         let colors = vec![color; vertices.len() / 3];
//         return VertexObject::new_from_non_indexed(vertices, colors, properties, material);
//     }

//     pub fn new_from_stl_bytes(stl_bytes: &Vec<u8>, color: Vec3, properties: PhongProperties, material: Box<dyn Material>) -> VertexObject {
//         let mut reader = Cursor::new(stl_bytes);
//         let triangle_iter = stl_io::create_stl_reader(&mut reader).expect("Failed to create TriangleIterator from stl bytes");
//         let mut vertices = Vec::with_capacity(triangle_iter.size_hint().0 * 3);
//         for t in triangle_iter {
//             let mut triangle = t.expect("Failed to unwrap a triangle in TriangleIterator");
//             (triangle.vertices[1], triangle.vertices[2]) = (triangle.vertices[2], triangle.vertices[1]);
//             for v in triangle.vertices {
//                 vertices.push(Vec3::new(v[0], v[1], v[2]));
//             }
//         }
//         console_log!("stl_object num triangles: {}", vertices.len() / 3);
//         return VertexObject::new_from_non_indexed_with_color(vertices, color, properties, material);
//     }
// }



// #[derive(Debug, Clone)]
// pub struct Sphere {
//     pub mesh: VertexObject,
//     pub radius: f32,
//     pub bounding_box: AABoundingBox,
// }

// impl SceneObject for Sphere {
//     fn get_vertices(&self) -> &Vec<Vec3> {
//         &self.mesh.vertices
//     }
//     fn get_vertices_mut(&mut self) -> &mut Vec<Vec3> {
//         &mut self.mesh.vertices
//     }
//     fn get_indices(&self) -> &Vec<usize> {
//         return &self.mesh.indices;
//     }
//     fn get_colors(&self) -> &Vec<Vec3> {
//         return &self.mesh.colors;
//     }
//     fn get_normals(&self) -> &Vec<Vec3> {
//         return &self.mesh.normals;
//     }
//     fn get_properties(&self) -> &PhongProperties {
//         &self.mesh.properties
//     }
//     fn get_material(&self) -> &Box<dyn Material> {
//         return &self.mesh.material;
//     }
//     fn get_id(&self) -> usize {
//         return self.mesh.id;
//     }
//     fn get_center(&self) -> Vec3 {
//         return self.mesh.center;
//     }
//     fn set_center(&mut self, center: Vec3) {
//         let delta = center - self.mesh.center;
//         for v in self.get_vertices_mut().iter_mut() {
//             *v += delta;
//         }
//         self.mesh.center = center;
//     }

//     fn clone_box(&self) -> Box<dyn SceneObject> {
//         Box::new(self.clone())
//     }
// }

// impl Sphere {
//     pub fn new(vertices: Vec<Vec3>, indices: Vec<usize>, colors: Vec<Vec3>, center: Vec3, radius: f32, properties: PhongProperties, material: Box<dyn Material>) -> Sphere {

//         let mut normals = Vec::with_capacity(colors.len());
//         for i in (0..indices.len()).step_by(3) {
//             let v1 = vertices[indices[i]];
//             let v2 = vertices[indices[i + 1]];
//             let v3 = vertices[indices[i + 2]];
//             let normal = (v3 - v1).cross(v2 - v1).normalized();
//             normals.push(normal);
//         }

//         let radius_vector = Vec3::new(radius, radius, radius);
//         let bounding_box = AABoundingBox::new(center - radius_vector, center + radius_vector);

//         let mesh = VertexObject {
//             vertices,
//             indices,
//             colors,
//             normals,
//             properties,
//             material,
//             id: NEXT_ID.fetch_add(1,Ordering::Relaxed),
//             center,
//             bounding_box: bounding_box.clone(),
//         };

//         Sphere {
//             mesh,
//             radius,
//             bounding_box
//         }
//     }

//     // building an icosphere
//     pub fn build_sphere(center: Vec3, radius: f32, subdivisions: u32, color: Vec3, properties: PhongProperties, material: Box<dyn Material>) -> Sphere {
//         let (mut vertices, mut indices) = get_icosahedron_vertices_and_indices(1.0);

//         for _ in 0..subdivisions {

//             let mut new_indices = Vec::new();
//             let mut midpoint_cache = HashMap::new();

//             for i in (0..indices.len()).step_by(3) {

//                 let i1 = indices[i];
//                 let i2 = indices[i + 1];
//                 let i3 = indices[i + 2];

//                 // Get or create middle point indices
//                 let j1 = get_or_create_midpoint(&mut vertices, &mut midpoint_cache, i1, i2);
//                 let j2 = get_or_create_midpoint(&mut vertices, &mut midpoint_cache, i2, i3);
//                 let j3 = get_or_create_midpoint(&mut vertices, &mut midpoint_cache, i3, i1);

//                 new_indices.extend_from_slice(&[
//                     i1, j1, j3,
//                     i2, j2, j1,
//                     i3, j3, j2,
//                     j1, j2, j3,
//                 ]); 
//             }
//             indices = new_indices;
//         }

//         for v in vertices.iter_mut() {
//             v.normalize();
//             *v *= radius;
//             *v += center;
//         }

//         let colors = vec![color; indices.len() / 3];
//         return Sphere::new(vertices, indices, colors, center, radius, properties, material);
//     }

//     pub fn get_mesh(&self) -> &VertexObject {
//         &self.mesh
//     }
// }

// pub fn build_cube(pos: Vec3, side_length: f32, color: Vec3, properties: PhongProperties, material: Box<dyn Material>) -> VertexObject {
//     let half = side_length / 2.0;

//     let a = pos - Vec3::new(half, half, half);
//     let b = a + Vec3::new(0.0, side_length, 0.0);
//     let c = a + Vec3::new(side_length, side_length, 0.0);
//     let d = a + Vec3::new(side_length, 0.0, 0.0);

//     let e = a + Vec3::new(0.0, 0.0, side_length);
//     let f = b + Vec3::new(0.0, 0.0, side_length);
//     let g = c + Vec3::new(0.0, 0.0, side_length);
//     let h = d + Vec3::new(0.0, 0.0, side_length);

//     let mut vertices = vec![a,b,c,d,e,f,g,h];

//     let faces = [
//         [a, d, c, c, b, a], // Front
//         [a, b, f, f, e, a], // Left
//         [b, c, g, g, f, b], // Top
//         [d, h, g, g, c, d], // Right
//         [a, e, h, h, d, a], // Bottom
//         [e, f, g, g, h, e], // Back
//     ];

//     let indices = vec![
//         0,3,2,2,1,0,
//         0,1,5,5,4,0,
//         1,2,6,6,5,1,
//         3,7,6,6,2,3,
//         0,4,7,7,3,0,
//         4,5,6,6,7,4,
//     ];

//     let colors = vec![color; indices.len() / 3];
//     VertexObject::new(vertices, indices, colors, properties, material)
// }

// pub fn build_checkerboard(center: Vec3, radius: i32, color1: Vec3, color2: Vec3, properties: PhongProperties, material: Box<dyn Material>) -> VertexObject {
//     let mut vertices = Vec::new();
//     for x in -radius..=radius {
//         for y in -radius..=radius {
//             vertices.push(Vec3::new(x as f32, y as f32, 0.0) + center);
//         }
//     }
//     let mut indices = Vec::new();
//     let mut colors = Vec::new();
//     for x in 0..(radius as usize * 2) {
//         for y in 0..(radius as usize * 2) {
//             let i = (x * (radius as usize * 2 + 1) + y) as usize;
//             indices.push(i);
//             indices.push(i + radius as usize * 2 + 2);
//             indices.push(i + radius as usize * 2 + 1);
//             indices.push(i);
//             indices.push(i + 1);
//             indices.push(i + radius as usize * 2 + 2);

//             let color = if (x + y) % 2 == 0 { color1 } else { color2 };
//             colors.push(color);
//             colors.push(color);
//         }
//     }

//     return VertexObject::new(vertices, indices, colors, properties, material);
// }

// pub fn build_icosahedron(center: Vec3, t: f32, color: Vec3, properties: PhongProperties, material: Box<dyn Material>) -> VertexObject {
//     let (mut vertices, indices) = get_icosahedron_vertices_and_indices(t);
//     for v in vertices.iter_mut() {
//         *v += center;
//     }
//     let colors = vec![color; indices.len() / 3];
//     return VertexObject::new(vertices, indices, colors, properties, material);
// }