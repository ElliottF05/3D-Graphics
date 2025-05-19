use std::f32::consts::PI;

use crate::{console_log, utils::{math::Vec3}};

use super::{buffers::{PixelBuf, ZBuffer}, camera::Camera, mesh::{Mesh, PhongProperties}, scene_object::SceneObject};


pub struct Light {
    pub color: Vec3,
    pub min_dist: f32,
    pub max_dist: f32,
    pub zbuf: ZBuffer,
    pub color_buf: PixelBuf,
    pub camera: Camera,
}

impl Light {
    // pub fn new(camera: Camera, color: Vec3, intensity: f32, zbuf: ZBuffer, color_buf: PixelBuf) -> Light {
    //     return Light {
    //         color,
    //         intensity,
    //         zbuf,
    //         camera,
    //         color_buf,
    //     }
    // }
    pub fn new(pos: Vec3, direction: Vec3, fov: f32, color: Vec3, min_dist: f32, buf_width: usize, buf_height: usize) -> Light {
        let mut cam = Camera::new(pos, 0.0, 0.0, fov, buf_width, buf_height);
        cam.look_in_direction(&direction);
        return Light {
            camera: cam,
            color,
            min_dist,
            max_dist: 1000.0,
            zbuf: ZBuffer::new(buf_width, buf_height),
            color_buf: PixelBuf::new(buf_width, buf_height)
        }
    }
    pub fn new_with_angle(pos: Vec3, theta_y: f32, theta_z: f32, fov: f32, color: Vec3, min_dist: f32, buf_width: usize, buf_height: usize) -> Light {
        let cam = Camera::new(pos, theta_y, theta_z, fov, buf_width, buf_height);
        return Light {
            camera: cam,
            color,
            min_dist,
            max_dist: 1000.0,
            zbuf: ZBuffer::new(buf_width, buf_height),
            color_buf: PixelBuf::new(buf_width, buf_height)
        }
    }
    pub fn new_looking_at(pos: Vec3, look_at: Vec3, fov: f32, color: Vec3, min_dist: f32, buf_width: usize, buf_height: usize) -> Light {
        let mut cam = Camera::new(pos, 0.0, 0.0, fov, buf_width, buf_height);
        cam.look_at(&look_at);
        return Light {
            camera: cam,
            color,
            min_dist,
            max_dist: 1000.0,
            zbuf: ZBuffer::new(buf_width, buf_height),
            color_buf: PixelBuf::new(buf_width, buf_height)
        }
    }
    pub fn new_omnidirectional(pos: Vec3, color: Vec3, min_dist: f32, buf_width: usize) -> Vec<Light> {
        let directions = vec![
            Vec3::new(1.0, 0.0, 0.0),
            Vec3::new(-1.0, 0.0, 0.0),
            Vec3::new(0.0, 1.0, 0.0),
            Vec3::new(0.0, -1.0, 0.0),
            Vec3::new(0.0, 0.0, 1.0),
            Vec3::new(0.0, 0.0, -1.0),
        ];
        return directions
            .iter()
            .map(|d| Light::new(pos, *d, PI/2.0, color, min_dist, buf_width, buf_width))
            .collect();
    }

    pub fn clear_shadow_map(&mut self) {
        self.zbuf.clear();
        self.color_buf.clear_to_white();
    }

    pub fn add_scene_object_to_shadow_map(&mut self, scene_obj: &SceneObject) {
        let mesh = &scene_obj.mesh;
        self.add_mesh_to_shadow_map(mesh);
    }
    pub fn add_scene_objects_to_shadow_map(&mut self, scene_objs: &Vec<SceneObject>) {
        for scene_obj in scene_objs {
            self.add_scene_object_to_shadow_map(scene_obj);
        }
    }

