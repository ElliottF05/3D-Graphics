use gltf::{buffer::{Data, Source}, image, json::extensions::material, mesh::{util::{tex_coords, ReadColors}, Reader}, scene, Gltf, Primitive};
use ::image::{load_from_memory, GenericImageView};
use wasm_bindgen::prelude::*;
use data_url;
use crate::{console_error, console_log, utils::utils::flip_indices_winding, wasm::wasm::GAME_INSTANCE};

use crate::utils::math::Vec3;

use super::{mesh::{Mesh, PhongProperties}, ray_tracing::material::{Lambertian, Material}, scene_object::SceneObject};

pub fn extract_combined_mesh_from_raw_glb_bytes(glb_bytes: &[u8]) -> Result<Mesh, String> {
    match decode_glb_bytes(glb_bytes) {
        Ok((gltf, buffers)) => {
            let combined_mesh = extract_combined_mesh_from_gltf(&gltf, &buffers)?;
            Ok(combined_mesh)
        },
        Err(e) => {
            console_error!("GLB error on decode_glb_bytes(): {}", e);
            Err(format!("GLB error on decode_glb_bytes(): {}", e))
        }
    }
}

#[wasm_bindgen]
pub fn load_glb_model(glb_bytes: &[u8]) -> bool {
    match decode_glb_bytes(glb_bytes) {
        Ok((gltf, buffers)) => {

            let mut combined_mesh = match extract_combined_mesh_from_gltf(&gltf, &buffers) {
                Ok(mesh) => mesh,
                Err(e) => {
                    console_error!("GLTF parse error on extract_combined_mesh_from_gltf(): {}", e);
                    return false;
                }
            };

            // center the mesh
            combined_mesh.translate_to(Vec3::new(0.0, 0.0, 0.0));
            if combined_mesh.radius > 50.0 {
                let scale_factor = 50.0 / combined_mesh.radius;
                combined_mesh.scale_by(scale_factor);
            }

            let combined_scene_obj = SceneObject::new_from_mesh(combined_mesh, Lambertian::default().clone_box(), false);
            GAME_INSTANCE.with(|game_instance| {
                let mut g = game_instance.borrow_mut();
                g.scene_objects.write().unwrap().push(combined_scene_obj);
                g.bvh = None; // invalidate the bvh
            });
            true
        },
        Err(e) => {
            console_error!("GLB error on decode_glb_bytes(): {}", e);
            false
        }
    }
}

pub fn decode_glb_bytes(glb_bytes: &[u8]) -> Result<(Gltf, Vec<Data>), String> {
    // Parse the GLB data - this works with both GLB and GLTF formats
    let gltf = match Gltf::from_slice(glb_bytes) {
        Ok(gltf) => gltf,
        Err(e) => {
            console_error!("Failed to parse GLB from slice: {}", e);
            return Err(format!("Failed to parse GLB from slice: {}", e));
        },
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
        console_error!("GLB file missing binary chunk");
        return Err("GLB file missing binary chunk".to_string());
    }

    Ok((gltf, buffers))
}

pub fn extract_combined_mesh_from_gltf(gltf: &Gltf, buffers: &[Data]) -> Result<Mesh, String> {
    match parse_gltf_objects(&gltf, &buffers) {
        Ok(meshes) => {

            // combine meshes into a one mesh
            let combined_vertices: Vec<Vec3> = meshes.iter().flat_map(|m| m.vertices.clone()).collect();
            let combined_colors: Vec<Vec3> = meshes.iter().flat_map(|m| m.colors.clone()).collect();
            let mut combined_indices = Vec::new();

            let mut vertex_offset = 0;
            for mesh in meshes {
                combined_indices.extend(mesh.indices.iter().map(|i| i + vertex_offset));
                vertex_offset += mesh.vertices.len();
            }

            // log lengths of buffers
            console_log!("combined_vertices.len(): {:?}", combined_vertices.len());
            console_log!("combined_indices.len(): {:?}", combined_indices.len());
            console_log!("combined_colors.len(): {:?}", combined_colors.len());

            let combined_mesh = Mesh::new(combined_vertices, combined_indices, combined_colors, PhongProperties::rt_default());
            console_log!("Extracted mesh from GLTF with {} vertices and {} faces", 
                combined_mesh.vertices.len(),  
                combined_mesh.colors.len());
            return Ok(combined_mesh);
        },
        Err(e) => {
            console_error!("GLTF parse error on parse_gltf_objects(): {}", e);
            return Err(format!("GLTF parse error on parse_gltf_objects(): {}", e))
        }
    }
}

pub fn parse_gltf_objects(gltf: &Gltf, buffers: &[Data]) -> Result<Vec<Mesh>, String> {
    
    let mut meshes = Vec::new();

    for mesh in gltf.meshes() {
        let mesh_objects = parse_gltf_mesh(&gltf, mesh, buffers)?;
        meshes.extend(mesh_objects);
    }

    Ok(meshes)
}

