use core::panic;
use std::{cell::RefCell, collections::HashSet, f32::consts::PI, vec};

use crate::{console_log, utils::math::Vec3, wasm::wasm::get_time};

use super::{buffers::{PixelBuf, ZBuffer}, camera::Camera, scene::{build_checkerboard, build_cube, MaterialProperties, SceneObject, VertexObject}};

pub struct Game {
    pub camera: Camera,
    pub objects: RefCell<Vec<Box<dyn SceneObject>>>,

    pub pixel_buf: PixelBuf,
    pub zbuf: ZBuffer,

    pub keys_currently_pressed: HashSet<String>,
    pub keys_pressed_last_frame: HashSet<String>,
    pub mouse_move: Vec3,
}

impl Game {

    pub fn new() -> Game {
        let mut game = Game {
            camera: Camera::new(Vec3::new(0.0, 0.0, 0.0), 0.0, 0.0, PI/2.0, 500, 500),
            objects: RefCell::new(Vec::new()),
            pixel_buf: PixelBuf::new(500, 500),
            zbuf: ZBuffer::new(500, 500),
            keys_currently_pressed: HashSet::new(),
            keys_pressed_last_frame: HashSet::new(),
            mouse_move: Vec3::new(0.0, 0.0, 0.0),
        };

        game.add_scene_object(build_cube(
            Vec3::new(10.0, 0.0, 0.0),
            1.0,
            MaterialProperties::default_from_color(Vec3::new(1.0, 0.0, 0.0)),
        ));
        game.add_scene_objects(
            build_checkerboard(
                &Vec3::new(10.0, 0.0, 0.0), 
                5, 
                &Vec3::new(0.8, 0.8, 0.8), 
                &Vec3::new(0.6, 0.6, 0.6)
            )
        );

        // let obj = VertexObject::new(
        //     vec![
        //         Vec3::new(10.0, 1.0, 0.0),
        //         Vec3::new(10.0, 1.0, 6.0),
        //         Vec3::new(20.0, 1.0, 0.0),
        //     ],
        //     MaterialProperties::default_from_color(Vec3::new(0.8, 0.8, 0.8)),
        // );
        // game.add_scene_object(obj);

        return game;
    }

    pub fn game_loop(&mut self) {
        self.process_input();
        self.render_frame();
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
        move_dir.rotate_z(self.camera.theta_z);
        self.camera.pos += move_dir * MOVE_SPEED;

        if self.keys_currently_pressed.contains("ArrowLeft") {
            self.camera.theta_z += KEY_ROTATE_SPEED;
        }
        if self.keys_currently_pressed.contains("ArrowRight") {
            self.camera.theta_z -= KEY_ROTATE_SPEED;
        }
        if self.keys_currently_pressed.contains("ArrowUp") {
            self.camera.theta_y += KEY_ROTATE_SPEED;
        }
        if self.keys_currently_pressed.contains("ArrowDown") {
            self.camera.theta_y -= KEY_ROTATE_SPEED;
        }
        self.camera.theta_z -= self.mouse_move.x * ROTATE_SPEED;
        self.camera.theta_y -= self.mouse_move.y * ROTATE_SPEED;
        self.mouse_move = Vec3::new(0.0, 0.0, 0.0);
        self.camera.theta_y = self.camera.theta_y.clamp(-0.5 * PI, 0.5 * PI);
    }

    fn render_frame(&mut self) {

        let t1 = get_time();

        self.pixel_buf.clear();
        self.zbuf.clear();

        let objects = self.objects.take();
        for obj in &objects {
            let vertices = obj.get_vertices();
            for i in (0..vertices.len()).step_by(3) {
                self.render_triangle(vertices[i], vertices[i+1], vertices[i+2], &obj);
            }
        }
        self.objects.replace(objects);

        let t2 = get_time();
        // console_log!("Frame time: {}", t2 - t1);
    }

    fn render_triangle(&mut self, mut v1: Vec3, mut v2: Vec3, mut v3: Vec3, scene_obj: &Box<dyn SceneObject>) {

        // do not render if normal is pointing away from cam - BACK FACE CULLING
        let normal = (&(v3 - v1)).cross(&(v2 - v1)).normalized();
        let cam_to_triangle = v1 - self.camera.pos;

        if normal.dot(&cam_to_triangle) > 0.0 {
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
        // Some basic clipping info: https://www.khronos.org/opengl/wiki/Vertex_Post-Processing
        const NEAR_PLANE: f32 = 0.001;
        if v1.x > 0.0 { // all vertices in view
            self.camera.vertices_camera_to_screen_space(&mut v1, &mut v2, &mut v3);
            self.fill_triangle(v1, v2, v3, scene_obj);
        } else if v2.x > 0.0 { // 2 vertices in view
            let q = (NEAR_PLANE - v2.x) / (v1.x - v2.x);
            let mut v1_new_1 = v2 + (v1 - v2) * q;
            let q = (NEAR_PLANE - v3.x) / (v1.x - v3.x);
            let mut v1_new_2 = v3 + (v1 - v3) * q;

            self.camera.vertices_camera_to_screen_space(&mut v1_new_1, &mut v2, &mut v3);
            self.camera.vertex_camera_to_screen_space(&mut v1_new_2);
            self.fill_triangle(v1_new_1, v2, v3, scene_obj);
            self.fill_triangle(v1_new_1, v1_new_2, v3, scene_obj);
        } else if v3.x > 0.0 { // 1 vertex in view
            let q = (NEAR_PLANE - v2.x) / (v3.x - v2.x);
            let mut v2_new = v2 + (v3 - v2) * q;
            let q = (NEAR_PLANE - v1.x) / (v3.x - v1.x);
            let mut v1_new = v1 + (v3 - v1) * q;

            self.camera.vertices_camera_to_screen_space(&mut v1_new, &mut v2_new, &mut v3);
            self.fill_triangle(v1_new, v2_new, v3, scene_obj);
        } else { // no vertices in view
            return;
        }
    }


    fn fill_triangle(&mut self, mut v1: Vec3, mut v2: Vec3, mut v3: Vec3, scene_obj: &Box<dyn SceneObject>) {
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
                        self.pixel_buf.set_pixel(x, y, properties.color);
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
                        self.pixel_buf.set_pixel(x, y, properties.color);
                    }
                }
                x1 += slope3;
                x2 += slope2;
            }
        }
    }

    pub fn add_scene_object<T: SceneObject + 'static>(&mut self, object: T) {
        self.objects.borrow_mut().push(Box::new(object));
    }
    pub fn add_scene_objects<T: SceneObject + 'static>(&mut self, objects: Vec<T>) {
        for obj in objects {
            self.objects.borrow_mut().push(Box::new(obj));
        }
    }
}