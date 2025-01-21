use crate::utils::math::Vec3;

pub struct Camera {
    pub pos: Vec3,
    pub theta_y: f32,
    pub theta_z: f32,
    pub fov: f32,

    pub width: usize,
    pub height: usize,
}

impl Camera {
    pub fn new(pos: Vec3, theta_y: f32, theta_z: f32, fov: f32, width: usize, height: usize) -> Camera {
        return Camera {
            pos,
            theta_y,
            theta_z,
            fov,
            width,
            height,
        }
    }

    pub fn vertex_world_to_camera_space(&self, v: &mut Vec3) {
        *v -= self.pos;
        (*v).rotate_z(-self.theta_z);
        (*v).rotate_y(-self.theta_y);
    }
    pub fn vertices_world_to_camera_space(&self, v1: &mut Vec3, v2: &mut Vec3, v3: &mut Vec3) {
        self.vertex_world_to_camera_space(v1);
        self.vertex_world_to_camera_space(v2);
        self.vertex_world_to_camera_space(v3);
    }
    pub fn vertex_camera_to_screen_space(&self, v: &mut Vec3) {
        let depth = v.x;
        v.x = v.y / depth;
        v.y = v.z / depth;
        v.z = depth;

        let max_plane_coord = f32::tan(0.5 * self.fov);
        v.x = (0.5 * self.width as f32) * (1.0 - v.x / max_plane_coord);
        v.y = 0.5 * (self.height as f32 - v.y / max_plane_coord * self.width as f32);
    }
    pub fn vertices_camera_to_screen_space(&self, v1: &mut Vec3, v2: &mut Vec3, v3: &mut Vec3) {
        self.vertex_camera_to_screen_space(v1);
        self.vertex_camera_to_screen_space(v2);
        self.vertex_camera_to_screen_space(v3);
    }
}