use core::panic;
use std::{cell::RefCell, collections::HashSet, f32::consts::PI, sync::atomic::AtomicU32, vec};

use crate::{console_log, utils::{math::Vec3, utils::{gamma_correct_color, get_time, sort_objects_by_distance_to_camera}}};

use super::{buffers::{PixelBuf, ZBuffer}, camera::Camera, lighting::Light, rt::{Dielectric, Lambertian, Metal}, scene::{build_checkerboard, build_cube, build_icosahedron, MaterialProperties, SceneObject, Sphere, VertexObject}};

pub struct Game {
    pub camera: Camera,
    pub objects: RefCell<Vec<Box<dyn SceneObject>>>,
    pub lights: Vec<Light>,

    pub max_sky_color: Vec3,
    pub min_sky_color: Vec3,

    pub pixel_buf: PixelBuf,
    pub zbuf: ZBuffer,

    pub keys_currently_pressed: HashSet<String>,
    pub keys_pressed_last_frame: HashSet<String>,
    pub mouse_move: Vec3,

    pub running: bool,
}

impl Game {

    pub fn new() -> Game {
        let mut game = Game {
            camera: Camera::new(Vec3::new(0.001, 0.001, 0.501), 0.001, 0.001, PI/8.0, 500, 500),
            objects: RefCell::new(Vec::new()),
            lights: Vec::new(),
            max_sky_color: Vec3::new(0.5, 0.7, 1.0),
            min_sky_color: Vec3::new(1.0, 1.0, 1.0),
            pixel_buf: PixelBuf::new(500, 500),
            zbuf: ZBuffer::new(500, 500),
            keys_currently_pressed: HashSet::new(),
            keys_pressed_last_frame: HashSet::new(),
            mouse_move: Vec3::new(0.0, 0.0, 0.0),
            running: true,
        };

        game.create_rt_test_scene();

        // game.add_scene_object(build_cube(
        //     Vec3::new(11.0, 0.0, 0.5),
        //     1.0,
        //     Vec3::new(1.0, 0.0, 0.0),
        //     MaterialProperties::default())
        // );

        // central sphere
        // game.add_scene_object(Sphere::build_sphere(
        //     Vec3::new(10.2, 0.0, 0.5), 
        //     0.5, 
        //     4, 
        //     Vec3::new(1.0, 0.0, 0.0),
        //     MaterialProperties::default(),
        //     Box::new(Lambertian::default())
        // )
        // );

        // left sphere
        // game.add_scene_object(Sphere::build_sphere(
        //     Vec3::new(10.0, 1.0, 0.5), 
        //     0.5, 
        //     4, 
        //     Vec3::new(1.0, 1.0, 1.0),
        //     MaterialProperties::default(),
        //     Box::new(Dielectric::new(1.5))
        // )
        // );

        // inner left bubble
        // game.add_scene_object(Sphere::build_sphere(
        //     Vec3::new(10.0, 1.0, 0.5), 
        //     0.4, 
        //     4, 
        //     Vec3::new(1.0, 1.0, 1.0),
        //     MaterialProperties::default(),
        //     Box::new(Dielectric::new(1.0 / 1.5))
        // )
        // );

        // right sphere
        // game.add_scene_object(Sphere::build_sphere(
        //     Vec3::new(10.0, -1.0, 0.5), 
        //     0.5, 
        //     4, 
        //     Vec3::new(0.8, 0.6, 0.2),
        //     MaterialProperties::default(),
        //     Box::new(Metal::new(1.0))
        // )
        // );

        // bottom large sphere "ground/floor"
        // game.add_scene_object(Sphere::build_sphere(
        //     Vec3::new(10.0, 0.0, -100.0), 
        //     100.0, 
        //     4, 
        //     Vec3::new(0.5, 1.0, 0.5),
        //     MaterialProperties::default(),
        //     Box::new(Lambertian::default())
        // )
        // );
        
        // game.add_scene_object(build_cube(
        //     Vec3::new(12.0, 0.0, 0.5),
        //     1.0,
        //     Vec3::new(1.0, 0.0, 0.0),
        //     MaterialProperties::new(
        //         0.05,
        //         0.8,
        //         1.0,
        //         1.0,
        //         32,
        //     ),
        // ));
        // game.add_scene_object(build_cube(
        //     Vec3::new(13.0, 0.0, 0.5),
        //     1.0,
        //     Vec3::new(0.0, 1.0, 0.0),
        //     MaterialProperties::new(
        //         0.2,
        //         0.8,
        //         1.0,
        //         1.0,
        //         32,
        //     ),
        // ));
        // game.add_scene_object(build_cube(
        //     Vec3::new(14.0, 0.0, 0.5),
        //     1.0,
        //     Vec3::new(0.0, 0.0, 1.0),
        //     MaterialProperties::new(
        //         0.5,
        //         0.8,
        //         1.0,
        //         1.0,
        //         32,
        //     ),
        // ));
        // game.add_scene_object(build_cube(
        //     Vec3::new(15.0, 0.0, 0.5),
        //     1.0,
        //     Vec3::new(1.0, 0.0, 0.0),
        //     MaterialProperties::new(
        //         0.8,
        //         0.8,
        //         1.0,
        //         1.0,
        //         32,
        //     ),
        // ));
        // game.add_scene_object(build_cube(
        //     Vec3::new(16.0, 0.0, 0.5),
        //     1.0,
        //     Vec3::new(0.0, 1.0, 0.0),
        //     MaterialProperties::new(
        //         0.95,
        //         0.8,
        //         1.0,
        //         1.0,
        //         32,
        //     ),
        // ));
        // game.add_scene_object(build_cube(
        //     Vec3::new(17.0, 0.0, 0.5),
        //     1.0,
        //     Vec3::new(0.0, 0.0, 1.0),
        //     MaterialProperties::new(
        //         1.0,
        //         0.8,
        //         1.0,
        //         1.0,
        //         32,
        //     ),
        // ));

        // game.add_scene_object(build_checkerboard(
        //         Vec3::new(10.0, 0.0, 0.0), 
        //         20, 
        //         Vec3::new(1.0, 1.0, 1.0), 
        //         Vec3::new(0.9, 0.9, 0.9),
        //         MaterialProperties::default(),
        //     )
        // );

        // let mut stl_obj = VertexObject::new_from_stl_bytes(
        //     &include_bytes!("../3DBenchy.stl").to_vec(),
        //     Vec3::new(1.0, 0.2, 0.5), 
        //     MaterialProperties::new(
        //         1.0,
        //         0.8,
        //         1.0,
        //         1.0,
        //         32,
        //     ));
        // stl_obj.scale_to_radius(2.0);
        // stl_obj.set_center(Vec3::new(7.0, 0.0, 1.0));
        // game.add_scene_object(stl_obj);

        let mut light = Light::new(
            Camera::new(Vec3::new(10.0, 100.0, 100.0), 0.0, 0.0, PI/15.0, 2000, 2000),
            Vec3::new(1.0, 1.0, 1.0),
            50.0,
            ZBuffer::new(2000, 2000),
            PixelBuf::new(2000, 2000),
        );
        light.camera.look_at(&Vec3::new(15.0, 0.0, 1.0));
        game.lights.push(light);

        for light in game.lights.iter_mut() {
            light.clear_shadow_map();
            light.add_objects_to_shadow_map(&mut game.objects.borrow_mut());
        }

        return game;
    }

