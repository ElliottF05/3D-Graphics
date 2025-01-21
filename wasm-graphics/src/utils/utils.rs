use super::math::Vec3;

pub fn color_to_u8(color: Vec3) -> (u8, u8, u8) {
    return (
        (color.x * 255.0) as u8,
        (color.y * 255.0) as u8,
        (color.z * 255.0) as u8,
    )
}