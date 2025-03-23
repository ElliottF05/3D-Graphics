use crate::{console_log, utils::math::Vec3};
use std::{io::Cursor, sync::atomic::{AtomicUsize, Ordering}};

static NEXT_ID: AtomicUsize = AtomicUsize::new(0);

pub struct MaterialProperties {
    pub color: Vec3,
    pub alpha: f32,
    pub ambient: f32,
    pub diffuse: f32,
    pub specular: f32,
    pub shininess: i32,
}

impl MaterialProperties {
    pub fn new(color: Vec3, alpha: f32, ambient: f32, diffuse: f32, specular: f32, shininess: i32) -> MaterialProperties {
        MaterialProperties {
            alpha,
            color,
            ambient,
            diffuse,
            specular,
            shininess,
        }
    }
    pub fn default_from_color(color: Vec3) -> MaterialProperties {
        MaterialProperties::new(color, 1.0, 0.8, 1.0, 0.5, 32)
    }
}

impl Default for MaterialProperties {
    fn default() -> Self {
        MaterialProperties::new(Vec3::new(1.0, 1.0, 1.0), 1.0, 0.8, 1.0, 0.5, 32)
    }
}

pub trait SceneObject {
    fn get_vertices(&self) -> &Vec<Vec3>;
    fn get_vertices_mut(&mut self) -> &mut Vec<Vec3>;
    fn get_properties(&self) -> &MaterialProperties;
    fn get_id(&self) -> usize;
    fn get_center(&self) -> Vec3;
    fn set_center(&mut self, center: Vec3);

    fn translate(&mut self, translation: Vec3) {
        self.set_center(self.get_center() + translation);
    }
    fn rotate_z(&mut self, theta_z: f32) {
        let center = self.get_center();
        let (sin_z, cos_z) = theta_z.sin_cos();
        for v in self.get_vertices_mut().iter_mut() {
            *v -= center;
            v.rotate_z_fast(sin_z, cos_z);
            *v += center;
        }
    }
    fn rotate_y(&mut self, theta_y: f32) {
        let center = self.get_center();
        let (sin_y, cos_y) = theta_y.sin_cos();
        for v in self.get_vertices_mut().iter_mut() {
            *v -= center;
            v.rotate_y_fast(sin_y, cos_y);
            *v += center;
        }
    }
    fn scale_by(&mut self, scale: f32) {
        let center = self.get_center();
        for v in self.get_vertices_mut().iter_mut() {
            *v -= center;
            *v *= scale;
            *v += center;
        }
    }
    fn scale_to_radius(&mut self, radius: f32) {
        let center = self.get_center();
        let max_dist = self
            .get_vertices()
            .iter()
            .map(|v| (*v - center).len_squared())
            .max_by(|a,b| a.total_cmp(b))
            .unwrap()
            .sqrt();
        let scale = radius / max_dist;
        self.scale_by(scale);
    }
}

pub struct VertexObject {
    pub vertices: Vec<Vec3>,
    pub properties: MaterialProperties,
    pub id: usize,
    pub center: Vec3,
}

impl SceneObject for VertexObject {
    fn get_vertices(&self) -> &Vec<Vec3> {
        &self.vertices
    }
    fn get_vertices_mut(&mut self) -> &mut Vec<Vec3> {
        &mut self.vertices
    }
    fn get_properties(&self) -> &MaterialProperties {
        &self.properties
    }
    fn get_id(&self) -> usize {
        return self.id;
    }
    fn get_center(&self) -> Vec3 {
        return self.center;
    }
    fn set_center(&mut self, center: Vec3) {
        let delta = center - self.center;
        for v in self.get_vertices_mut().iter_mut() {
            *v += delta;
        }
        self.center = center;
    }
}