fn parse_gltf_mesh(gltf: &Gltf, mesh: gltf::Mesh, buffers: &[Data]) -> Result<Vec<Mesh>, String> {
    let mut vertex_objects = Vec::new();
    for primitive in mesh.primitives() {

        let reader = primitive.reader(|buf| Some(&buffers[buf.index()]));
    
        // Get positions and indices data
        let vertices = match reader.read_positions() {
            Some(positions) => positions.map(|p| Vec3::new(-p[2], -p[0], p[1])).collect::<Vec<_>>(),
            None => return Err("Mesh has no position data".to_string()),
        };
        let mut indices: Vec<usize> = match reader.read_indices() {
            Some(indices_reader) => indices_reader.into_u32().map(|x| x as usize).collect(),
            None => return Err("Mesh has no index data".to_string()),
        };
        flip_indices_winding(&mut indices);

        // Get material properties
        let phong_properties = PhongProperties::default();
        let pbr = primitive.material().pbr_metallic_roughness();
        let pbr_base_color = pbr.base_color_factor();

        let base_color = Vec3::new(pbr_base_color[0], pbr_base_color[1], pbr_base_color[2]);
        let read_colors = reader.read_colors(0);

        // get_colors_from_vertex_colors() is unused, models often have meaningless vertex colors

        let colors =  {
            // Try to get colors from texture
            let tex_coords_vec = match reader.read_tex_coords(0) {
                Some(tc) => tc.into_f32().collect(),
                None => vec![] // Empty if no texture coords
            };

            let indices_for_texture = match reader.read_indices() {
                Some(idx) => idx.into_u32().collect(),
                None => vec![] // Empty if no indices
            };

            if !tex_coords_vec.is_empty() && !indices_for_texture.is_empty() {
                match get_colors_from_texture(gltf, &primitive, tex_coords_vec, indices_for_texture, buffers) {
                    Ok(texture_colors) => {
                        console_log!("using texture colors");
                        texture_colors
                    },
                    Err(_) => {
                        console_log!("Failed to get colors from texture, using base color");
                        vec![base_color; indices.len() / 3]
                    }
                }
            } else {
                console_log!("No texture coordinates or indices found, using base color");
                vec![base_color; indices.len() / 3]
            }
        };
        // console_log!("colors: {:?}", colors);

        let vertex_object = Mesh::new(vertices, indices, colors, phong_properties);
        
        vertex_objects.push(vertex_object);
    }

    Ok(vertex_objects)
}

fn get_colors_from_texture(
    gltf: &Gltf,
    primitive: &gltf::Primitive,
    tex_coords_vec: Vec<[f32; 2]>,
    indices: Vec<u32>,
    buffers: &[Data]
) -> Result<Vec<Vec3>, String> {

    // Get the material and check if it has a base color texture
    let material = primitive.material();
    let pbr = material.pbr_metallic_roughness();
    
    if let Some(texture) = pbr.base_color_texture() {
        // Get the texture and its source image
        let texture_info = texture.texture();
        let image = texture_info.source();
        
        // Get the image data
        let image_data = match get_image_data(gltf, &image, buffers) {
            Ok(data) => data,
            Err(e) => {
                console_log!("Failed to get image data for texture: {}", e);
                return Err(format!("Failed to get image data for texture: {}", e));
            },
        };

        // Cache the decoded image to avoid repeatedly decoding it
        let decoded_image = match load_from_memory(&image_data) {
            Ok(img) => img,
            Err(e) => {
                console_log!("Failed to decode the received texture image: {}", e);
                return Err(format!("Failed to decode the received texture image: {}", e));
            },
        };
        let (width, height) = decoded_image.dimensions();
        
        // For each triangle, sample the texture at the first vertex
        let mut colors = Vec::new();
        
        for chunk in indices.chunks(3) {
            if chunk.len() == 3 {
                // Get the UV coordinates of the first vertex of the triangle
                let vertex_idx = chunk[0] as usize;
                if vertex_idx < tex_coords_vec.len() {
                    let uv = tex_coords_vec[vertex_idx];

                    // Calculate pixel coordinates from UV (handle wrapping)
                    let mut x = (uv[0] * width as f32) as u32;
                    let mut y = (uv[1] * height as f32) as u32;
                    
                    x = x % width;
                    y = y % height;

                    // Get the pixel color
                    let pixel = decoded_image.get_pixel(x, y);
                    colors.push(Vec3::new(
                        pixel[0] as f32 / 255.0,
                        pixel[1] as f32 / 255.0,
                        pixel[2] as f32 / 255.0
                    ));
                } else {
                    colors.push(Vec3::new(1.0, 0.0, 1.0)); // Fallback to magenta
                }
            }
        }
        
        Ok(colors)
    } else {
        // No base color texture, return an error
        Err("No base color texture found".to_string())
    }
}

