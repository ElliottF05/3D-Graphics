use std::{cell::RefCell, collections::HashSet, f32::consts::PI};

use crate::{console_error, console_log, console_warn, utils::{math::Vec3, utils::{clamp_color, gamma_correct_color, get_time, shift_color}}, wasm::wasm::{js_set_is_object_selected, GameCommand, UI_COMMAND_QUEUE}};

use super::{buffers::{PixelBuf, ZBuffer}, camera::Camera, lighting::Light, mesh::{Mesh, PhongProperties}, ray_tracing::{bvh::BVHNode, hittable::Hittable, material::{Dielectric, Lambertian, Material, Metal}}, scene_object::SceneObject};

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum GameStatus {
    Rasterizing(RasterStatus),
    RayTracing,
    Paused,
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum RasterStatus {
    Normal,
    EditMode { selected_index: Option<usize> },
}

pub struct Game {
    pub scene_objects: RefCell<Vec<SceneObject>>,
    lights: Vec<Light>,

    pub camera: Camera,

    pub max_sky_color: Vec3,
    pub min_sky_color: Vec3,

    pub pixel_buf: PixelBuf,
    pub zbuf: ZBuffer,

    pub keys_currently_pressed: HashSet<String>,
    pub keys_pressed_last_frame: HashSet<String>,
    pub mouse_move: Vec3,
    pub mouse_clicked_last_frame: bool,

    pub looking_at: Option<(usize, Vec3)>,
    pub selected_index: Option<usize>,
    pub follow_camera: bool,

    pub status: GameStatus,
    pub rt_row: usize,

    // ray-tracing variables
    pub bvh: Option<BVHNode>,
    rt_lights: Vec<Box<dyn Hittable>>,
    pub ray_samples: usize,
    pub ray_max_depth: usize,

    pub defocus_angle: f32,
    pub focus_dist: f32,

    // debug stuff
    pub rt_start_time: f64,

    // testing
}

impl Game {

    pub fn new() -> Game {
        let mut game = Game {
            scene_objects: RefCell::new(Vec::new()),
            lights: Vec::new(),

            camera: Camera::new(Vec3::new(0.001, 0.001, 0.501), 0.001, 0.001, PI/2.0, 500, 500),

            max_sky_color: Vec3::new(0.5, 0.7, 1.0),
            min_sky_color: Vec3::new(1.0, 1.0, 1.0),

            pixel_buf: PixelBuf::new(500, 500),
            zbuf: ZBuffer::new(500, 500),

            keys_currently_pressed: HashSet::new(),
            keys_pressed_last_frame: HashSet::new(),
            mouse_move: Vec3::new(0.0, 0.0, 0.0),
            mouse_clicked_last_frame: false,

            looking_at: None,
            selected_index: None,
            follow_camera: false,

            status: GameStatus::Rasterizing(RasterStatus::Normal),
            rt_row: 0,

            // ray tracing variables
            bvh: None,
            rt_lights: Vec::new(),
            ray_samples: 10,
            ray_max_depth: 10,

            defocus_angle: 0.0,
            focus_dist: 10.0,

            // debug stuff
            rt_start_time: 0.0,

            // testing
        };

        // game.create_rt_test_scene_spheres();
        // game.create_rt_test_scene_simple_light();
        // game.create_rt_test_scene_cornell();
        game.create_rt_test_scene_cornell_metal();

        // game.add_mesh(Mesh::build_cube(
        //     Vec3::new(11.0, 0.0, 0.5),
        //     1.0,
        //     Vec3::new(1.0, 0.0, 0.0),
        //     PhongProperties::default())
        // );

        // game.add_mesh(Mesh::build_checkerboard(
        //     Vec3::zero(), 
        //     20, 
        //     Vec3::new(0.8, 0.8, 0.8), 
        //     Vec3::new(0.6, 0.6, 0.6), 
        //     PhongProperties::default(),
        //     false,
        // ));

        // let light = Light::new(
        //     Vec3::new(10.0, 100.0, 100.0), 
        //     Vec3::new(0.0, -100.0, -100.0),
        //     PI/15.0,
        //     50.0 * Vec3::white(),
        //     1.0,
        //     2000,
        //     2000
        // );
        // game.lights.push(light);

        // for light in game.lights.iter_mut() {
        //     light.clear_shadow_map();
        //     light.add_meshes_to_shadow_map(&game.meshes.borrow());
        // }

        // let rt_objects = game
        //     .meshes
        //     .borrow()
        //     .iter()
        //     .flat_map(|m| m.to_rt_hittables(&Lambertian::default()))
        //     .collect();
        
        // game.bvh = Some(BVHNode::new(rt_objects));

        return game;
    }


    pub fn game_loop(&mut self) {

        // process these no matter the game state
        self.process_js_ui_commands();
        self.process_all_input();

        match self.status {

            GameStatus::Rasterizing(raster_status) => {
                match raster_status {
                    RasterStatus::Normal => {
                        self.pre_raster_render_logic();
                        self.render_frame();
                        self.apply_post_processing_effects();
                    },
                    RasterStatus::EditMode { selected_index } => {
                        self.pre_raster_render_logic();
                        self.render_frame();
                        self.apply_post_processing_effects();
                    },
                }
            },

            GameStatus::RayTracing => {
                self.render_ray_tracing();
            },
            GameStatus::Paused => {

            }
        }
    }

    fn process_js_ui_commands(&mut self) { // Takes &mut self
        // Access the shared UI_COMMAND_QUEUE (needs to be in scope or use full path)
        // To access thread_local from another module, you might need to make UI_COMMAND_QUEUE pub
        // or pass the commands in. For simplicity, let's assume it's accessible.
        // If not, init_and_begin_game_loop would need to drain it and pass to game.game_loop().
        // For now, let's assume direct access for clarity of the pattern:
        UI_COMMAND_QUEUE.with(|queue_cell| { // Adjust path to UI_COMMAND_QUEUE as needed
            let mut queue = queue_cell.borrow_mut();
            for command in queue.drain(..) { // drain() consumes the commands
                match command {
                    GameCommand::SetMaterialColor { r, g, b } => {
                        self.process_set_material_color(r, g, b);
                    }
                    // Handle other commands here
                }
            }
        });
    }

    fn process_set_material_color(&mut self, r: f32, g: f32, b: f32) {
        if let Some(selected_index) = self.selected_index {
            let mut scene_objects_mut = self.scene_objects.borrow_mut();
            if selected_index < scene_objects_mut.len() {
                let scene_obj = &mut scene_objects_mut[selected_index];
                let color_vec = Vec3::new(r, g, b);
                scene_obj.set_color(color_vec);
            } else {
                console_warn!("set_material_color: selected_index out of bounds.");
            }
        } else {
            console_warn!("set_material_color: No object selected.");
        }
    }

    fn process_all_input(&mut self) {

        if let GameStatus::Rasterizing(_) = self.status {
            self.process_rasterization_input();
        }

        if self.keys_pressed_last_frame.contains("p") {
            console_log!("Pausing or unpausing");
            if self.status == GameStatus::Paused {
                self.status = GameStatus::Rasterizing(RasterStatus::Normal);
            } else {
                self.status = GameStatus::Paused;
            }
        }
        if self.keys_pressed_last_frame.contains("r") {
            self.status = GameStatus::RayTracing;
            self.rt_start_time = get_time();
        }

        self.keys_pressed_last_frame.clear();
        self.mouse_clicked_last_frame = false;
    }

    fn process_rasterization_input(&mut self) {
        self.process_movement_input();
        if self.mouse_clicked_last_frame {
            if let Some((looking_at_index, looking_at_pos)) = self.looking_at {
                if self.selected_index.is_some() && self.selected_index.unwrap() == looking_at_index {
                    self.deselect_object();
                } else {
                    self.select_object(looking_at_index);
                }
            } else {
                self.deselect_object();
            }
        }
    }

    fn select_object(&mut self, index: usize) {
        console_log!("Selected object with index: {}", index);
        self.selected_index = Some(index);
        self.follow_camera = false;
        js_set_is_object_selected(true);
    }

    pub fn deselect_object(&mut self) {
        console_log!("Deselected object");
        self.selected_index = None;
        js_set_is_object_selected(false);
    }

    pub fn delete_selected_object(&mut self) {
        if let Some(selected_index) = self.selected_index {
            console_log!("Deleting object with index: {}", selected_index);
            self.scene_objects.borrow_mut().remove(selected_index);
            self.deselect_object();
            self.bvh = None; // invalidate bvh if obj is deleted
            self.extract_lights_from_scene_objects();
            self.recalculate_shadow_maps();
        } else {
            console_error!("Game::delete_selected_object() called but no object is selected");
        }
    }

    fn process_movement_input(&mut self) {
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
    }

    fn pre_raster_render_logic(&mut self) {
        if self.follow_camera {
            if let Some(selected_index) = self.selected_index {
                let selected_obj = &mut self.scene_objects.borrow_mut()[selected_index];
                let mut looking_at_pos = if let Some(looking_at) = self.looking_at {
                    console_log!("looking at object, translating to {:?}", looking_at.1);
                    looking_at.1
                } else {
                    let mut looking_at_pos = Vec3::new(self.camera.width as f32 / 2.0, self.camera.height as f32 / 2.0, 8.0 * selected_obj.mesh.radius);
                    self.camera.vertex_screen_to_world_space(&mut looking_at_pos);
                    console_log!("not looking at anything, translating to {:?}", looking_at_pos);
                    looking_at_pos
                };
                // let cam_to_pos_dir = (looking_at_pos - self.camera.pos).normalized();
                // looking_at_pos -= cam_to_pos_dir * selected_obj.mesh.radius;
                selected_obj.translate_to(looking_at_pos);
            }
        }
        self.looking_at = None;
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

        let t1 = get_time();

        // clear buffers
        self.clear_pixel_buf_to_sky();
        self.zbuf.clear();

        // Also, for fixing the alpha blending sorting not applying within an
        // object's triangles (since only objects are sorted, not invidivual triangles),
        // could try a hybrid approach of sorting the triangles within only each transparent object,
        // not within the whole scene.;

        // self.sort_meshes_by_distance_to_camera();
        let scene_objects = self.scene_objects.take();

        // opaque objects
        for (scene_obj_index, scene_obj) in scene_objects.iter().enumerate() {
            let mesh = &scene_obj.mesh;
            if mesh.properties.alpha < 1.0 {
                continue;
            }
            let vertices = &mesh.vertices;
            let transformed_vertices = self.camera.vertices_world_to_camera_space(&vertices);
            let indices = &mesh.indices;
            let colors = &mesh.colors;
            let normals = &mesh.normals;
            for i in 0..colors.len() {
                let v1 = transformed_vertices[indices[i*3]];
                let v2 = transformed_vertices[indices[i*3+1]];
                let v3 = transformed_vertices[indices[i*3+2]];
                let color = colors[i];
                let normal = normals[i];
                self.render_triangle_from_transformed_vertices(v1, v2, v3, normal, color, &scene_obj, scene_obj_index);
            }
        }

        // transparent objects
        for (idx, scene_obj) in scene_objects.iter().rev().enumerate() {
            let scene_obj_index = scene_objects.len() - 1 - idx;
            let mesh = &scene_obj.mesh;
            if mesh.properties.alpha == 1.0 {
                continue;
            }
            let vertices = &mesh.vertices;
            let transformed_vertices = self.camera.vertices_world_to_camera_space(&vertices);
            let indices = &mesh.indices;
            let colors = &mesh.colors;
            let normals = &mesh.normals;
            for i in 0..colors.len() {
                let v1 = transformed_vertices[indices[i*3]];
                let v2 = transformed_vertices[indices[i*3+1]];
                let v3 = transformed_vertices[indices[i*3+2]];
                let color = colors[i];
                let normal = normals[i];
                self.render_triangle_from_transformed_vertices(v1, v2, v3, normal, color, &scene_obj, scene_obj_index);
            }
        }

        self.scene_objects.replace(scene_objects);

        let t2 = get_time();
        // console_log!("Frame time: {}", t2 - t1);
    }

    fn render_triangle(&mut self, mut v1: Vec3, mut v2: Vec3, mut v3: Vec3, color: Vec3, scene_obj: &SceneObject, scene_obj_index: usize) {
        // render triangle from transformed vertices
        let normal = (v3 - v1).cross(v2 - v1).normalized();
        self.camera.three_vertices_world_to_camera_space(&mut v1, &mut v2, &mut v3);
        self.render_triangle_from_transformed_vertices(v1, v2, v3, normal, color, scene_obj, scene_obj_index);
    }

    fn render_triangle_from_transformed_vertices(&mut self, mut v1: Vec3, mut v2: Vec3, mut v3: Vec3, mut normal: Vec3, color: Vec3, scene_obj: &SceneObject, scene_obj_index: usize) {

        // do not render if normal is pointing away from cam - BACK FACE CULLING
        // only applies to opaque objects
        if scene_obj.mesh.properties.alpha == 1.0 {
            let cam_normal = (v3 - v1).cross(v2 - v1);
            let cam_to_tri = v1;
            if cam_to_tri.dot(cam_normal) > 0.0 {
                if scene_obj.mesh.properties.cull_faces {
                    return; // cull triangle
                } else {
                    normal *= -1.0; // ensure normal points towards cam if not culling faces
                }
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
            self.fill_triangle(v1, v2, v3, &normal, color, scene_obj, scene_obj_index);
        } else if v2.x > 0.0 { // 2 vertices in view
            let q = (NEAR_PLANE - v2.x) / (v1.x - v2.x);
            let mut v1_new_1 = v2 + (v1 - v2) * q;
            let q = (NEAR_PLANE - v3.x) / (v1.x - v3.x);
            let mut v1_new_2 = v3 + (v1 - v3) * q;

            self.camera.vertices_camera_to_screen_space(&mut v1_new_1, &mut v2, &mut v3);
            self.camera.vertex_camera_to_screen_space(&mut v1_new_2);
            self.fill_triangle(v1_new_1, v2, v3, &normal, color, scene_obj, scene_obj_index);
            self.fill_triangle(v1_new_1, v1_new_2, v3, &normal, color, scene_obj, scene_obj_index);
        } else if v3.x > 0.0 { // 1 vertex in view
            let q = (NEAR_PLANE - v2.x) / (v3.x - v2.x);
            let mut v2_new = v2 + (v3 - v2) * q;
            let q = (NEAR_PLANE - v1.x) / (v3.x - v1.x);
            let mut v1_new = v1 + (v3 - v1) * q;

            self.camera.vertices_camera_to_screen_space(&mut v1_new, &mut v2_new, &mut v3);
            self.fill_triangle(v1_new, v2_new, v3, &normal, color, scene_obj, scene_obj_index);
        } else { // no vertices in view
            return;
        }
    }


    fn fill_triangle(&mut self, mut v1: Vec3, mut v2: Vec3, mut v3: Vec3, normal: &Vec3, color: Vec3, scene_obj: &SceneObject, scene_obj_index: usize) {
        // depth calculations from https://www.scratchapixel.com/lessons/3d-basic-rendering/rasterization-practical-implementation/visibility-problem-depth-buffer-depth-interpolation.html#:~:text=As%20previously%20mentioned%2C%20the%20correct,z%20%3D%201%20V%200.

        let properties = &scene_obj.mesh.properties;

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

        let looking_at_selected = if let Some(selected_index) = self.selected_index {
            selected_index == scene_obj_index
        } else {
            false
        };

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

                        if !looking_at_selected && x == self.camera.width / 2 && y == self.camera.height / 2 {
                            self.looking_at = Some((scene_obj_index, world_pos));
                        }

                        if looking_at_selected && (x == left || x == right || y == top as usize) {
                            let edge_color = shift_color(color);
                            self.zbuf.set_depth(x, y, depth);
                            self.pixel_buf.set_pixel(x, y, edge_color);
                        } else if properties.is_light {
                            self.zbuf.set_depth(x, y, depth);
                            self.pixel_buf.set_pixel(x, y, color);
                        } else {
                            let sky_color = self.get_sky_color(normal);

                            // start as ambient light
                            let mut blended_color = properties.ambient * Vec3::mul_elementwise_of(sky_color, color);

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
                            }
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

                        if !looking_at_selected && x == self.camera.width / 2 && y == self.camera.height / 2 {
                            self.looking_at = Some((scene_obj_index, world_pos));
                        }

                        if looking_at_selected && (x == left || x == right || y == bottom as usize) {
                            let edge_color = shift_color(color);
                            self.zbuf.set_depth(x, y, depth);
                            self.pixel_buf.set_pixel(x, y, edge_color);
                        } else if properties.is_light {
                            self.zbuf.set_depth(x, y, depth);
                            self.pixel_buf.set_pixel(x, y, color);
                        } else {
                            let sky_color = self.get_sky_color(normal);

                            // start as ambient light
                            let mut blended_color = properties.ambient * Vec3::mul_elementwise_of(sky_color, color);

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
                            }
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

    pub fn sort_meshes_by_distance_to_camera(&mut self) {
        let camera_pos = self.camera.pos;
        self.scene_objects.borrow_mut().sort_by(|a, b| {
            let d1 = (a.mesh.center - camera_pos).len_squared();
            let d2 = (b.mesh.center - camera_pos).len_squared();
            return d1.total_cmp(&d2);
        });
    }

    pub fn add_scene_objs_to_shadow_maps(&mut self, scene_objs: &Vec<SceneObject>) {
        for light in self.lights.iter_mut() {
            light.add_scene_objects_to_shadow_map(scene_objs);
        }
    }

    pub fn add_scene_object(&mut self, scene_obj: SceneObject) {
        self.scene_objects.borrow_mut().push(scene_obj);
    }

    pub fn get_lights(&self) -> &Vec<Light> {
        return &self.lights;
    }
    pub fn get_rt_lights(&self) -> &Vec<Box<dyn Hittable>> {
        return &self.rt_lights;
    }

    pub fn extract_lights_from_scene_objects(&mut self) {
        self.lights = self
            .scene_objects
            .borrow()
            .iter()
            .flat_map(|s| s.lights.clone())
            .collect();
        self.rt_lights = self
            .scene_objects
            .borrow()
            .iter()
            .filter(|s| s.is_light())
            .flat_map(|s| s.hittables.iter().map(|h| h.clone_box()))
            .collect();
    }

    pub fn recalculate_shadow_maps(&mut self) {
        for light in self.lights.iter_mut() {
            light.clear_shadow_map();
            light.add_scene_objects_to_shadow_map(&self.scene_objects.borrow());
        }
    }

    pub fn rebuild_bvh(&mut self) {
        let rt_objects = self
            .scene_objects
            .borrow()
            .iter()
            .flat_map(|s| s.hittables.iter().map(|h| h.clone_box()))
            .collect();
        self.bvh = Some(BVHNode::new(rt_objects));
    }
}