    pub fn game_loop(&mut self) {
        self.process_input();
        if self.running {
            self.render_frame();
            self.apply_post_processing_effects();
        }
    }

    fn process_input(&mut self) {
        const MOVE_SPEED: f32 = 0.1;
        const ROTATE_SPEED: f32 = 0.01;
        const KEY_ROTATE_SPEED: f32 = 0.03;

        let mut move_dir = Vec3::new(0.0, 0.0, 0.0);
        if self.keys_currently_pressed.contains("w") {
            move_dir.x += 1.0;
        }
        if self.keys_currently_pressed.contains("s") {
            move_dir.x -= 1.0;
        }
        if self.keys_currently_pressed.contains("a") {
            move_dir.y += 1.0;
        }
        if self.keys_currently_pressed.contains("d") {
            move_dir.y -= 1.0;
        }
        if self.keys_currently_pressed.contains(" ") {
            move_dir.z += 1.0;
        }
        if self.keys_currently_pressed.contains("Shift") {
            move_dir.z -= 1.0;
        }
        move_dir.rotate_z(self.camera.get_theta_z());
        self.camera.pos += move_dir * MOVE_SPEED;

        let mut d_theta_z = 0.0;
        let mut d_theta_y = 0.0;
        if self.keys_currently_pressed.contains("ArrowLeft") {
            d_theta_z += KEY_ROTATE_SPEED;
        }
        if self.keys_currently_pressed.contains("ArrowRight") {
            d_theta_z -= KEY_ROTATE_SPEED;
        }
        if self.keys_currently_pressed.contains("ArrowUp") {
            d_theta_y += KEY_ROTATE_SPEED;
        }
        if self.keys_currently_pressed.contains("ArrowDown") {
            d_theta_y -= KEY_ROTATE_SPEED;
        }
        d_theta_z -= self.mouse_move.x * ROTATE_SPEED;
        d_theta_y -= self.mouse_move.y * ROTATE_SPEED;
        self.mouse_move = Vec3::new(0.0, 0.0, 0.0);

        self.camera.set_theta_y(self.camera.get_theta_y() + d_theta_y);
        self.camera.set_theta_z(self.camera.get_theta_z() + d_theta_z);


        if self.keys_pressed_last_frame.contains("p") {
            console_log!("Pausing or unpausing");
            self.running = !self.running;
        }
        if self.keys_pressed_last_frame.contains("r") {
            // TODO: make this input robust, including cancelling raytracing
            self.render_ray_tracing();
        }

        self.keys_pressed_last_frame.clear();

    }

