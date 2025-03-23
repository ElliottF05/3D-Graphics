use gltf::{Gltf, buffer::{Data, Source}};

use crate::utils::math::Vec3;

use super::scene::{MaterialProperties, VertexObject};

// First parse just the GLTF structure
pub fn decode_gltf_bytes(gltf_bytes: &[u8], bin_bytes: &[u8]) 
    -> Result<(Gltf, Vec<Data>), String> {
    
    // Parse the GLTF structure
    let gltf = match gltf::Gltf::from_slice(gltf_bytes) {
        Ok(gltf) => gltf,
        Err(e) => return Err(format!("Failed to parse GLTF: {}", e)),
    };
    
    let buffer = match Data::from_source_and_blob(Source::Bin, None, &mut Some(bin_bytes.to_vec())) {
        Ok(buffer) => buffer,
        Err(e) => return Err(format!("Failed to create buffer: {}", e)),
    };
    let buffers = vec![buffer];

    return Ok((gltf, buffers));
}

pub fn parse_gltf_objects(gltf: Gltf, buffers: &[Data]) 
    -> Result<Vec<VertexObject>, String> {
    
    let mut vertex_objects = Vec::new();

    for mesh in gltf.meshes() {
        let mesh_objects = parse_gltf_mesh(mesh, buffers)?;
        vertex_objects.extend(mesh_objects);
    }

    Ok(vertex_objects)
}

fn parse_gltf_mesh(mesh: gltf::Mesh, buffers: &[Data]) -> Result<Vec<VertexObject>, String> {
    let mut vertex_objects = Vec::new();
    for primitive in mesh.primitives() {
        // Fix: correctly pass the closure to access buffer data
        let reader = primitive.reader(|buf| Some(&buffers[buf.index()]));
    
        // Get positions and indices data
        let vertices = match reader.read_positions() {
            Some(positions) => positions.map(|p| Vec3::new(p[0], p[1], p[2])).collect::<Vec<_>>(),
            None => return Err("Mesh has no position data".to_string()),
        };
        
        let indices: Vec<u32> = match reader.read_indices() {
            Some(indices_reader) => indices_reader.into_u32().collect(),
            None => return Err("Mesh has no index data".to_string()),
        };

        // TODO: add colors and textures

        let vertex_object = VertexObject::new_from_indexed(&vertices, &indices, MaterialProperties::default_from_color(Vec3::new(0.5, 0.5, 0.5)));
        
        vertex_objects.push(vertex_object);
    }

    Ok(vertex_objects)
}