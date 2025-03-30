use rand::{distr::uniform::SampleRange, rngs::ThreadRng, Rng};

use crate::graphics::scene::SceneObject;

use super::math::Vec3;

// WEB-SYS / JS BASED UTILITIES
pub fn get_time() -> f64 {
    return web_sys::window()
        .expect("No global window exists")
        .performance()
        .expect("Window doesn't have Performance")
        .now()
}

// RAND UTILITIES
pub fn random_float() -> f32 {
    // let mut rng = rand::rng();
    // return rng.random();
    return rand::random::<f32>();
}
pub fn random_range(min: f32, max: f32) -> f32 {
    return random_float() * (max - min) + min;
}
/// Samples points in a square in the range [-0.5, 0.5].
/// Returns Vec3([-0.5, 0.5], [-0.5, 0.5], 0)
pub fn sample_square() -> Vec3 {
    let x = random_float();
    let y = random_float();
    return Vec3::new(x - 0.5, y - 0.5, 0.0);
}

// OTHER UTILITIES
pub fn color_to_u8(color: &Vec3) -> (u8, u8, u8) {
    (
        (color.x.clamp(0.0, 1.0) * 255.0) as u8,
        (color.y.clamp(0.0, 1.0) * 255.0) as u8,
        (color.z.clamp(0.0, 1.0) * 255.0) as u8,
    )
}

pub fn gamma_correct_color(color: &Vec3) -> Vec3 {
    Vec3::new(
        color.x.max(0.0).sqrt(),
        color.y.max(0.0).sqrt(),
        color.z.max(0.0).sqrt(),
    )
}

pub fn sort_objects_by_distance_to_camera(objects: &mut Vec<Box<dyn SceneObject>>, camera_pos: &Vec3) {
    objects.sort_by(|a, b| 
        (a.get_center() - *camera_pos).len_squared()
            .partial_cmp(&(b.get_center() - *camera_pos).len_squared())
            .unwrap_or(std::cmp::Ordering::Equal)
    );
}

pub fn flip_indices_winding(indices: &mut Vec<usize>) {
    for i in (0..indices.len()).step_by(3) {
        (indices[i], indices[i + 2]) = (indices[i + 2], indices[i]);
    }
}