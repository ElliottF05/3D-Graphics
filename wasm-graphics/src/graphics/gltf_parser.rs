use gltf::{buffer::{Data, Source}, image, json::extensions::material, mesh::util::tex_coords, Gltf, Primitive};
use ::image::{load_from_memory, GenericImageView};
use wasm_bindgen::prelude::*;
use std::f32::consts::PI;
use crate::{console_error, console_log, wasm::wasm::GAME_INSTANCE};

use crate::utils::math::Vec3;

use super::scene::{MaterialProperties, VertexObject};

#[wasm_bindgen]
pub fn load_gltf_model(gltf_bytes: &[u8], bin_bytes: &[u8]) -> bool {
    match decode_gltf_bytes(gltf_bytes, bin_bytes) {
        Ok((gltf, buffers)) => {
            match parse_gltf_objects(gltf, &buffers) {
                Ok(mut vertex_objects) => {

                    // TODO: remove later, this is for testing
                    for obj in vertex_objects.iter_mut() {
                        for vertex in obj.vertices.iter_mut() {
                            // vertex.rotate_z(PI / 2.0);
                            vertex.rotate_y(-PI / 2.0);
                            *vertex *= 0.2;
                            vertex.x += 10.0;
                            vertex.z += 5.0;
                        }
                    }

                    GAME_INSTANCE.with(|game_instance| {
                        game_instance.borrow_mut().add_scene_objects(vertex_objects);
                    });
                    true
                },
                Err(e) => {
                    console_error!("GLTF parse error on parse_gltf_objects(): {}", e);
                    false
                }
            }
        },
        Err(e) => {
            console_error!("GLTF parse error on decode_gltf_bytes(): {}", e);
            false
        }
    }
}

// First parse just the GLTF structure
pub fn decode_gltf_bytes(gltf_bytes: &[u8], bin_bytes: &[u8]) 
    -> Result<(Gltf, Vec<Data>), String> {
    
    // Parse the GLTF structure
    let gltf = match gltf::Gltf::from_slice(gltf_bytes) {
        Ok(gltf) => gltf,
        Err(e) => return Err(format!("Failed to parse GLTF: {}", e)),
    };

    console_log!("buffers().len(): {:?}", gltf.buffers().len());
    console_log!("buffers(): {:?}", gltf.buffers());
    console_log!("images().len(): {:?}", gltf.images().len());
    console_log!("images(): {:?}", gltf.images());

    console_log!("has blob: {:?}", gltf.blob.is_some());
    
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
        let mesh_objects = parse_gltf_mesh(&gltf, mesh, buffers)?;
        vertex_objects.extend(mesh_objects);
    }

    Ok(vertex_objects)
}

fn parse_gltf_mesh(gltf: &Gltf, mesh: gltf::Mesh, buffers: &[Data]) -> Result<Vec<VertexObject>, String> {
    let mut vertex_objects = Vec::new();
    for primitive in mesh.primitives() {

        let reader = primitive.reader(|buf| Some(&buffers[buf.index()]));
    
        // Get positions and indices data
        let vertices = match reader.read_positions() {
            Some(positions) => positions.map(|p| Vec3::new(-p[2], -p[0], p[1])).collect::<Vec<_>>(),
            None => return Err("Mesh has no position data".to_string()),
        };
        let indices: Vec<u32> = match reader.read_indices() {
            Some(indices_reader) => indices_reader.into_u32().collect(),
            None => return Err("Mesh has no index data".to_string()),
        };
        console_log!("indices.len() {}", indices.len());

        // Get material properties
        let material_props = match get_material_properties_for_gltf(gltf, &primitive, buffers) {
            Ok(props) => props,
            Err(e) => {
                console_error!("Failed to get material properties: {}", e);
                return Err("Failed to get material properties".to_string());
            }
        };

        let vertex_object = VertexObject::new_from_indexed(&vertices, &indices, true, material_props);
        
        vertex_objects.push(vertex_object);
    }

    Ok(vertex_objects)
}

