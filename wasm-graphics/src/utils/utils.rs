use rand::{distr::{uniform::SampleRange, Uniform}, rngs::ThreadRng, Rng};

use crate::graphics::mesh::Mesh;

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
#[inline(always)]
pub fn random_float() -> f32 {
    // let mut rng = rand::rng();
    // return rng.random();
    return rand::random::<f32>();
}
#[inline(always)]
pub fn random_range(min: f32, max: f32) -> f32 {
    return random_float() * (max - min) + min;
}
/// Samples uniformly from the range [low, high]
pub fn random_int(min: i32, max: i32) -> i32 {
    let distr = Uniform::new_inclusive(min, max).expect("rand::distr::Uniform::new_inclusive failed");
    let mut rng = rand::rng();
    return rng.sample(distr);
}
/// Samples points in a square in the range [-0.5, 0.5].
#[inline(always)]
pub fn sample_square(side_length: f32) -> (f32, f32) {
    let x = random_float();
    let y = random_float();
    return ((x - 0.5) * side_length, (y - 0.5) * side_length);
}

/// Returns a random x,y coordinate in the unit circle, with even distribution
#[inline(always)]
pub fn sample_circle(radius: f32) -> (f32 ,f32) {
    loop {
        let x = random_range(-1.0, 1.0);
        let y = random_range(-1.0, 1.0);
        if x*x + y*y <= 1.0 {
            return (radius * x, radius * y);
        }
    }
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

pub fn sort_meshes_by_distance_to_camera(meshes: &mut Vec<Mesh>, camera_pos: &Vec3) {
    meshes.sort_by(|a, b| {
        let d1 = (a.center - *camera_pos).len_squared();
        let d2 = (b.center - *camera_pos).len_squared();
        return d1.total_cmp(&d2);
    });
}

pub fn flip_indices_winding(indices: &mut Vec<usize>) {
    for i in (0..indices.len()).step_by(3) {
        (indices[i], indices[i + 2]) = (indices[i + 2], indices[i]);
    }
}