impl VertexObject {
    pub fn new(vertices: Vec<Vec3>, properties: MaterialProperties) -> VertexObject {
        let mut center = Vec3::new(0.0, 0.0, 0.0);
        for v in & vertices {
            center += *v;
        }
        center /= vertices.len() as f32;
        VertexObject {
            vertices,
            properties,
            id: NEXT_ID.fetch_add(1,Ordering::Relaxed),
            center,
        }
    }
    pub fn new_from_indexed(vertices: &Vec<Vec3>, indices: &Vec<u32>, reverse_winding_order: bool, properties: MaterialProperties) -> VertexObject {
        let mut new_vertices = Vec::with_capacity(indices.len() as usize);
        if reverse_winding_order {
            for i in indices.iter().rev() {
                new_vertices.push(vertices[*i as usize]);
            }
        } else {
            for i in indices {
                new_vertices.push(vertices[*i as usize]);
            }
        }

        return VertexObject::new(new_vertices, properties);
    }

    pub fn new_from_stl_bytes(stl_bytes: &Vec<u8>, properties: MaterialProperties) -> VertexObject {
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
        return VertexObject::new(vertices, properties);
    }
}

pub struct Sphere {
    pub vertices: Vec<Vec3>,
    pub properties: MaterialProperties,
    pub id: usize,
    pub center: Vec3,
    pub radius: f32,
}

impl SceneObject for Sphere {
    fn get_vertices(&self) -> &Vec<Vec3> {
        &self.vertices
    }
    fn get_vertices_mut(&mut self) -> &mut Vec<Vec3> {
        &mut self.vertices
    }
    fn get_properties(&self) -> &MaterialProperties {
        &self.properties
    }
    fn get_id(&self) -> usize {
        return self.id;
    }
    fn get_center(&self) -> Vec3 {
        return self.center;
    }
    fn set_center(&mut self, center: Vec3) {
        let delta = center - self.center;
        for v in self.get_vertices_mut().iter_mut() {
            *v += delta;
        }
        self.center = center;
    }
}

impl Sphere {
    pub fn new(vertices: Vec<Vec3>, center: Vec3, radius: f32, properties: MaterialProperties) -> Sphere {
        Sphere {
            vertices,
            properties,
            id: NEXT_ID.fetch_add(1,Ordering::Relaxed),
            center,
            radius,
        }
    }

    // building an icosphere
    pub fn build_sphere(center: Vec3, radius: f32, iterations: u32, properties: MaterialProperties) -> Sphere {
        let mut vertices = get_icosahedron_vertices(1.0);
        for _ in 0..iterations {
            let mut next_vertices = Vec::with_capacity(vertices.len() * (4 as usize).pow(iterations));
            for i in (0..vertices.len()).step_by(3) {
                let (p1, p2, p3) = (vertices[i], vertices[i+1], vertices[i+2]);
                let (m1, m2, m3) = (p1.midpoint_with(&p2), p2.midpoint_with(&p3), p3.midpoint_with(&p1));

                next_vertices.push(p1);
                next_vertices.push(m1);
                next_vertices.push(m3);

                next_vertices.push(m1);
                next_vertices.push(p2);
                next_vertices.push(m2);

                next_vertices.push(m3);
                next_vertices.push(m2);
                next_vertices.push(p3);

                next_vertices.push(m1);
                next_vertices.push(m2);
                next_vertices.push(m3);
            }
            vertices = next_vertices;
        }

        for v in vertices.iter_mut() {
            v.normalize();
            *v *= radius;
            *v += center;
        }

        return Sphere::new(vertices, center, radius, properties);
    }
}