fn get_material_properties_for_gltf(gltf: &Gltf, primitive: &Primitive, buffers: &[Data]) -> Result<MaterialProperties, String> {
    let mut material_props = MaterialProperties::default();
    let pbr = primitive.material().pbr_metallic_roughness();

    if let Some(base_color_texture) = pbr.base_color_texture() {
        let texture_index = base_color_texture.texture().index();
        let image_index = base_color_texture.texture().source().index();

        // Get the reader for this primitive
        let reader = primitive.reader(|buf| Some(&buffers[buf.index()]));
        if let Some(tex_coords) = reader.read_tex_coords(0) {
            // Collect UV coordinates to find the region this primitive uses
            let tex_coords: Vec<[f32; 2]> = tex_coords.into_f32().collect();
            
            // Find the min and max UV coordinates to determine the region
            let mut min_u = 1.0 as f32;
            let mut min_v = 1.0 as f32;
            let mut max_u = 0.0 as f32;
            let mut max_v = 0.0 as f32;
            
            for uv in &tex_coords {
                min_u = min_u.min(uv[0]);
                min_v = min_v.min(uv[1]);
                max_u = max_u.max(uv[0]);
                max_v = max_v.max(uv[1]);
            }

            // console_log!("UV bounds: ({:.2}, {:.2}) - ({:.2}, {:.2})", min_u, min_v, max_u, max_v);

            // Get the image data
            if let Some(image) = gltf.images().nth(image_index) {
                if let gltf::image::Source::View { view, mime_type } = image.source() {
                    // Get buffer data for the image
                    let buffer = &buffers[view.buffer().index()];
                    let start = view.offset();
                    let end = start + view.length();
                    let image_data = &buffer[start..end];
                    
                    // Calculate average color for the specific UV region
                    match compute_average_color_for_uv_region(
                        image_data, mime_type, min_u, min_v, max_u, max_v
                    ) {
                        Ok(avg_color) => {
                            material_props.color = avg_color;
                            console_log!("UV region color: {:?}", avg_color);
                        },
                        Err(e) => {
                            console_log!("Failed to compute average color for UV region: {}", e);
                            return Err(format!("Failed to compute average color for UV region: {}", e));
                        }
                    }
                }
            } else {
                console_log!("Image not found for texture index: {}", texture_index);
                return Err("Image not found for texture index".to_string());
            }
        } else {
            console_log!("No texture coordinates found for primitive");
            return Err("No texture coordinates found for primitive".to_string());
        }

    } else { // use base_color_factor if no texture
        console_log!("No base color texture found, using base color factor instead.");
        let base_color = pbr.base_color_factor();
        let color = Vec3::new(base_color[0], base_color[1], base_color[2]);
        // let alpha = base_color[3];
        let alpha = 1.0; // Force alpha to 1.0 for now
        material_props = MaterialProperties {
            color: color,
            alpha,
            ..MaterialProperties::default()
        };
    }
    return Ok(material_props);
}

fn compute_average_color(image_data: &[u8], mime_type: &str) -> Result<Vec3, String> {
    // The mime_type parameter isn't needed when using the image crate
    // as it can automatically detect the format
    match load_from_memory(image_data) {
        Ok(img) => {
            let (width, height) = img.dimensions();
            let mut r_sum = 0;
            let mut g_sum = 0;
            let mut b_sum = 0;
            let total_pixels = width as usize * height as usize;
            
            // For large images, consider sampling instead of processing every pixel
            let sample_size = if total_pixels > 10000 { 10 } else { 1 };
            let mut sample_count = 0;
            
            for (x, y, pixel) in img.pixels() {
                // Only process every Nth pixel for large images
                if x % sample_size == 0 && y % sample_size == 0 {
                    r_sum += pixel[0] as u32;
                    g_sum += pixel[1] as u32;
                    b_sum += pixel[2] as u32;
                    sample_count += 1;
                }
            }
            
            if sample_count > 0 {
                Ok(Vec3::new(
                    r_sum as f32 / (255.0 * sample_count as f32),
                    g_sum as f32 / (255.0 * sample_count as f32),
                    b_sum as f32 / (255.0 * sample_count as f32)
                ))
            } else {
                Err("No pixels sampled to compute average color".to_string())
            }
        },
        Err(e) => {
            console_error!("Failed to decode image: {}", e);
            Err("Failed to decode image".to_string())
        }
    }
}