    pub fn apply_post_processing_effects(&mut self) {
        for y in 0..self.camera.height {
            for x in 0..self.camera.width {
                let color = self.pixel_buf.get_pixel(x, y);
                let gamma_color = gamma_correct_color(&color);
                self.pixel_buf.set_pixel(x, y, gamma_color);
            }
        }
    }

    fn render_frame(&mut self) {
        // curr time is ~73ms - finished adding lighting)
        // ~57ms (20% improvement)- after precomputing sin/cos)
        // almost 30% improvement from using powi over powf
        // negligible decrease in perf for most cases from adding transparency + transparent shadows
        //      WARNING - transparent objects can cause significant (3x-5x) performance drops 
        //      since multiple layers must be rendered each time

        // Now testing on fantasy book gltf, with indexed triangles:
        // 65ms (from performance tab)
        // 

        let t1 = get_time();

        // clear buffers
        self.clear_pixel_buf_to_sky();
        self.zbuf.clear();


        let mut objects = self.objects.take();
        sort_objects_by_distance_to_camera(&mut objects, &self.camera.pos);

        // opaque objects
        for obj in objects.iter() {
            if obj.get_properties().alpha < 1.0 {
                continue;
            }
            let vertices = obj.get_vertices();
            let transformed_vertices = self.camera.vertices_world_to_camera_space(&vertices);
            let indices = obj.get_indices();
            let colors = obj.get_colors();
            let normals = obj.get_normals();
            for i in 0..colors.len() {
                let v1 = transformed_vertices[indices[i*3]];
                let v2 = transformed_vertices[indices[i*3+1]];
                let v3 = transformed_vertices[indices[i*3+2]];
                let color = colors[i];
                let normal = normals[i];
                self.render_triangle_from_transformed_vertices(v1, v2, v3, normal, color, &obj);
            }
        }

        // transparent objects
        for obj in objects.iter().rev() {
            if obj.get_properties().alpha == 1.0 {
                continue;
            }
            let vertices = obj.get_vertices();
            let transformed_vertices = self.camera.vertices_world_to_camera_space(&vertices);
            let indices = obj.get_indices();
            let colors = obj.get_colors();
            let normals = obj.get_normals();
            for i in 0..colors.len() {
                let v1 = transformed_vertices[indices[i*3]];
                let v2 = transformed_vertices[indices[i*3+1]];
                let v3 = transformed_vertices[indices[i*3+2]];
                let color = colors[i];
                let normal = normals[i];
                self.render_triangle_from_transformed_vertices(v1, v2, v3, normal, color, &obj);
            }
        }


        self.objects.replace(objects);

        let t2 = get_time();
        // console_log!("Frame time: {}", t2 - t1);
    }

