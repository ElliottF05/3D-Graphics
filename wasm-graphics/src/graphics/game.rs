use std::{cell::RefCell, collections::HashSet, f32::consts::{E, PI}, sync::RwLock};

use web_sys::console;

use rayon::prelude::*;

use crate::{console_error, console_log, console_warn, utils::{math::{degrees_to_radians, radians_to_degrees, Vec3}, utils::{clamp_color, gamma_correct_color, get_time, shift_color}}, wasm::wasm::{js_update_dof_strength, js_update_focal_distance, js_update_follow_camera, js_update_fov, js_update_game_status, js_update_selected_obj_mat_props, MaterialProperties}};

use super::{buffers::{PixelBuf, ZBuffer}, camera::Camera, lighting::Light, mesh::{Mesh, PhongProperties}, ray_tracing::{bvh::{BVHNode, FlattenedBVH}, hittable::Hittable, material::{Dielectric, Lambertian, Material, Metal}}, scene_object::SceneObject};

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum GameStatus {
    RasterizingNoLighting,
    RasterizingWithLighting,
    RayTracing,
    Paused,
}


pub struct Game {
    pub scene_objects: RwLock<Vec<SceneObject>>,
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

    pub looking_at: RwLock<Option<(usize, Vec3)>>,
    pub follow_camera: bool,

    pub status: GameStatus,
    pub selected_object_index: Option<usize>,
    pub ray_samples_accumulated: usize,

    // ray-tracing variables
    // pub bvh: Option<BVHNode>,
    pub bvh: Option<FlattenedBVH>,
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

            scene_objects: RwLock::new(Vec::new()),
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

            looking_at: RwLock::new(None),
            follow_camera: false,

            status: GameStatus::RasterizingNoLighting,
            selected_object_index: None,
            ray_samples_accumulated: 0,

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
        // game.create_rt_test_scene_cornell_metal();

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

        // Update JS initial states where needed
        js_update_fov(radians_to_degrees(game.camera.get_fov()));
        js_update_focal_distance(game.focus_dist);
        js_update_dof_strength(game.defocus_angle);