fn compute_average_color_for_uv_region(
    image_data: &[u8], 
    mime_type: &str, 
    min_u: f32, 
    min_v: f32, 
    max_u: f32, 
    max_v: f32
) -> Result<Vec3, String> {
    match load_from_memory(image_data) {
        Ok(img) => {
            let (width, height) = img.dimensions();
            let mut r_sum = 0;
            let mut g_sum = 0;
            let mut b_sum = 0;
            let mut pixel_count = 0;

            for y in 0..height {
                for x in 0..width {
                    let u = x as f32 / width as f32;
                    let v = y as f32 / height as f32;

                    if u >= min_u && u <= max_u && v >= min_v && v <= max_v {
                        let pixel = img.get_pixel(x, y);
                        r_sum += pixel[0] as u32;
                        g_sum += pixel[1] as u32;
                        b_sum += pixel[2] as u32;
                        pixel_count += 1;
                    }
                }
            }

            if pixel_count > 0 {
                Ok(Vec3::new(
                    r_sum as f32 / (255.0 * pixel_count as f32),
                    g_sum as f32 / (255.0 * pixel_count as f32),
                    b_sum as f32 / (255.0 * pixel_count as f32)
                ))
            } else {
                Ok(Vec3::new(0.0, 0.0, 0.0)) // Return black if no pixels found
                // Err("No pixels found in the specified UV region".to_string())
            }
        },
        Err(e) => {
            console_log!("Failed to decode image: {}", e);
            Err(format!("Failed to decode image: {}", e))
        }
    }
}



#[wasm_bindgen]
pub fn load_glb_model(glb_bytes: &[u8]) -> bool {
    match decode_glb_bytes(glb_bytes) {
        Ok((gltf, buffers)) => {
            match parse_gltf_objects(gltf, &buffers) {
                Ok(mut vertex_objects) => {

                    // TODO: remove later, this is for testing
                    for obj in vertex_objects.iter_mut() {
                        for vertex in obj.vertices.iter_mut() {
                            // vertex.rotate_z(PI / 2.0);
                            vertex.rotate_y(-PI / 2.0);
                            *vertex *= 0.2;
                            vertex.x += 10.0;
                            vertex.z += 5.0;
                        }
                    }

                    // Add objects to scene
                    GAME_INSTANCE.with(|game_instance| {
                        game_instance.borrow_mut().add_scene_objects(vertex_objects);
                    });
                    true
                },
                Err(e) => {
                    console_error!("GLTF parse error on parse_gltf_objects(): {}", e);
                    false
                }
            }
        },
        Err(e) => {
            console_error!("GLB parse error: {}", e);
            false
        }
    }
}

pub fn decode_glb_bytes(glb_bytes: &[u8]) -> Result<(Gltf, Vec<Data>), String> {
    // Parse the GLB data - this works with both GLB and GLTF formats
    let gltf = match Gltf::from_slice(glb_bytes) {
        Ok(gltf) => gltf,
        Err(e) => return Err(format!("Failed to parse GLB: {}", e)),
    };

    // console_log!("buffers().len(): {:?}", gltf.buffers().len());
    // console_log!("buffers(): {:?}", gltf.buffers());
    // console_log!("images().len(): {:?}", gltf.images().len());
    // console_log!("images(): {:?}", gltf.images());

    // console_log!("has blob: {:?}", gltf.blob.is_some());

    // Extract buffer data directly from GLB
    let mut buffers = Vec::new();
    if let Some(blob) = gltf.blob.as_ref() {
        buffers.push(Data::from_source_and_blob(Source::Bin, None, &mut Some(blob.clone())).unwrap());
    } else {
        return Err("GLB file missing binary chunk".to_string());
    }

    Ok((gltf, buffers))
}