    pub fn add_mesh_to_shadow_map(&mut self, mesh: &Mesh) {
        // TODO: don't recalculate the shared vertices, take advantage of indexed data structure
        let vertices = &mesh.vertices;
        let indices = &mesh.indices;
        let colors = &mesh.colors;
        for i in 0..colors.len() {
            let v1 = vertices[indices[i*3]];
            let v2 = vertices[indices[i*3+1]];
            let v3 = vertices[indices[i*3+2]];
            let color = colors[i];
            self.add_triangle_to_shadow_map(v1, v2, v3, color, mesh);
        }
    }

    pub fn add_meshes_to_shadow_map(&mut self, meshes: &Vec<Mesh>) {
        for mesh in meshes {
            self.add_mesh_to_shadow_map(mesh);
        }
    }

    fn add_triangle_to_shadow_map(&mut self, mut v1: Vec3, mut v2: Vec3, mut v3: Vec3, color: Vec3, mesh: &Mesh) {

        // do not render if normal is pointing toward light - FRONT FACE CULLING
        let normal = (v3 - v1).cross(v2 - v1).normalized();
        let cam_to_triangle = v1 - self.camera.pos;

        if mesh.properties.cull_faces && normal.dot(cam_to_triangle) < 0.0 {
            return;
        }

        // translate vertices to camera space
        self.camera.three_vertices_world_to_camera_space(&mut v1, &mut v2, &mut v3);

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
            self.fill_triangle(v1, v2, v3, color, mesh);
        } else if v2.x > 0.0 { // 2 vertices in view
            let q = (NEAR_PLANE - v2.x) / (v1.x - v2.x);
            let mut v1_new_1 = v2 + (v1 - v2) * q;
            let q = (NEAR_PLANE - v3.x) / (v1.x - v3.x);
            let mut v1_new_2 = v3 + (v1 - v3) * q;

            self.camera.vertices_camera_to_screen_space(&mut v1_new_1, &mut v2, &mut v3);
            self.camera.vertex_camera_to_screen_space(&mut v1_new_2);
            self.fill_triangle(v1_new_1, v2, v3, color, mesh);
            self.fill_triangle(v1_new_1, v1_new_2, v3, color, mesh);
        } else if v3.x > 0.0 { // 1 vertex in view
            let q = (NEAR_PLANE - v2.x) / (v3.x - v2.x);
            let mut v2_new = v2 + (v3 - v2) * q;
            let q = (NEAR_PLANE - v1.x) / (v3.x - v1.x);
            let mut v1_new = v1 + (v3 - v1) * q;

            self.camera.vertices_camera_to_screen_space(&mut v1_new, &mut v2_new, &mut v3);
            self.fill_triangle(v1_new, v2_new, v3, color, mesh);
        } else { // no vertices in view
            return;
        }
    }

    fn fill_triangle(&mut self, mut v1: Vec3, mut v2: Vec3, mut v3: Vec3, color: Vec3, mesh: &Mesh) {
        // depth calculations from https://www.scratchapixel.com/lessons/3d-basic-rendering/rasterization-practical-implementation/visibility-problem-depth-buffer-depth-interpolation.html#:~:text=As%20previously%20mentioned%2C%20the%20correct,z%20%3D%201%20V%200.

        let properties = &mesh.properties;

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
        if v1.y != v2.y && bottom >= 0.0 {
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

                self.fill_triangle_scanline_row(y, x1, x2, left, right, inv_left_depth, inv_right_depth, color, *properties);

                // for x in left..=right {

                //     let q3 = (x as f32 - x1) / (x2 - x1);
                //     let inv_depth = inv_left_depth * (1.0 - q3) + inv_right_depth * q3;
                //     let depth = 1.0 / inv_depth;

                //     let mut world_pos = Vec3::new(x as f32, y as f32, depth);
                //     self.camera.vertex_screen_to_camera_space(&mut world_pos);
                //     if world_pos.len_squared() < self.min_dist * self.min_dist {
                //         continue;
                //     }

                //     if depth < self.zbuf.get_depth(x, y) {
                //         if properties.alpha == 1.0 {
                //             self.zbuf.set_depth(x, y, depth);
                //         } else {
                //             let old_color = self.color_buf.get_pixel(x, y);
                //             let new_color = (1.0 - properties.alpha) * (properties.alpha * Vec3::mul_elementwise_of(old_color, color) + (1.0 - properties.alpha) * old_color);
                //             self.color_buf.set_pixel(x, y, new_color);
                //         }
                //     }
                // }
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
        if v2.y != v3.y && bottom >= 0.0 {
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

                self.fill_triangle_scanline_row(y, x1, x2, left, right, inv_left_depth, inv_right_depth, color, *properties);

                // for x in left..=right {

                //     let q3 = (x as f32 - x1) / (x2 - x1);
                //     let inv_depth = inv_left_depth * (1.0 - q3) + inv_right_depth * q3;
                //     let depth = 1.0 / inv_depth;

                //     let mut world_pos = Vec3::new(x as f32, y as f32, depth);
                //     self.camera.vertex_screen_to_camera_space(&mut world_pos);
                //     if world_pos.len_squared() < self.min_dist * self.min_dist {
                //         continue;
                //     }

                //     if depth < self.zbuf.get_depth(x, y) {
                //         if properties.alpha == 1.0 {
                //             self.zbuf.set_depth(x, y, depth);
                //         } else {
                //             let old_color = self.color_buf.get_pixel(x, y);
                //             // let new_color = (1.0 - properties.alpha) * Vec3::pairwise_mul_new(&properties.color, &old_color);
                //             let new_color = (1.0 - properties.alpha) * (properties.alpha * Vec3::mul_elementwise_of(old_color, color) + (1.0 - properties.alpha) * old_color);
                //             self.color_buf.set_pixel(x, y, new_color);
                //         }
                //     }
                // }
                x1 += slope3;
                x2 += slope2;
            }
        }
    }

    fn fill_triangle_scanline_row(&self, y: usize, x1: f32, x2: f32, left: usize, right: usize, inv_left_depth: f32, inv_right_depth: f32, color: Vec3, properties: PhongProperties) {
        let mut zbuf_row = self.zbuf.get_row_guard(y).lock().unwrap();
        let mut pixel_row = self.color_buf.get_row_guard(y).lock().unwrap();
        for x in left..=right {

            let q3 = (x as f32 - x1) / (x2 - x1);
            let inv_depth = inv_left_depth * (1.0 - q3) + inv_right_depth * q3;
            let depth = 1.0 / inv_depth;

            let mut world_pos = Vec3::new(x as f32, y as f32, depth);
            self.camera.vertex_screen_to_camera_space(&mut world_pos);
            if world_pos.len_squared() < self.min_dist * self.min_dist {
                continue;
            }

            if depth < zbuf_row[x] {
                if properties.alpha == 1.0 {
                    zbuf_row[x] = depth;
                } else {
                    let old_color = pixel_row[x];
                    // let new_color = (1.0 - properties.alpha) * Vec3::pairwise_mul_new(&properties.color, &old_color);
                    let new_color = (1.0 - properties.alpha) * (properties.alpha * Vec3::mul_elementwise_of(old_color, color) + (1.0 - properties.alpha) * old_color);
                    pixel_row[x] = new_color;
                }
            }
        }
    }

    pub fn get_lighting_at(&self, world_pos: &Vec3, observer_camera_pos: &Vec3, normal: &Vec3, color: Vec3, properties: &PhongProperties) -> Vec3 {
        
        // compute pixel-to-light vector and normalize
        let pixel_to_light = (self.camera.pos - *world_pos).normalized();

        // if normal pointing away from light, in shadow
        if normal.dot(pixel_to_light) < 0.0 {
            return Vec3::zero();
        }

        // transform world position to camera space
        let mut v = world_pos.clone();
        self.camera.vertex_world_to_camera_space(&mut v);

        let depth = v.x;
        if depth <= 0.0 { // pixel is behind the camera/light
            return Vec3::new(0.0, 0.0, 0.0);
        }

        let inv_dist = 1.0 / v.len();

        // transform world position to screen space
        self.camera.vertex_camera_to_screen_space(&mut v);

        let mut proportion_in_light = 0.0;
        let mut shadow_color = Vec3::new(0.0, 0.0, 0.0);
        let mut samples = 0;
        let filter_radius = 1;
        let bias = if properties.cull_faces {0.0} else {0.03};

        let x = v.x.round() as i32;
        let y = v.y.round() as i32;

        if x < 0 || x >= self.zbuf.width as i32 || y < 0 || y >= self.zbuf.height as i32 {
            return Vec3::new(0.0, 0.0, 0.0);
        }

        // for dy in -filter_radius..=filter_radius {
        //     for dx in -filter_radius..=filter_radius {
        //         let sample_x = x + dx;
        //         let sample_y = y + dy;

        //         if sample_x < 0 || sample_x >= self.zbuf.width as i32 || sample_y < 0 || sample_y >= self.zbuf.height as i32 {
        //             continue;
        //         }

        //         shadow_color += self.color_buf.get_pixel(sample_x as usize, sample_y as usize);
        //         if depth - bias < self.zbuf.get_depth(sample_x as usize, sample_y as usize) {
        //             proportion_in_light += 1.0;
        //         }
        //         samples += 1;
        //     }
        // }

        // only sample horizontally for better concurrency
        {
            let zbuf_row = self.zbuf.get_row_guard(y as usize).lock().unwrap();
            let pixel_row = self.color_buf.get_row_guard(y as usize).lock().unwrap();
            for dx in -filter_radius..=filter_radius {
                let sample_x = x + dx;
                if sample_x < 0 || sample_x >= self.zbuf.width as i32 {
                    continue;
                }
                shadow_color += pixel_row[sample_x as usize];
                if depth - bias < zbuf_row[sample_x as usize] {
                    proportion_in_light += 1.0;
                }
                samples += 1;
            }
        }

        // compute lighting components
        let angle_multiplier = pixel_to_light.dot(*normal);
        if angle_multiplier <= 0.0 || proportion_in_light == 0.0 || samples == 0 {
            return Vec3::new(0.0, 0.0, 0.0) // light is behind the surface or fully occluded
        }

        proportion_in_light /= samples as f32;
        shadow_color /= samples as f32;

        let mut light_color = Vec3::mul_elementwise_of(color, self.color);
        if properties.alpha == 1.0 {
            light_color.mul_elementwise_inplace(shadow_color);
        }

        let diffuse_light = properties.diffuse
            * angle_multiplier
            * inv_dist 
            * proportion_in_light
            * light_color;
            // Vec3::pairwise_mul_new(&properties.color, &self.color);
            // Vec3::pairwise_mul_new(&Vec3::pairwise_mul_new(&properties.color, &self.color), &self.color_buf.get_pixel(x as usize, y as usize));
            
        let mut specular_light = Vec3::new(0.0, 0.0, 0.0);

        if properties.specular > 0.0 {
            // BLINN PHONG MODEL
            // https://en.wikipedia.org/wiki/Phong_reflection_model
            // https://en.wikipedia.org/wiki/Blinn%E2%80%93Phong_reflection_model
            let V = (*observer_camera_pos - *world_pos).normalized();
            let H = (V + pixel_to_light).normalized();

            let NdotH = normal.dot(H);
            let exp_multiplier = 4;
            if NdotH >= 0.0 {
                specular_light = properties.specular 
                * NdotH.powi(exp_multiplier * properties.shininess)
                * inv_dist * proportion_in_light * light_color;
            }
        }
        return diffuse_light + specular_light;
    }
}

impl Clone for Light {
    fn clone(&self) -> Self {
        return Light {
            color: self.color.clone(),
            min_dist: self.min_dist,
            max_dist: self.max_dist,
            zbuf: ZBuffer::new(self.zbuf.width, self.zbuf.height),
            color_buf: PixelBuf::new(self.color_buf.width, self.color_buf.height),
            camera: self.camera.clone(),
        }
    }
}