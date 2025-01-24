use crate::{console_log, utils::math::Vec3};

use super::{buffers::ZBuffer, camera::Camera, scene::{MaterialProperties, SceneObject}};

pub struct Light {
    pub color: Vec3,
    pub intensity: f32,
    pub zbuf: ZBuffer,
    pub camera: Camera,
}

impl Light {
    pub fn new(camera: Camera, color: Vec3, intensity: f32, zbuf: ZBuffer) -> Light {
        return Light {
            color,
            intensity,
            zbuf,
            camera,
        }
    }

    pub fn clear_shadow_map(&mut self) {
        self.zbuf.clear();
    }

    pub fn add_objects_to_shadow_map(&mut self, objects: &Vec<Box<dyn SceneObject>>) {
        for obj in objects {
            self.add_vertices_to_shadow_map(obj.get_vertices());
        }
    }

    pub fn add_vertices_to_shadow_map(&mut self, vertices: &Vec<Vec3>) {
        for i in (0..vertices.len()).step_by(3) {
            self.add_triangle_to_shadow_map(vertices[i], vertices[i+1], vertices[i+2]);
        }
    }

    pub fn add_triangle_to_shadow_map(&mut self, mut v1: Vec3, mut v2: Vec3, mut v3: Vec3) {

        // do not render if normal is pointing toward light - FRONT FACE CULLING
        let normal = (&(v3 - v1)).cross(&(v2 - v1)).normalized();
        let cam_to_triangle = v1 - self.camera.pos;

        if normal.dot(&cam_to_triangle) < 0.0 {
            return;
        }

        // translate vertices to camera space
        self.camera.vertices_world_to_camera_space(&mut v1, &mut v2, &mut v3);

        // sort vertices by depth (v1 has lowest depth, v3 has highest)
        if v1.x > v2.x {
            std::mem::swap(&mut v1, &mut v2);
        }
        if v2.x > v3.x {
            std::mem::swap(&mut v2, &mut v3);
        }
        if v1.x > v2.x {
            std::mem::swap(&mut v1, &mut v2);
        }

        // CLIPPING VERTICES
        // Some basic clipping info (why is there so little info online): https://www.khronos.org/opengl/wiki/Vertex_Post-Processing
        const NEAR_PLANE: f32 = 0.001;
        if v1.x > 0.0 { // all vertices in view
            self.camera.vertices_camera_to_screen_space(&mut v1, &mut v2, &mut v3);
            self.fill_triangle(v1, v2, v3);
        } else if v2.x > 0.0 { // 2 vertices in view
            let q = (NEAR_PLANE - v2.x) / (v1.x - v2.x);
            let mut v1_new_1 = v2 + (v1 - v2) * q;
            let q = (NEAR_PLANE - v3.x) / (v1.x - v3.x);
            let mut v1_new_2 = v3 + (v1 - v3) * q;

            self.camera.vertices_camera_to_screen_space(&mut v1_new_1, &mut v2, &mut v3);
            self.camera.vertex_camera_to_screen_space(&mut v1_new_2);
            self.fill_triangle(v1_new_1, v2, v3);
            self.fill_triangle(v1_new_1, v1_new_2, v3);
        } else if v3.x > 0.0 { // 1 vertex in view
            let q = (NEAR_PLANE - v2.x) / (v3.x - v2.x);
            let mut v2_new = v2 + (v3 - v2) * q;
            let q = (NEAR_PLANE - v1.x) / (v3.x - v1.x);
            let mut v1_new = v1 + (v3 - v1) * q;

            self.camera.vertices_camera_to_screen_space(&mut v1_new, &mut v2_new, &mut v3);
            self.fill_triangle(v1_new, v2_new, v3);
        } else { // no vertices in view
            return;
        }
    }