    fn render_triangle(&mut self, mut v1: Vec3, mut v2: Vec3, mut v3: Vec3, color: Vec3, scene_obj: &Box<dyn SceneObject>) {
        // render triangle from transformed vertices
        let normal = (&(v3 - v1)).cross(&(v2 - v1)).normalized();
        self.camera.three_vertices_world_to_camera_space(&mut v1, &mut v2, &mut v3);
        self.render_triangle_from_transformed_vertices(v1, v2, v3, normal, color, scene_obj);
    }

    fn render_triangle_from_transformed_vertices(&mut self, mut v1: Vec3, mut v2: Vec3, mut v3: Vec3, normal: Vec3, color: Vec3, scene_obj: &Box<dyn SceneObject>) {

        // do not render if normal is pointing away from cam - BACK FACE CULLING
        // only applies to opaque objects
        if scene_obj.get_properties().alpha == 1.0 {
            let cam_normal = (&(v3 - v1)).cross(&(v2 - v1));
            let cam_to_tri = v1;
            if cam_to_tri.dot(&cam_normal) > 0.0 {
                return;
            }
        }

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
            self.fill_triangle(v1, v2, v3, &normal, color, scene_obj);
        } else if v2.x > 0.0 { // 2 vertices in view
            let q = (NEAR_PLANE - v2.x) / (v1.x - v2.x);
            let mut v1_new_1 = v2 + (v1 - v2) * q;
            let q = (NEAR_PLANE - v3.x) / (v1.x - v3.x);
            let mut v1_new_2 = v3 + (v1 - v3) * q;

            self.camera.vertices_camera_to_screen_space(&mut v1_new_1, &mut v2, &mut v3);
            self.camera.vertex_camera_to_screen_space(&mut v1_new_2);
            self.fill_triangle(v1_new_1, v2, v3, &normal, color, scene_obj);
            self.fill_triangle(v1_new_1, v1_new_2, v3, &normal, color, scene_obj);
        } else if v3.x > 0.0 { // 1 vertex in view
            let q = (NEAR_PLANE - v2.x) / (v3.x - v2.x);
            let mut v2_new = v2 + (v3 - v2) * q;
            let q = (NEAR_PLANE - v1.x) / (v3.x - v1.x);
            let mut v1_new = v1 + (v3 - v1) * q;

            self.camera.vertices_camera_to_screen_space(&mut v1_new, &mut v2_new, &mut v3);
            self.fill_triangle(v1_new, v2_new, v3, &normal, color, scene_obj);
        } else { // no vertices in view
            return;
        }
    }


    fn fill_triangle(&mut self, mut v1: Vec3, mut v2: Vec3, mut v3: Vec3, normal: &Vec3, color: Vec3, scene_obj: &Box<dyn SceneObject>) {
        // depth calculations from https://www.scratchapixel.com/lessons/3d-basic-rendering/rasterization-practical-implementation/visibility-problem-depth-buffer-depth-interpolation.html#:~:text=As%20previously%20mentioned%2C%20the%20correct,z%20%3D%201%20V%200.

        let properties = scene_obj.get_properties();

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

        let width = 500.0; // TODO: replace hardcoded values
        let height = 500.0;

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

                for x in left..=right {

                    let q3 = (x as f32 - x1) / (x2 - x1);
                    let inv_depth = inv_left_depth * (1.0 - q3) + inv_right_depth * q3;
                    let depth = 1.0 / inv_depth;
                    let bias = if properties.alpha == 1.0 {0.0} else {0.01};

                    if depth - bias < self.zbuf.get_depth(x, y) {

                        let mut world_pos = Vec3::new(x as f32, y as f32, depth);
                        self.camera.vertex_screen_to_camera_space(&mut world_pos);
                        self.camera.vertex_camera_to_world_space(&mut world_pos);

                        let sky_color = self.get_sky_color(normal);

                        // start as ambient light
                        let mut blended_color = properties.ambient * Vec3::element_mul(&sky_color, &color);

                        for light in &self.lights {
                            blended_color += light.get_lighting_at(&world_pos, &self.camera.pos, normal, color, properties);
                        }
                        blended_color.x = blended_color.x.min(1.0);
                        blended_color.y = blended_color.y.min(1.0);
                        blended_color.z = blended_color.z.min(1.0);

                        if properties.alpha == 1.0 {
                            self.zbuf.set_depth(x, y, depth);
                            self.pixel_buf.set_pixel(x, y, blended_color);
                        } else {
                            // alpha blending, don't set depth
                            let old_color = self.pixel_buf.get_pixel(x, y);
                            blended_color = blended_color * properties.alpha + old_color * (1.0 - properties.alpha);
                            self.pixel_buf.set_pixel(x, y, blended_color);

                            // self.pixel_buf.set_pixel(x, y, color);
                        }
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

                for x in left..=right {

                    let q3 = (x as f32 - x1) / (x2 - x1);
                    let inv_depth = inv_left_depth * (1.0 - q3) + inv_right_depth * q3;
                    let depth = 1.0 / inv_depth;
                    let bias = if properties.alpha == 1.0 {0.0} else {0.01};

                    if depth - bias < self.zbuf.get_depth(x, y) {

                        let mut world_pos = Vec3::new(x as f32, y as f32, depth);
                        self.camera.vertex_screen_to_camera_space(&mut world_pos);
                        self.camera.vertex_camera_to_world_space(&mut world_pos);

                        let sky_color = self.get_sky_color(normal);

                        // start as ambient light
                        let mut blended_color = properties.ambient * Vec3::element_mul(&sky_color, &color);

                        for light in &self.lights {
                            blended_color += light.get_lighting_at(&world_pos, &self.camera.pos, normal, color, properties);
                        }
                        blended_color.x = blended_color.x.min(1.0);
                        blended_color.y = blended_color.y.min(1.0);
                        blended_color.z = blended_color.z.min(1.0);

                        if properties.alpha == 1.0 {
                            self.zbuf.set_depth(x, y, depth);
                            self.pixel_buf.set_pixel(x, y, blended_color);
                        } else {
                            // alpha blending, don't set depth
                            let old_color = self.pixel_buf.get_pixel(x, y);
                            blended_color = blended_color * properties.alpha + old_color * (1.0 - properties.alpha);
                            self.pixel_buf.set_pixel(x, y, blended_color);

                            // self.pixel_buf.set_pixel(x, y, color);
                        }
                    }
                }
                x1 += slope3;
                x2 += slope2;
            }
        }
    }

    pub fn get_sky_color(&self, dir: &Vec3) -> Vec3 {
        let a = 0.5 * (dir.z + 1.0);
        return self.min_sky_color * (1.0 - a) + self.max_sky_color * a;
    }
    pub fn clear_pixel_buf_to_sky(&mut self) {
        let width = self.pixel_buf.width;
        let height = self.pixel_buf.height;
        for x in 0..height {
            for y in 0..width {
                let mut v = Vec3::new(x as f32, y as f32, 1.0);
                self.camera.vertex_screen_to_world_space(& mut v);
                v -= self.camera.pos;
                v.normalize();
                let sky_color = self.get_sky_color(&v);
                self.pixel_buf.set_pixel(x, y, sky_color);
            }
        }
    }

    pub fn add_scene_object<T: SceneObject + 'static>(&mut self, object: T) {
        self.objects.borrow_mut().push(Box::new(object));
        for light in self.lights.iter_mut() {
            // light.clear_shadow_map();
            // light.add_objects_to_shadow_map(&mut self.objects.borrow_mut());
            light.add_object_to_shadow_map(self.objects.borrow().last().unwrap());
        }
    }
    pub fn add_scene_objects<T: SceneObject + 'static>(&mut self, objects: Vec<T>) {
        for obj in objects {
            self.add_scene_object(obj);
        }
    }
}