        return game;
    }


    pub fn game_loop(&mut self) {

        // process these no matter the game state
        self.process_all_input();

        match self.status {
            GameStatus::RasterizingNoLighting => {
                self.pre_raster_render_logic();
                self.render_frame();
                // self.gamma_correct_post_processing(); // already done in wasm.rs by get_gamma_corrected_buf_as_u8
            },
            GameStatus::RasterizingWithLighting => {
                self.pre_raster_render_logic();
                self.render_frame();
                self.depth_of_field_post_processing();
                // self.gamma_correct_post_processing(); // already done in wasm.rs by get_gamma_corrected_buf_as_u8
            },
            GameStatus::RayTracing => {
                self.render_ray_tracing();
            },
            GameStatus::Paused => {

            }
        }
    }

    fn set_game_status(&mut self, status: GameStatus) {
        console_log!("WASM: Game status changed from {:?} to {:?}", self.status, status);
        self.status = status;

        // 0 = Rasterizing, 1 = Editing, 2 = RayTracing
        let game_status_number = match self.status {
            GameStatus::RasterizingWithLighting => 0,
            GameStatus::RasterizingNoLighting => 1,
            GameStatus::RayTracing => 2,
            GameStatus::Paused => 0, // TODO: check this
        };
        js_update_game_status(game_status_number);
    }

    pub fn set_fov(&mut self, fov: f32) {
        self.camera.set_fov(fov);
        let fov_degrees = radians_to_degrees(fov);
        js_update_fov(fov_degrees);
    }

    pub fn set_focal_dist(&mut self, dist: f32) {
        self.focus_dist = dist;
        js_update_focal_distance(dist);
    }
    pub fn set_defocus_angle(&mut self, angle: f32) {
        self.defocus_angle = angle;
        js_update_dof_strength(angle);
    }

    pub fn enter_edit_mode(&mut self) {
        self.deselect_object();
        self.set_game_status(GameStatus::RasterizingNoLighting);
    }

    pub fn enter_ray_tracing_mode(&mut self) {
        js_update_game_status(2);
        self.status = GameStatus::RayTracing;
        self.ray_samples_accumulated = 0;
        self.rt_start_time = get_time();

        if self.bvh.is_none() {
            self.rebuild_bvh();
        }
    }

    pub fn stop_ray_tracing(&mut self) {
        match self.status {
            GameStatus::RayTracing => {
                self.set_game_status(GameStatus::RasterizingNoLighting);
            },
            _ => {
                console_error!("Game::stop_ray_tracing() called but not in RayTracing state, got {:?}", self.status);
                return;
            }
        }
    }

    pub fn exit_edit_mode(&mut self) {
        console_log!("game.rs: exit_edit_mode()");
        self.extract_lights_from_scene_objects();
        self.recalculate_shadow_maps();
        self.set_game_status(GameStatus::RasterizingWithLighting);
        self.deselect_object();
    }

    pub fn set_follow_camera(&mut self, follow: bool) {
        self.follow_camera = follow;
        js_update_follow_camera(follow);
    }

    pub fn set_selected_object_material_properties(&mut self, props: MaterialProperties) {
        if self.status == GameStatus::RasterizingNoLighting {
            if let Some(selected_index) = self.selected_object_index {
                console_log!("WASM: Set selected object material properties with props: {:?}", props);

                // let selected_obj = &mut self.scene_objects.borrow_mut()[selected_index];
                let selected_obj = &mut self.scene_objects.write().unwrap()[selected_index];
                let color = Vec3::new(props.r, props.g, props.b);
                let material_type = props.material_type;
                let extra_prop = props.extra_prop;

                selected_obj.set_color(color);
                selected_obj.set_material_properties(material_type, extra_prop, color);

                self.bvh = None; // invalidate bvh if obj is changed

                let props = self.parse_selected_obj_mat_props(selected_obj);
                js_update_selected_obj_mat_props(Some(props));
            } else {
                console_error!("Game::set_selected_object_material_properties() called but no object is selected");
            }
        } else {
            console_error!("Game::set_selected_object_material_properties() called but not in EditMode");
        }
    }

    pub fn translate_selected_obj(&mut self, x: f32, y: f32, z: f32) {
        if self.status == GameStatus::RasterizingNoLighting {
            if let Some(selected_index) = self.selected_object_index {
                // let selected_obj = &mut self.scene_objects.borrow_mut()[selected_index];
                let selected_obj = &mut self.scene_objects.write().unwrap()[selected_index];
                let offset = Vec3::new(x,y,z);
                selected_obj.translate_by(offset);
                self.bvh = None; // invalidate bvh if obj is changed
            } else {
                console_error!("Game::translate_selected_obj() called but no object is selected");
            }
        } else {
            console_error!("Game::translate_selected_obj() called but not in EditMode with obj selected, got GameStatus: {:?}", self.status);
        }
    }

    pub fn rotate_selected_obj(&mut self, x: f32, y: f32, z: f32) {
        let x_rad = degrees_to_radians(x);
        let y_rad = degrees_to_radians(y);
        let z_rad = degrees_to_radians(z);
        if self.status == GameStatus::RasterizingNoLighting {
            if let Some(selected_index) = self.selected_object_index {
                // let selected_obj = &mut self.scene_objects.borrow_mut()[selected_index];
                let selected_obj = &mut self.scene_objects.write().unwrap()[selected_index];
                selected_obj.rotate_around_center(z_rad, y_rad);
                self.bvh = None; // invalidate bvh if obj is changed
            } else {
                console_error!("Game::rotate_selected_obj() called but no object is selected");
            }
        } else {
            console_error!("Game::rotate_selected_obj() called but not in EditMode with obj selected, got GameStatus: {:?}", self.status);
        }
    }

    pub fn scale_selected_obj(&mut self, scale_factor: f32) {
        if self.status == GameStatus::RasterizingNoLighting {
            if let Some(selected_index) = self.selected_object_index {
                // let selected_obj = &mut self.scene_objects.borrow_mut()[selected_index];
                let selected_obj = &mut self.scene_objects.write().unwrap()[selected_index];
                selected_obj.scale_by(scale_factor);
                self.bvh = None; // invalidate bvh if obj is changed
            } else {
                console_error!("Game::scale_selected_obj() called but no object is selected");
            }
        } else {
            console_error!("Game::scale_selected_obj() called but not in EditMode with obj selected, got GameStatus: {:?}", self.status);
        }
    }

    pub fn add_sphere(&mut self, radius: f32) {
        if self.status == GameStatus::RasterizingNoLighting {
            let mut new_sphere = SceneObject::new_sphere(
                Vec3::new(0.0, 0.0, 0.0),
                radius,
                Vec3::new(0.7, 0.7, 0.7),
                3,
                SceneObject::new_diffuse_mat(),
            );

            if let Some((_, looking_at_pos)) = *self.looking_at.read().unwrap() {
                new_sphere.translate_to(looking_at_pos);
            } else {
                let mut looking_at_pos = Vec3::new(self.camera.width as f32 / 2.0, self.camera.height as f32 / 2.0, 8.0 * new_sphere.mesh.radius);
                self.camera.vertex_screen_to_world_space(&mut looking_at_pos);
                new_sphere.translate_to(looking_at_pos);
            }

            // self.scene_objects.borrow_mut().push(new_sphere);
            self.scene_objects.write().unwrap().push(new_sphere);
            self.bvh = None; // invalidate bvh if obj is added
        } else {
            console_error!("Game::add_sphere() called but not in Rasterizing state");
        }
    }

    pub fn add_box(&mut self, x: f32, y: f32, z: f32) {
        if self.status == GameStatus::RasterizingNoLighting {
            let mut new_box = SceneObject::new_box_from_side_lengths(
                Vec3::new(0.0, 0.0, 0.0),
                x, y, z,
                Vec3::new(0.7, 0.7, 0.7),
                SceneObject::new_diffuse_mat(),
            );

            if let Some((_, looking_at_pos)) = *self.looking_at.read().unwrap() {
                new_box.translate_to(looking_at_pos);
            } else {
                let mut looking_at_pos = Vec3::new(self.camera.width as f32 / 2.0, self.camera.height as f32 / 2.0, 8.0 * new_box.mesh.radius);
                self.camera.vertex_screen_to_world_space(&mut looking_at_pos);
                new_box.translate_to(looking_at_pos);
            }

            // self.scene_objects.borrow_mut().push(new_box);
            self.scene_objects.write().unwrap().push(new_box);
            self.bvh = None; // invalidate bvh if obj is added
        } else {
            console_error!("Game::add_sphere() called but not in Rasterizing state");
        }
    }

    fn process_all_input(&mut self) {

        match self.status {
            GameStatus::RasterizingNoLighting => {
                self.process_rasterization_input();
            },
            GameStatus::RasterizingWithLighting => {
                self.process_rasterization_input();
            },
            _ => {} 
        }

        if self.keys_pressed_last_frame.contains("p") {
            console_log!("Pausing or unpausing");
            if self.status == GameStatus::Paused {
                self.status = GameStatus::RasterizingNoLighting;
            } else {
                self.status = GameStatus::Paused;
            }
        }
        if self.keys_pressed_last_frame.contains("r") {
            self.enter_ray_tracing_mode();
        }

        self.keys_pressed_last_frame.clear();
        self.mouse_clicked_last_frame = false;
    }

    fn process_rasterization_input(&mut self) {
        
        if self.status != GameStatus::RasterizingNoLighting && self.status != GameStatus::RasterizingWithLighting {
            console_error!("Game::process_rasterization_input() called but not in Rasterizing state");
            return;
        }

        self.process_movement_input();
        if self.mouse_clicked_last_frame {

            match self.status {
                GameStatus::RasterizingWithLighting => { // if NOT in edit mode
                    console_warn!("Not in edit mode, so not selecting object")
                },
                GameStatus::RasterizingNoLighting => {
                    let looking_at = {
                        *self.looking_at.read().unwrap()
                    };
                    if let Some((looking_at_index, _)) = looking_at { // if clicked on something
                        if let Some(selected_index) = self.selected_object_index {
                            if looking_at_index == selected_index { // if clicked on already-selected object
                                self.deselect_object();
                            } else { // if clicked on non-selected object
                                self.select_object(looking_at_index);
                            }
                        } else { // if clicked on something and nothing selected
                            self.select_object(looking_at_index);
                        }
                    } else { // if clicked on nothing
                        self.deselect_object();
                    }
                },
                _ => {
                    console_error!("In process_rasterization_input, got unreachable game status {:?}", self.status);
                    unreachable!();
                }
            }
        }
    }

    fn select_object(&mut self, index: usize) {
        match self.status {
            GameStatus::RasterizingNoLighting => {
                console_log!("WASM: Selected object with index: {}", index);
                self.selected_object_index = Some(index);
                self.follow_camera = false;
                // let selected_obj = &self.scene_objects.borrow()[index];
                let selected_obj = &self.scene_objects.read().unwrap()[index];
                // notify JS of changes:
                let props = self.parse_selected_obj_mat_props(selected_obj);
                js_update_follow_camera(false);
                js_update_selected_obj_mat_props(Some(props));
                js_update_game_status(1); // this is just for redundancy
            },
            _ => {
                console_error!("Game::select_object() called but not in RasterizingNoLighting state, got {:?}", self.status);
                return;
            }
        }

    }

    fn parse_selected_obj_mat_props(&self, selected_obj: &SceneObject) -> MaterialProperties {
        let props = MaterialProperties {
            mat_is_editable: selected_obj.mat_is_editable,
            r: selected_obj.mesh.colors[0].x,
            g: selected_obj.mesh.colors[0].y,
            b: selected_obj.mesh.colors[0].z,
            material_type: selected_obj.get_material_number(),
            extra_prop: selected_obj.get_material_extra_prop(),
        };
        return props;
    }

    pub fn deselect_object(&mut self) {
        console_log!("WASM: Deselected object");
        self.follow_camera = false;
        self.selected_object_index = None;
        
        // notify JS of changes:
        js_update_follow_camera(false);
        js_update_selected_obj_mat_props(None);
    }

    pub fn delete_selected_object(&mut self) {
        // Only allow deletion if in RasterizingNoLighting state
        match self.status {
            GameStatus::RasterizingNoLighting => {
                if let Some(selected_index) = self.selected_object_index {
                    console_log!("Deleting object with index: {}", selected_index);
                    // self.scene_objects.borrow_mut().remove(selected_index);
                    self.scene_objects.write().unwrap().remove(selected_index);
                    self.deselect_object();
                    self.bvh = None; // invalidate bvh if obj is deleted
                } else {
                    console_error!("Game::delete_selected_object() called while in edit mode but no object is selected");
                    return;
                }
            },
            _ => {
                console_error!("Game::delete_selected_object() called but not in RasterizingNoLighting state, got {:?}", self.status);
                return;
            }
        }
    }

    fn process_movement_input(&mut self) {
        const MOVE_SPEED: f32 = 0.05;
        const ROTATE_SPEED: f32 = 0.005;
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
        // if self.keys_currently_pressed.contains("ArrowLeft") {
        //     d_theta_z += KEY_ROTATE_SPEED;
        // }
        // if self.keys_currently_pressed.contains("ArrowRight") {
        //     d_theta_z -= KEY_ROTATE_SPEED;
        // }
        // if self.keys_currently_pressed.contains("ArrowUp") {
        //     d_theta_y += KEY_ROTATE_SPEED;
        // }
        // if self.keys_currently_pressed.contains("ArrowDown") {
        //     d_theta_y -= KEY_ROTATE_SPEED;
        // }
        d_theta_z -= self.mouse_move.x * ROTATE_SPEED;
        d_theta_y -= self.mouse_move.y * ROTATE_SPEED;
        self.mouse_move = Vec3::new(0.0, 0.0, 0.0);

        self.camera.set_theta_y(self.camera.get_theta_y() + d_theta_y);
        self.camera.set_theta_z(self.camera.get_theta_z() + d_theta_z);
    }

    fn pre_raster_render_logic(&mut self) {
        if self.follow_camera && self.status == GameStatus::RasterizingNoLighting {
            if let Some(selected_index) = self.selected_object_index {
                let selected_obj = &mut self.scene_objects.write().unwrap()[selected_index];
                if let Some((_, looking_at_pos)) = *self.looking_at.read().unwrap(){
                    console_log!("looking at object, translating to {:?}", looking_at_pos);
                    selected_obj.translate_to(looking_at_pos);
                } else {
                    let mut looking_at_pos = Vec3::new(self.camera.width as f32 / 2.0, self.camera.height as f32 / 2.0, 8.0 * selected_obj.mesh.radius);
                    self.camera.vertex_screen_to_world_space(&mut looking_at_pos);
                    console_log!("not looking at anything, translating to {:?}", looking_at_pos);
                    selected_obj.translate_to(looking_at_pos);
                }
                self.bvh = None; // invalidate bvh if obj is moved
            }
        }
        *self.looking_at.write().unwrap() = None;
    }

    #[inline(never)]
    fn depth_of_field_post_processing(&mut self) {
        // first ver: 12 ms per frame

        // using prefix sums:
        // 6 ms per frame?

        // parallelizing:
        // 4 ms per frame?

        let focal_dist = self.focus_dist;
        let aperture_factor = 10.0 * self.defocus_angle / self.camera.get_fov();

        if aperture_factor <= 0.0 {
            return;
        }

        let width = self.pixel_buf.width;
        let height = self.pixel_buf.height;

        let mut temp_pixels = Vec::with_capacity(width * height);
        for y in 0..height {
            let guard = self.pixel_buf.get_row_guard(y);
            let pixel_row = guard.lock().unwrap();
            temp_pixels.extend_from_slice(&pixel_row);
        }

        // use prefix sums
        temp_pixels.par_chunks_mut(width).for_each(|row| {
            for x in 1..width {
                let prev = row[x - 1];
                row[x] += prev;
            }
        });
        for y in 1..height {
            for x in 0..width {
                let prev = temp_pixels[(y - 1) * width + x];
                temp_pixels[y * width + x] += prev;
            }
        }

        let new_pixel_buf = PixelBuf::new(width, height);
        
        (0..height).into_par_iter().for_each(|y| {
            let mut new_pixel_row = new_pixel_buf.get_row_guard(y).lock().unwrap();
            let old_pixel_row = self.pixel_buf.get_row_guard(y).lock().unwrap();
            let zbuf_row = self.zbuf.get_row_guard(y).lock().unwrap();
            for x in 0..width {
                let depth = zbuf_row[x];

                let coc_radius = ((depth - focal_dist).abs() * aperture_factor)
                    .round()
                    .max(0.0) as i32;

                let curr_pixel_color = old_pixel_row[x];

                if coc_radius < 1 { // pixel is in focus
                    new_pixel_row[x] = curr_pixel_color;
                } else { // pixel is out of focus
                    let kernel_radius = coc_radius.min(5); // clamp for performance

                    let min_x = (x as i32 - kernel_radius).max(0) as usize;
                    let max_x = (x as i32 + kernel_radius).min(width as i32 - 1) as usize;
                    let min_y = (y as i32 - kernel_radius).max(0) as usize;
                    let max_y = (y as i32 + kernel_radius).min(height as i32 - 1) as usize;

                    let num_samples = ((max_x - min_x + 1) * (max_y - min_y + 1)) as f32;

                    let a = temp_pixels[max_y * width + max_x];
                    let b = if min_x > 0 {temp_pixels[max_y * width + min_x - 1]} else {Vec3::zero()};
                    let c = if min_y > 0 {temp_pixels[(min_y - 1) * width + max_x]} else {Vec3::zero()};
                    let d = if min_x > 0 && min_y > 0 {temp_pixels[(min_y - 1) * width + min_x - 1]} else {Vec3::zero()};

                    let new_color = (a - b - c + d) / num_samples;

                    new_pixel_row[x] = new_color;
                }
            }
        });

        // set pixel buf to new_pixel_buf
        self.pixel_buf = new_pixel_buf;


        // let output_pixel_buf = 
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
        // let scene_objects = self.scene_objects.take();
        let scene_objects = self.scene_objects.read().unwrap();

        // opaque objects
        scene_objects.par_iter().enumerate().for_each(|(scene_obj_index, scene_obj)| {
            let mesh = &scene_obj.mesh;
            if mesh.properties.alpha == 1.0 {
                let vertices = &mesh.vertices;
                let transformed_vertices = self.camera.vertices_world_to_camera_space(&vertices);
                let indices = &mesh.indices;
                let colors = &mesh.colors;
                let normals = &mesh.normals;

                if colors.len() > 200 {
                    (0..colors.len()).into_par_iter().for_each(|i| {
                        let v1 = transformed_vertices[indices[i*3]];
                        let v2 = transformed_vertices[indices[i*3+1]];
                        let v3 = transformed_vertices[indices[i*3+2]];
                        let color = colors[i];
                        let normal = normals[i];
                        self.render_triangle_from_transformed_vertices(v1, v2, v3, normal, color, &scene_obj, scene_obj_index);
                    });
                } else {
                    for i in 0..colors.len() {
                        let v1 = transformed_vertices[indices[i*3]];
                        let v2 = transformed_vertices[indices[i*3+1]];
                        let v3 = transformed_vertices[indices[i*3+2]];
                        let color = colors[i];
                        let normal = normals[i];
                        self.render_triangle_from_transformed_vertices(v1, v2, v3, normal, color, &scene_obj, scene_obj_index);
                    }
                }
            }
        });

        // transparent objects
        scene_objects.par_iter().rev().enumerate().for_each(|(idx, scene_obj)| {
            let scene_obj_index = scene_objects.len() - 1 - idx;
            let mesh = &scene_obj.mesh;
            if mesh.properties.alpha < 1.0 {
                let vertices = &mesh.vertices;
                let transformed_vertices = self.camera.vertices_world_to_camera_space(&vertices);
                let indices = &mesh.indices;
                let colors = &mesh.colors;
                let normals = &mesh.normals;
                
                if colors.len() > 200 {
                    (0..colors.len()).into_par_iter().for_each(|i| {
                        let v1 = transformed_vertices[indices[i*3]];
                        let v2 = transformed_vertices[indices[i*3+1]];
                        let v3 = transformed_vertices[indices[i*3+2]];
                        let color = colors[i];
                        let normal = normals[i];
                        self.render_triangle_from_transformed_vertices(v1, v2, v3, normal, color, &scene_obj, scene_obj_index);
                    });
                } else {
                    for i in 0..colors.len() {
                        let v1 = transformed_vertices[indices[i*3]];
                        let v2 = transformed_vertices[indices[i*3+1]];
                        let v3 = transformed_vertices[indices[i*3+2]];
                        let color = colors[i];
                        let normal = normals[i];
                        self.render_triangle_from_transformed_vertices(v1, v2, v3, normal, color, &scene_obj, scene_obj_index);
                    }
                }
            }
        });

        // self.scene_objects.replace(scene_objects);

        let t2 = get_time();
        // console_log!("Frame time: {}", t2 - t1);
    }

    fn render_triangle(&self, mut v1: Vec3, mut v2: Vec3, mut v3: Vec3, color: Vec3, scene_obj: &SceneObject, scene_obj_index: usize) {
        // render triangle from transformed vertices
        let normal = (v3 - v1).cross(v2 - v1).normalized();
        self.camera.three_vertices_world_to_camera_space(&mut v1, &mut v2, &mut v3);
        self.render_triangle_from_transformed_vertices(v1, v2, v3, normal, color, scene_obj, scene_obj_index);
    }

    fn render_triangle_from_transformed_vertices(&self, mut v1: Vec3, mut v2: Vec3, mut v3: Vec3, mut normal: Vec3, color: Vec3, scene_obj: &SceneObject, scene_obj_index: usize) {

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
            self.fill_triangle(v1, v2, v3, normal, color, scene_obj, scene_obj_index);
        } else if v2.x > 0.0 { // 2 vertices in view
            let q = (NEAR_PLANE - v2.x) / (v1.x - v2.x);
            let mut v1_new_1 = v2 + (v1 - v2) * q;
            let q = (NEAR_PLANE - v3.x) / (v1.x - v3.x);
            let mut v1_new_2 = v3 + (v1 - v3) * q;

            self.camera.vertices_camera_to_screen_space(&mut v1_new_1, &mut v2, &mut v3);
            self.camera.vertex_camera_to_screen_space(&mut v1_new_2);
            self.fill_triangle(v1_new_1, v2, v3, normal, color, scene_obj, scene_obj_index);
            self.fill_triangle(v1_new_1, v1_new_2, v3, normal, color, scene_obj, scene_obj_index);
        } else if v3.x > 0.0 { // 1 vertex in view
            let q = (NEAR_PLANE - v2.x) / (v3.x - v2.x);
            let mut v2_new = v2 + (v3 - v2) * q;
            let q = (NEAR_PLANE - v1.x) / (v3.x - v1.x);
            let mut v1_new = v1 + (v3 - v1) * q;

            self.camera.vertices_camera_to_screen_space(&mut v1_new, &mut v2_new, &mut v3);
            self.fill_triangle(v1_new, v2_new, v3, normal, color, scene_obj, scene_obj_index);
        } else { // no vertices in view
            return;
        }
    }


    fn fill_triangle(&self, mut v1: Vec3, mut v2: Vec3, mut v3: Vec3, normal: Vec3, color: Vec3, scene_obj: &SceneObject, scene_obj_index: usize) {
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

        let width = self.camera.width as f32;
        let height = self.camera.height as f32;

        // calculate slopes
        let slope1 = (v2.x - v1.x) / (v2.y - v1.y); // slope of line from v1 to v2
        let slope2 = (v3.x - v1.x) / (v3.y - v1.y); // slope of line from v1 to v3
        let slope3 = (v3.x - v2.x) / (v3.y - v2.y); // slope of line from v2 to v3

        if v1.y == v3.y { // triangle has no height
            return;
        }

        let looking_at_selected = self.status == GameStatus::RasterizingNoLighting && {
            if let Some(selected_index) = self.selected_object_index {
                selected_index == scene_obj_index
            } else {
                false
            }
        };

        // calculate starting and ending x values
        let top = v1.y.ceil().max(0.0);
        let bottom = v2.y.floor().min(height - 1.0);

        // fill top half
        if v1.y != v2.y && bottom >= 0.0 {
            for y in (top as usize)..=(bottom as usize) {
                let x1 = slope1 * (y as f32 - v1.y) + v1.x;
                let x2 = slope2 * (y as f32 - v1.y) + v1.x;

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

                match self.status {
                    GameStatus::RasterizingNoLighting => {
                        self.fill_triangle_scanline_row_no_lighting(y, x1, x2, left, right, inv_left_depth, inv_right_depth, looking_at_selected, top as usize, color, normal, scene_obj, scene_obj_index);
                    },
                    GameStatus::RasterizingWithLighting => {
                        self.fill_triangle_scanline_row_with_lighting(y, x1, x2, left, right, inv_left_depth, inv_right_depth, color, normal, scene_obj);
                    },
                    _ => {
                        console_error!("Game::fill_triangle() called but not in Rasterizing state, got {:?}", self.status);
                        return;
                    }
                }
            }
        }


        // calculate starting and ending x values (for bottom half)
        let top = v2.y.ceil().max(0.0);
        let bottom = v3.y.floor().min(height - 1.0);

        // fill bottom half
        if v2.y != v3.y && bottom >= 0.0 {
            for y in (top as usize)..=(bottom as usize) {
                let x1 = slope3 * (y as f32 - v2.y) + v2.x;
                let x2 = slope2 * (y as f32 - v1.y) + v1.x;
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

                match self.status {
                    GameStatus::RasterizingNoLighting => {
                        self.fill_triangle_scanline_row_no_lighting(y, x1, x2, left, right, inv_left_depth, inv_right_depth, looking_at_selected, bottom as usize, color, normal, scene_obj, scene_obj_index);
                    },
                    GameStatus::RasterizingWithLighting => {
                        self.fill_triangle_scanline_row_with_lighting(y, x1, x2, left, right, inv_left_depth, inv_right_depth, color, normal, scene_obj);
                    },
                    _ => {
                        console_error!("Game::fill_triangle() called but not in Rasterizing state, got {:?}", self.status);
                        return;
                    }
                }
            }
        }
    }

    fn fill_triangle_scanline_row_with_lighting(&self, y: usize, x1: f32, x2: f32, left: usize, right: usize, inv_left_depth: f32, inv_right_depth: f32, color: Vec3, normal: Vec3, scene_obj: &SceneObject) {
        let properties = scene_obj.mesh.properties;
        let mut zbuf_row = self.zbuf.get_row_guard(y as usize).lock().unwrap();
        let mut pixel_row = self.pixel_buf.get_row_guard(y as usize).lock().unwrap();
        for x in left..=right {

            let q3 = (x as f32 - x1) / (x2 - x1);
            let inv_depth = inv_left_depth * (1.0 - q3) + inv_right_depth * q3;
            let depth = 1.0 / inv_depth;
            let bias = if properties.alpha == 1.0 {0.0} else {0.01};

            if depth - bias < zbuf_row[x] {

                let mut world_pos = Vec3::new(x as f32, y as f32, depth);
                self.camera.vertex_screen_to_camera_space(&mut world_pos);
                self.camera.vertex_camera_to_world_space(&mut world_pos);

                if properties.is_light {
                    zbuf_row[x] = depth;
                    pixel_row[x] = color;
                } else {
                    let sky_color = self.get_sky_color(&normal);

                    // start as ambient light
                    let mut blended_color = properties.ambient * Vec3::mul_elementwise_of(sky_color, color);

                    for light in &self.lights {
                        blended_color += light.get_lighting_at(&world_pos, &self.camera.pos, &normal, color, &properties);
                    }

                    if properties.alpha == 1.0 {
                        zbuf_row[x] = depth;
                        pixel_row[x] = blended_color;
                    } else {
                        // alpha blending, don't set depth
                        let old_color = pixel_row[x];
                        blended_color = blended_color * properties.alpha + old_color * (1.0 - properties.alpha);
                        pixel_row[x] = blended_color;
                    }
                }
            }
        }
    }

    fn fill_triangle_scanline_row_no_lighting(&self, y: usize, x1: f32, x2: f32, left: usize, right: usize, inv_left_depth: f32, inv_right_depth: f32, looking_at_selected: bool, y_extremity: usize, color: Vec3, normal: Vec3, scene_obj: &SceneObject, scene_obj_index: usize) {
        let properties = scene_obj.mesh.properties;
        let mut zbuf_row = self.zbuf.get_row_guard(y as usize).lock().unwrap();
        let mut pixel_row = self.pixel_buf.get_row_guard(y as usize).lock().unwrap();

        for x in left..=right {

            let q3 = (x as f32 - x1) / (x2 - x1);
            let inv_depth = inv_left_depth * (1.0 - q3) + inv_right_depth * q3;
            let depth = 1.0 / inv_depth;
            let bias = if properties.alpha == 1.0 {0.0} else {0.01};

            if depth - bias < zbuf_row[x] {

                let mut world_pos = Vec3::new(x as f32, y as f32, depth);
                self.camera.vertex_screen_to_camera_space(&mut world_pos);
                self.camera.vertex_camera_to_world_space(&mut world_pos);

                if !looking_at_selected && x == self.camera.width / 2 && y == self.camera.height / 2 {
                    *self.looking_at.write().unwrap() = Some((scene_obj_index, world_pos));
                }

                if looking_at_selected && (x == left || x == right || y == y_extremity) {
                    let edge_color = shift_color(color);
                    zbuf_row[x] = depth;
                    pixel_row[x] = edge_color;
                } else if properties.is_light {
                    zbuf_row[x] = depth;
                    pixel_row[x] = color;
                } else {
                    let sky_color = self.get_sky_color(&normal);

                    // start as ambient light
                    let mut blended_color = Vec3::mul_elementwise_of(sky_color, color);

                    if properties.alpha == 1.0 {
                        zbuf_row[x] = depth;
                        pixel_row[x] = blended_color;
                    } else {
                        // alpha blending, don't set depth
                        let old_color = pixel_row[x];
                        blended_color = blended_color * properties.alpha + old_color * (1.0 - properties.alpha);
                        pixel_row[x] = blended_color;
                    }
                }
            }
        }
    }

    pub fn get_sky_color(&self, dir: &Vec3) -> Vec3 {
        let a = 0.5 * (dir.z + 1.0);
        return self.min_sky_color * (1.0 - a) + self.max_sky_color * a;
    }
    pub fn clear_pixel_buf_to_sky(&mut self) {
        let height = self.pixel_buf.height;
        for y in 0..height {
            let mut pixel_row = self.pixel_buf.get_row_guard(y).lock().unwrap();
            for x in 0..pixel_row.len() {
                let mut v = Vec3::new(x as f32, y as f32, 1.0);
                self.camera.vertex_screen_to_world_space(& mut v);
                v -= self.camera.pos;
                v.normalize();
                let sky_color = self.get_sky_color(&v);
                pixel_row[x] = sky_color;
            }
        }
    }

    pub fn sort_meshes_by_distance_to_camera(&self) {
        let camera_pos = self.camera.pos;
        self.scene_objects.write().unwrap().sort_by(|a, b| {
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
        // self.scene_objects.borrow_mut().push(scene_obj);
        self.scene_objects.write().unwrap().push(scene_obj);
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
            .read()
            .unwrap()
            .iter()
            .flat_map(|s| s.lights.clone())
            .collect();
        self.rt_lights = self
            .scene_objects
            .read()
            .unwrap()
            .iter()
            .filter(|s| s.is_light())
            .flat_map(|s| s.hittables.iter().map(|h| h.clone_box()))
            .collect();
    }

    pub fn recalculate_shadow_maps(&mut self) {
        for light in self.lights.iter_mut() {
            light.clear_shadow_map();
            light.add_scene_objects_to_shadow_map(&self.scene_objects.read().unwrap());
        }
    }

    pub fn rebuild_bvh(&mut self) {
        let rt_objects = self
            .scene_objects
            .read()
            .unwrap()
            .iter()
            .flat_map(|s| s.hittables.iter().map(|h| h.clone_box()))
            .collect();
        // self.bvh = Some(BVHNode::new(rt_objects));
        self.bvh = Some(FlattenedBVH::new(rt_objects));
    }
}