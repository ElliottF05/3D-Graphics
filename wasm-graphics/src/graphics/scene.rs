use crate::{console_log, utils::math::Vec3};
use std::sync::atomic::{AtomicUsize, Ordering};

static NEXT_ID: AtomicUsize = AtomicUsize::new(0);

pub struct MaterialProperties {
    pub color: Vec3,
    pub ambient: f32,
    pub diffuse: f32,
    pub specular: f32,
    pub shininess: f32,
}

impl MaterialProperties {
    pub fn new(color: Vec3, ambient: f32, diffuse: f32, specular: f32, shininess: f32) -> MaterialProperties {
        MaterialProperties {
            color,
            ambient,
            diffuse,
            specular,
            shininess,
        }
    }
    pub fn default_from_color(color: Vec3) -> MaterialProperties {
        MaterialProperties::new(color, 0.1, 1.0, 0.000014, 32.0)
    }
}

pub trait SceneObject {
    fn get_vertices(&self) -> &Vec<Vec3>;
    fn get_properties(&self) -> &MaterialProperties;
    fn get_id(&self) -> usize;
}

pub struct VertexObject {
    pub vertices: Vec<Vec3>,
    pub properties: MaterialProperties,
    pub id: usize,
}

impl SceneObject for VertexObject {
    fn get_vertices(&self) -> &Vec<Vec3> {
        &self.vertices
    }
    fn get_properties(&self) -> &MaterialProperties {
        &self.properties
    }
    fn get_id(&self) -> usize {
        return self.id;
    }
}

impl VertexObject {
    pub fn new(vertices: Vec<Vec3>, properties: MaterialProperties) -> VertexObject {
        VertexObject {
            vertices,
            properties,
            id: NEXT_ID.fetch_add(1,Ordering::Relaxed),
        }
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

pub fn build_checkerboard(center: &Vec3, radius: i32, color1: &Vec3, color2: &Vec3) -> Vec<VertexObject> {
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
        VertexObject::new(vertices1, MaterialProperties::default_from_color(color1.clone())),
        VertexObject::new(vertices2, MaterialProperties::default_from_color(color2.clone())),
    ];

}