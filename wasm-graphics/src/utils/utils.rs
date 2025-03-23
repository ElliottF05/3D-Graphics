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

// OTHER UTILITIES
pub fn color_to_u8(color: &Vec3) -> (u8, u8, u8) {
    (
        (color.x.clamp(0.0, 1.0) * 255.0) as u8,
        (color.y.clamp(0.0, 1.0) * 255.0) as u8,
        (color.z.clamp(0.0, 1.0) * 255.0) as u8,
    )
}

pub fn sort_objects_by_distance_to_camera(objects: &mut Vec<Box<dyn SceneObject>>, camera_pos: &Vec3) {
    objects.sort_by(|a, b| 
        (a.get_center() - *camera_pos).len_squared()
            .partial_cmp(&(b.get_center() - *camera_pos).len_squared())
            .unwrap_or(std::cmp::Ordering::Equal)
    );
}