    pub fn fill_triangle(&mut self, mut v1: Vec3, mut v2: Vec3, mut v3: Vec3) {
        // depth calculations from https://www.scratchapixel.com/lessons/3d-basic-rendering/rasterization-practical-implementation/visibility-problem-depth-buffer-depth-interpolation.html#:~:text=As%20previously%20mentioned%2C%20the%20correct,z%20%3D%201%20V%200.

        // sort vertices by y (v1 has lowest y, v3 has highest y)
        if v1.y > v2.y {
            std::mem::swap(&mut v1, &mut v2);
        }
        if v2.y > v3.y {
            std::mem::swap(&mut v2, &mut v3);
        }
        if v1.y > v2.y {
            std::mem::swap(&mut v1, &mut v2);
        }

        let width = self.zbuf.width as f32;
        let height = self.zbuf.height as f32;

        // calculate slopes
        let slope1 = (v2.x - v1.x) / (v2.y - v1.y); // slope of line from v1 to v2
        let slope2 = (v3.x - v1.x) / (v3.y - v1.y); // slope of line from v1 to v3
        let slope3 = (v3.x - v2.x) / (v3.y - v2.y); // slope of line from v2 to v3

        if v1.y == v3.y { // triangle has no height
            return;
        }

        // calculate starting and ending x values
        let top = v1.y.ceil().max(0.0);
        let mut x1 = slope1 * (top - v1.y) + v1.x;
        let mut x2 = slope2 * (top - v1.y) + v1.x;
        let bottom = v2.y.floor().min(height - 1.0);

        // fill top half
        if v1.y != v2.y {
            for y in (top as usize)..=(bottom as usize) {
                let left;
                let right;
                if x1 < x2 {
                    left = x1.ceil().max(0.0) as usize;
                    right = x2.floor().min(width-1.0) as usize;
                } else {
                    left = x2.ceil().max(0.0) as usize;
                    right = x1.floor().min(width-1.0) as usize;
                }
                
                let q1 = (y as f32 - v1.y) / (v2.y - v1.y);
                let inv_left_depth = (1.0 / v1.z) * (1.0 - q1) + (1.0 / v2.z) * q1;

                let q2 = (y as f32 - v1.y) / (v3.y - v1.y);
                let inv_right_depth = (1.0 / v1.z) * (1.0 - q2) + (1.0 / v3.z) * q2;

                for x in left..=right {

                    let q3 = (x as f32 - x1) / (x2 - x1);
                    let inv_depth = inv_left_depth * (1.0 - q3) + inv_right_depth * q3;
                    let depth = 1.0 / inv_depth;

                    if depth < self.zbuf.get_depth(x, y) {
                        self.zbuf.set_depth(x, y, depth);
                    }
                }
                x1 += slope1;
                x2 += slope2;
            }
        }


        // calculate starting and ending x values (for bottom half)
        let top = v2.y.ceil().max(0.0);
        let mut x1 = slope3 * (top - v2.y) + v2.x;
        let mut x2 = slope2 * (top - v1.y) + v1.x;
        let bottom = v3.y.floor().min(height - 1.0);

        // fill bottom half
        if v2.y != v3.y {
            for y in (top as usize)..=(bottom as usize) {
                let left;
                let right;
                if x1 < x2 {
                    left = x1.ceil().max(0.0) as usize;
                    right = x2.floor().min(width-1.0) as usize;
                } else {
                    left = x2.ceil().max(0.0) as usize;
                    right = x1.floor().min(width-1.0) as usize;
                }
                
                let q1 = (y as f32 - v2.y) / (v3.y - v2.y);
                let inv_left_depth = (1.0 / v2.z) * (1.0 - q1) + (1.0 / v3.z) * q1;

                let q2 = (y as f32 - v1.y) / (v3.y - v1.y);
                let inv_right_depth = (1.0 / v1.z) * (1.0 - q2) + (1.0 / v3.z) * q2;

                for x in left..=right {

                    let q3 = (x as f32 - x1) / (x2 - x1);
                    let inv_depth = inv_left_depth * (1.0 - q3) + inv_right_depth * q3;
                    let depth = 1.0 / inv_depth;

                    if depth < self.zbuf.get_depth(x, y) {
                        self.zbuf.set_depth(x, y, depth);
                    }
                }
                x1 += slope3;
                x2 += slope2;
            }
        }
    }

    pub fn get_lighting_at(&self, world_pos: &Vec3, observer_camera_pos: &Vec3, normal: &Vec3, properties: &MaterialProperties) -> Vec3 {
        
        // compute pixel-to-light vector and normalize
        let pixel_to_light = (self.camera.pos - *world_pos).normalized();

        // transform world position to camera space
        let mut v = world_pos.clone();
        self.camera.vertex_world_to_camera_space(&mut v);
        // console_log!("v is: {:?}", v);
        // console_log!("camera.theta_y: {}", self.camera.theta_y);

        let depth = v.x;
        if depth <= 0.0 { // pixel is behind the camera/light
            return Vec3::new(0.0, 0.0, 0.0);
        }

        let inv_dist = 1.0 / v.len();
        // console_log!("inv_dist: {}", inv_dist);

        // transform world position to screen space
        self.camera.vertex_camera_to_screen_space(&mut v);

        let mut proportion_in_light = 0.0;
        let mut samples = 0;
        let filter_radius = 1;
        let bias = 0.01;

        for dy in -filter_radius..=filter_radius {
            for dx in -filter_radius..=filter_radius {
                let sample_x = v.x.round() as i32 + dx;
                let sample_y = v.y.round() as i32 + dy;

                if sample_x < 0 || sample_x >= self.zbuf.width as i32 || sample_y < 0 || sample_y >= self.zbuf.height as i32 {
                    continue;
                }

                if depth + bias < self.zbuf.get_depth(sample_x as usize, sample_y as usize) {
                    proportion_in_light += 1.0;
                }
                samples += 1;
            }
        }
        proportion_in_light /= samples as f32;
        // console_log!("proportion_in_shadow: {}", proportion_in_shadow);

        // compute lighting components
        let angle_multiplier = pixel_to_light.dot(normal);
        if angle_multiplier <= 0.0 || proportion_in_light == 0.0 || samples == 0 {
            return Vec3::new(0.0, 0.0, 0.0) // light is behind or parallel to the surface
        }

        let diffuse_light = properties.diffuse * angle_multiplier * inv_dist * self.intensity * proportion_in_light * Vec3::pairwise_mul_new(&properties.color, &self.color);
        let mut specular_light = Vec3::new(0.0, 0.0, 0.0);

        if properties.specular > 0.0 {
            // BLINN PHONG MODEL
            // https://en.wikipedia.org/wiki/Phong_reflection_model
            // https://en.wikipedia.org/wiki/Blinn%E2%80%93Phong_reflection_model
            let V = (*observer_camera_pos - *world_pos).normalized();
            let H = (V + pixel_to_light).normalized();

            let NdotH = normal.dot(&H);
            let exp_multiplier = 4;
            if NdotH >= 0.0 {
                specular_light = properties.specular * NdotH.powi(exp_multiplier * properties.shininess) * self.intensity * inv_dist * proportion_in_light * Vec3::pairwise_mul_new(&properties.color, &self.color);
            }
        }
        return diffuse_light + specular_light;
    }
}