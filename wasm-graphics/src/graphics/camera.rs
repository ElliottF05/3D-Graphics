use crate::utils::math::Vec3;

pub struct Camera {
    pub pos: Vec3,
    pub theta_y: f32,
    pub theta_z: f32,
    pub fov: f32,
}