pub fn build_cube(pos: Vec3, side_length: f32, properties: MaterialProperties) -> VertexObject {
    let half = side_length / 2.0;

    let a = pos - Vec3::new(half, half, half);
    let b = a + Vec3::new(0.0, side_length, 0.0);
    let c = a + Vec3::new(side_length, side_length, 0.0);
    let d = a + Vec3::new(side_length, 0.0, 0.0);

    let e = a + Vec3::new(0.0, 0.0, side_length);
    let f = b + Vec3::new(0.0, 0.0, side_length);
    let g = c + Vec3::new(0.0, 0.0, side_length);
    let h = d + Vec3::new(0.0, 0.0, side_length);

    let mut vertices = Vec::with_capacity(36);

    let faces = [
        [a, d, c, c, b, a], // Front
        [a, b, f, f, e, a], // Left
        [b, c, g, g, f, b], // Top
        [d, h, g, g, c, d], // Right
        [a, e, h, h, d, a], // Bottom
        [e, f, g, g, h, e], // Back
    ];

    for face in faces.iter() {
        vertices.extend_from_slice(face);
    }

    VertexObject::new(vertices, properties)
}

pub fn build_checkerboard(center: Vec3, radius: i32, properties1: MaterialProperties, properties2: MaterialProperties) -> Vec<VertexObject> {
    let mut vertices1 = Vec::new();
    let mut vertices2 = Vec::new();

    for x in -radius..radius {
        for y in -radius..radius {
            let target_vertices = if (x + y) % 2 == 0 { &mut vertices1 } else { &mut vertices2 };
            let a = Vec3::new(center.x + x as f32, center.y + y as f32, center.z);
            let b = a + Vec3::new(1.0, 0.0, 0.0);
            let c = a + Vec3::new(1.0, 1.0, 0.0);
            let d = a + Vec3::new(0.0, 1.0, 0.0);

            target_vertices.extend_from_slice(&[a, c, b, a, d, c]);
        }
    }

    return vec![
        VertexObject::new(vertices1, properties1),
        VertexObject::new(vertices2, properties2),
    ];
}

pub fn build_checkerboard_with_color(center: Vec3, radius: i32, color1: Vec3, color2: Vec3) -> Vec<VertexObject> {
    return build_checkerboard(center, radius, MaterialProperties::default_from_color(color1), MaterialProperties::default_from_color(color2));
}

pub fn get_icosahedron_vertices(t: f32) -> Vec<Vec3> {
    let base_vertices = vec![
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

    let vertices = vec![
        base_vertices[11], base_vertices[0], base_vertices[5],
        base_vertices[5], base_vertices[0], base_vertices[1],
        base_vertices[1], base_vertices[0], base_vertices[7],
        base_vertices[7], base_vertices[0], base_vertices[10],
        base_vertices[10], base_vertices[0], base_vertices[11],

        base_vertices[5], base_vertices[1], base_vertices[9],
        base_vertices[11], base_vertices[5], base_vertices[4],
        base_vertices[10], base_vertices[11], base_vertices[2],
        base_vertices[7], base_vertices[10], base_vertices[6],
        base_vertices[1], base_vertices[7], base_vertices[8],

        base_vertices[9], base_vertices[3], base_vertices[4],
        base_vertices[4], base_vertices[3], base_vertices[2],
        base_vertices[2], base_vertices[3], base_vertices[6],
        base_vertices[6], base_vertices[3], base_vertices[8],
        base_vertices[8], base_vertices[3], base_vertices[9],

        base_vertices[9], base_vertices[4], base_vertices[5],
        base_vertices[4], base_vertices[2], base_vertices[11],
        base_vertices[2], base_vertices[6], base_vertices[10],
        base_vertices[6], base_vertices[8], base_vertices[7],
        base_vertices[8], base_vertices[9], base_vertices[1],
    ];

    return vertices;
}

pub fn build_icosahedron(center: Vec3, t: f32, properties: MaterialProperties) -> VertexObject {
    let mut vertices = get_icosahedron_vertices(t);
    for v in vertices.iter_mut() {
        *v += center;
    }
    return VertexObject::new(vertices, properties);
}

pub fn build_sphere(center: Vec3, radius: f32, iterations: u32, properties: MaterialProperties) -> Sphere {
    return Sphere::build_sphere(center, radius, iterations, properties);
}