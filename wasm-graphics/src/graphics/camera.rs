use std::f32::consts::PI;

use crate::{console_log, utils::math::Vec3};

#[derive(Debug)]
pub struct Camera {
    pub pos: Vec3,
    theta_y: f32,
    theta_z: f32,
    looking_dir: Vec3,
    fov: f32,

    pub sin_theta_y: f32,
    pub cos_theta_y: f32,
    pub sin_theta_z: f32,
    pub cos_theta_z: f32,

    pub max_plane_coord: f32,

    pub width: usize,
    pub height: usize,
}

impl Camera {
    pub fn new(pos: Vec3, theta_y: f32, theta_z: f32, fov: f32, width: usize, height: usize) -> Camera {
        let (sin_theta_y, cos_theta_y) = theta_y.sin_cos();
        let (sin_theta_z, cos_theta_z) = theta_z.sin_cos();
        let looking_dir = Vec3::new(
            cos_theta_y * cos_theta_z,
            cos_theta_y * sin_theta_z,
            sin_theta_y,
        );
        let max_plane_coord = f32::tan(0.5 * fov);

        return Camera {
            pos,
            theta_y,
            theta_z,
            looking_dir,
            fov,
            sin_theta_y,
            cos_theta_y,
            sin_theta_z,
            cos_theta_z,
            max_plane_coord,
            width,
            height,
        }
    }

    pub fn get_theta_y(&self) -> f32 {
        return self.theta_y;
    }
    pub fn get_theta_z(&self) -> f32 {
        return self.theta_z;
    }
    pub fn set_theta_y(&mut self, theta_y: f32) {
        self.theta_y = theta_y;
        self.theta_y = self.theta_y.clamp(-0.5 * PI, 0.5 * PI);
        (self.sin_theta_y, self.cos_theta_y) = self.theta_y.sin_cos();

        self.looking_dir = Vec3::new(
            self.cos_theta_y * self.cos_theta_z,
            self.cos_theta_y * self.sin_theta_z,
            self.sin_theta_y,
        );
    }
    pub fn set_theta_z(&mut self, theta_z: f32) {
        self.theta_z = theta_z;
        (self.sin_theta_z, self.cos_theta_z) = theta_z.sin_cos();

        self.looking_dir = Vec3::new(
            self.cos_theta_y * self.cos_theta_z,
            self.cos_theta_y * self.sin_theta_z,
            self.sin_theta_y,
        );
    }
    pub fn get_looking_dir(&self) -> &Vec3 {
        return &self.looking_dir;
    }
    pub fn get_fov(&self) -> f32 {
        return self.fov;
    }
    pub fn set_fov(&mut self, fov: f32) {
        self.fov = fov;
        self.max_plane_coord = f32::tan(0.5 * fov);
    }

    pub fn look_in_direction(&mut self, dir: &Vec3) {
        self.set_theta_y((dir.z / dir.len()).asin());
        self.set_theta_z((dir.y).atan2(dir.x));
    }
    pub fn look_at(&mut self, at: &Vec3) {
        let dir = *at - self.pos;
        self.look_in_direction(&dir);
    }

    pub fn vertex_world_to_camera_space(&self, v: &mut Vec3) {
        *v -= self.pos;
        // v.rotate_z(-self.theta_z);
        // v.rotate_y(-self.theta_y);
        v.rotate_z_fast(-self.sin_theta_z, self.cos_theta_z);
        v.rotate_y_fast(-self.sin_theta_y, self.cos_theta_y);
    }
    pub fn three_vertices_world_to_camera_space(&self, v1: &mut Vec3, v2: &mut Vec3, v3: &mut Vec3) {
        self.vertex_world_to_camera_space(v1);
        self.vertex_world_to_camera_space(v2);
        self.vertex_world_to_camera_space(v3);
    }
    pub fn vertices_world_to_camera_space(&self, vertices: &Vec<Vec3>) -> Vec<Vec3> {
        let mut transformed_vertices = vertices.clone();
        for v in transformed_vertices.iter_mut() {
            self.vertex_world_to_camera_space(v);
        }
        return transformed_vertices;
    }
    pub fn vertex_camera_to_screen_space(&self, v: &mut Vec3) {
        let depth = v.x;
        v.x = v.y / depth;
        v.y = v.z / depth;
        v.z = depth;

        // let max_plane_coord = f32::tan(0.5 * self.fov);
        v.x = (0.5 * self.width as f32) * (1.0 - v.x / self.max_plane_coord);
        v.y = 0.5 * (self.height as f32 - v.y / self.max_plane_coord * self.width as f32);
    }
    pub fn vertices_camera_to_screen_space(&self, v1: &mut Vec3, v2: &mut Vec3, v3: &mut Vec3) {
        self.vertex_camera_to_screen_space(v1);
        self.vertex_camera_to_screen_space(v2);
        self.vertex_camera_to_screen_space(v3);
    }

    pub fn vertex_screen_to_camera_space(&self, v: &mut Vec3) {
        let depth = v.z;
        // let max_plane_coord = f32::tan(0.5 * self.fov);
        v.z = -((v.y * 2.0 - self.height as f32) / self.width as f32 * self.max_plane_coord);
        v.y = -(v.x * 2.0 / self.width as f32 - 1.0) * self.max_plane_coord;
        v.x = 1.0;
        *v *= depth;
    }
    pub fn vertex_camera_to_world_space(&self, v: &mut Vec3) {
        // v.rotate_y(self.theta_y);
        // v.rotate_z(self.theta_z);
        v.rotate_y_fast(self.sin_theta_y, self.cos_theta_y);
        v.rotate_z_fast(self.sin_theta_z, self.cos_theta_z);
        *v += self.pos;
    }
    pub fn vertex_screen_to_world_space(&self, v: &mut Vec3) {
        self.vertex_screen_to_camera_space(v);
        self.vertex_camera_to_world_space(v);
    }
}