// Helper function to get image data from a GLTF image
fn get_image_data(gltf: &Gltf, image: &gltf::Image, buffers: &[Data]) -> Result<Vec<u8>, String> {
    match image.source() {
        gltf::image::Source::View { view, mime_type: _ } => {
            let buffer = &buffers[view.buffer().index()];
            let begin = view.offset();
            let end = begin + view.length();
            Ok(buffer[begin..end].to_vec())
        },
        gltf::image::Source::Uri { uri, mime_type: _ } => {
            // For embedded base64 data URIs
            if uri.starts_with("data:") {
                match data_url::DataUrl::process(uri) {
                    Ok(data_url) => {
                        let (body, _) = data_url.decode_to_vec().unwrap();
                        Ok(body)
                    },
                    Err(_) => Err("Failed to parse data URL".to_string()),
                }
            } else {
                // External URIs would require network requests
                console_log!("GLB tried to reference external image URIs, not supported yet");
                Err("External image URIs not supported yet".to_string())
            }
        }
    }
}

fn get_colors_from_vertex_colors(read_colors: Option<ReadColors>) -> Result<Vec<Vec3>, String> {
    if read_colors.is_none() {
        console_log!("No vertex colors found");
        return Err("No vertex colors found".to_string());
    }
    match read_colors.unwrap() {
        ReadColors::RgbU8(iter) => {
            let colors = iter.map(|rgb| Vec3::new(
                rgb[0] as f32 / 255.0,
                rgb[1] as f32 / 255.0,
                rgb[2] as f32 / 255.0,
            )).collect();
            Ok(colors)
        },
        ReadColors::RgbU16(iter) => {
            let colors = iter.map(|rgb| Vec3::new(
                rgb[0] as f32 / 65535.0,
                rgb[1] as f32 / 65535.0,
                rgb[2] as f32 / 65535.0,
            )).collect();
            Ok(colors)
        },
        ReadColors::RgbF32(iter) => {
            let colors = iter.map(|rgb| Vec3::new(
                rgb[0], rgb[1], rgb[2]
            )).collect();
            Ok(colors)
        },
        ReadColors::RgbaU8(iter) => {
            let colors = iter.map(|rgba| Vec3::new(
                rgba[0] as f32 / 255.0,
                rgba[1] as f32 / 255.0,
                rgba[2] as f32 / 255.0,
            )).collect();
            Ok(colors)
        },
        ReadColors::RgbaU16(iter) => {
            let colors = iter.map(|rgba| Vec3::new(
                rgba[0] as f32 / 65535.0,
                rgba[1] as f32 / 65535.0,
                rgba[2] as f32 / 65535.0,
            )).collect();
            Ok(colors)
        },
        ReadColors::RgbaF32(iter) => {
            let colors = iter.map(|rgba| Vec3::new(
                rgba[0], rgba[1], rgba[2]
            )).collect();
            Ok(colors)
        },
    }
}

fn sample_texture_at_uv(
    image_data: &[u8], 
    u: f32, 
    v: f32
) -> Result<Vec3, String> {
    match load_from_memory(image_data) {
        Ok(img) => {
            let (width, height) = img.dimensions();
            
            // Calculate pixel coordinates from UV (handle wrapping)
            let mut x = (u * width as f32) as u32;
            let mut y = (v * height as f32) as u32;
            
            // Ensure coordinates are within bounds (wrap around for texture repeating)
            x = x % width;
            y = y % height;
            
            // Get the pixel at the calculated position
            let pixel = img.get_pixel(x, y);
            
            // Return the color as Vec3 (normalized to 0.0-1.0)
            Ok(Vec3::new(
                pixel[0] as f32 / 255.0,
                pixel[1] as f32 / 255.0,
                pixel[2] as f32 / 255.0
            ))
        },
        Err(e) => {
            console_log!("Failed to decode image: {}", e);
            Err(format!("Failed to decode image: {}", e))
        }
    }
}

// fn parse_gltf_lights(gltf: &Gltf) -> Result<Vec<Light>, String> {
//     console_log!("Parsing lights");
//     // let lights = Vec::new();
//     for node in gltf.nodes() {
//         if let Some(gltf_light) = node.light() {
//             let position_floats = node.transform().decomposed().0;
//             let pos = Vec3::new(position_floats[0], position_floats[1], position_floats[2]);

//             let color_floats = gltf_light.color();
//             let color = Vec3::new(color_floats[0], color_floats[1], color_floats[2]);

//             let intensity = gltf_light.intensity();

//             match gltf_light.kind() {
//                 gltf::khr_lights_punctual::Kind::Directional => {
//                     console_log!("Found directional light");
//                 },
//                 gltf::khr_lights_punctual::Kind::Point => {
//                     console_log!("Found point light");
//                 },
//                 gltf::khr_lights_punctual::Kind::Spot{inner_cone_angle, outer_cone_angle} => {
//                     console_log!("Found spot light");
//                 },
//             }
//         } else {
//             console_log!("No light found");
//         }
//     }

//     return Err("Not implemented".to_string());
// }