use std::{cell::RefCell, f32::consts::PI, fmt::Debug, sync::RwLock};

use rayon::prelude::*;

use crate::{console_log, graphics::{buffers::{PixelBuf, ZBuffer}, camera::Camera, game::GameStatus, lighting::Light, scene_object::SceneObject}, utils::{math::{degrees_to_radians, Vec3}, utils::{gamma_correct_color, get_time, random_float, random_range, sample_circle, sample_square}}};

use super::{super::{game::Game, mesh::{Mesh, PhongProperties}}, bvh::BVHNode, hittable::{Hittable, Sphere, Triangle}, material::{Dielectric, DiffuseLight, Lambertian, Material, Metal}};

// const SAMPLES: usize = 10; // 10
// const MAX_DEPTH: usize = 10; // 10

#[derive(Debug, Clone, Default)]
pub struct Ray {
    pub origin: Vec3,
    pub direction: Vec3,
}

impl Ray {
    pub fn new(origin: Vec3, direction: Vec3) -> Self {
        Self { origin, direction }
    }
    pub fn at(&self, t: f32) -> Vec3 {
        self.origin + self.direction * t
    }
}

#[derive(Debug, Default)]
pub struct HitRecord<'a> {
    pub t: f32,
    pub pos: Vec3,
    pub normal: Vec3,
    pub front_face: bool,
    pub surface_color: Vec3,
    pub material: Option<&'a dyn Material>,
}

impl HitRecord<'_> {
    pub fn set_face_normal(&mut self, ray: &Ray, outward_normal: Vec3) {
        self.front_face = ray.direction.dot(outward_normal) < 0.0;
        self.normal = if self.front_face { outward_normal } else { -outward_normal };
    }
}

impl Game {

    pub fn render_ray_tracing(&mut self) {

        // BENCHMARKING
        // perf tips here: https://users.rust-lang.org/t/peter-shirleys-ray-tracing-in-one-weekend-implementation-in-rust/26972/11

        // benchmark conditions:
        // scene: create_rt_test_scene_spheres()
        // samples: 10
        // max_depth: 10
        // also: dev console closed in browser

        // first version after finishing rt in one weekend, no performance improvements
        // 22.65 second average

        // after removing new box clone (heap allocation) on object.hit() in favor of lifetimes (ooh)
        // 21.83 seconds avg (barely faster lol)

        // after switching recursion to iteration in ray_trace()
        // 20.49 seconds avg

        // after removing dynamic dispatch: 
        // 7.80 seconds avg!!!

        // after adding #[inline(always)] to SceneObject::hit()
        // 6.10 second avg!

        // after removing indirection for Vec3 functions (and inlining them though compiler probably did this anyway)
        // 5.47 second avg

        // after adding bvh
        // 2.92 second avg

        // next steps?
        // 1) look into improving cache locality by have the BVH tree hold indexes into a Vec<SceneObject> 
        //      - this is called a flattened bvh tree
        // 2) Multi-threading (duh)

        // New baseline (after adding MIS, russian roulette, and UI; still same test scene):
        // 2.59 second avg

        // After adding FlattenedBVH:
        // 2.398 second avg

        // After adding Rayon parallelism:
        // 0.720 second avg

        if self.bvh.is_none() {
            console_log!("No RT objects in the scene, can't raytrace!");
            return;
        }

        console_log!("Rendering ray tracing");

        let start_time = get_time();
        let samples_per_pixel_per_pass = 1;

        loop {

            self.ray_samples_accumulated += samples_per_pixel_per_pass;

            (0..self.camera.height).into_par_iter().for_each(|y| {
                let mut pixel_row = self.pixel_buf.get_row_guard(y).lock().unwrap();
                for x in 0..pixel_row.len() {

                    let mut new_color = Vec3::zero();
                    for _ in 0..samples_per_pixel_per_pass {
                        let ray = self.get_rand_ray_at_pixel_with_defocus(x, y);
                        // let ray_color = self.ray_trace(ray, self.ray_max_depth);
                        let ray_color = self.ray_trace_mis(ray, self.ray_max_depth);
                        new_color += ray_color;
                    }

                    let existing_color = pixel_row[x];
                    let updated_color = (
                        existing_color * (self.ray_samples_accumulated as f32 - samples_per_pixel_per_pass as f32)
                        + new_color * samples_per_pixel_per_pass as f32)
                        / self.ray_samples_accumulated as f32;

                    pixel_row[x] = updated_color;
                }
            });

            let curr_time = get_time();
            if curr_time - start_time > 0.5 {
                break;
            }
        }
    }

    fn ray_trace(&self, mut ray: Ray, mut depth: usize) -> Vec3 {

        // start with identity for multiplication = 1
        let mut pixel_color = Vec3::new(1.0, 1.0, 1.0);

        while depth > 0 {
            depth -= 1;
            let mut hit_record = HitRecord::default();
            let mut hit_anything = false;

            if self.bvh.as_ref().unwrap().hit(&ray, 0.001, 5000.0, &mut hit_record) {
                hit_anything = true;
            }

            if hit_anything {
                let successful_scatter;
                let attenuation;
                let scattered_ray;

                if let Some(material) = hit_record.material {

                    let emitted_color = material.emitted(&hit_record);
                    (successful_scatter, attenuation, scattered_ray) = material.scatter(&ray, &hit_record);
                    if successful_scatter {
                        pixel_color += emitted_color;
                        pixel_color.mul_elementwise_inplace(attenuation);
                        ray = scattered_ray;
                    } else {
                        return pixel_color.mul_elementwise(emitted_color);
                        // no scatter, terminate here with current emitted color
                    }

                } else {
                    return Vec3::zero(); // no material, return black
                }

            } else { // if hit_anything == false, then ray hit nothing, goes off into sky
                let sky_color = self.get_sky_color(&ray.direction.normalized());
                return pixel_color.mul_elementwise(sky_color);
            }
        }

        // max depth reached, return black
        return Vec3::zero();
    }

    fn ray_trace_mis(&self, mut ray: Ray, mut depth: usize) -> Vec3 {

        let use_direct = true;

        // throughput will hold the multiplied indirect attenuations, start with identity for multiplication = 1
        // accum color will hold the accumulated direct lighting, start with 0
        let mut throughput = Vec3::new(1.0, 1.0, 1.0);
        let mut accum_color = Vec3::zero();

        while depth > 0 {
            depth -= 1;
            let mut hit_record = HitRecord::default();
            let mut hit_anything = false;

            // russian-roulette optimization
            if depth >= self.ray_max_depth - 3 {
                // skip russian roulette on first bounces
            } else {
                // use luminance
                let lum = 0.2126*throughput.x + 0.7152*throughput.y + 0.0722*throughput.z;
                let p_continue = lum.clamp(0.05, 1.0);
                if random_float() > p_continue {
                    break;
                }
                throughput /= p_continue;
            }

            if self.bvh.as_ref().unwrap().hit(&ray, 0.001, 5000.0, &mut hit_record) {
                hit_anything = true;
            }

            if hit_anything {
                let successful_scatter;
                let attenuation;
                let scattered_ray;
                let light_sampling_option;

                if let Some(material) = hit_record.material {

                    let emitted_color = material.emitted(&hit_record);
                    (successful_scatter, attenuation, scattered_ray, light_sampling_option) = material.scatter_mis(&ray, &hit_record, self.get_rt_lights());
                    if successful_scatter {

                        let mut direct_lighting = Vec3::zero();
                        if use_direct {
                            if let Some((direct_attenuation, shadow_ray, light_dist)) = light_sampling_option {
                                let mut shadow_ray_hit_record = HitRecord::default();
                                let light_is_occluded = self.bvh.as_ref().unwrap().hit(&shadow_ray, 0.001, light_dist - 0.001, &mut shadow_ray_hit_record);
                                if !light_is_occluded {
                                    direct_lighting = direct_attenuation;
                                }
                            }
                        }

                        accum_color += throughput.mul_elementwise(emitted_color);
                        accum_color += throughput.mul_elementwise(direct_lighting);
                        throughput.mul_elementwise_inplace(attenuation);

                        ray = scattered_ray;
                    } else {
                        return accum_color + throughput.mul_elementwise(emitted_color);
                        // no scatter, terminate here with current emitted color
                    }

                } else {
                    return Vec3::zero(); // no material, return black
                }

            } else { // if hit_anything == false, then ray hit nothing, goes off into sky
                let sky_color = self.get_sky_color(&ray.direction.normalized());
                accum_color += throughput.mul_elementwise(sky_color);
                return accum_color;
            }
        }

        // max depth reached, return accumulated color
        return accum_color;
    }

    fn get_ray_at_pixel(&self, x: usize, y: usize) -> Ray {
        let origin = self.camera.pos;

        let mut v = Vec3::new(x as f32, y as f32, 1.0);
        self.camera.vertex_screen_to_world_space(&mut v);
        let direction = (v - origin).normalized();

        Ray::new(origin, direction)
    }

    fn get_rand_ray_at_pixel(&self, x: usize, y: usize) -> Ray {
        let origin = self.camera.pos;
        let (offset_x, offset_y) = sample_square(1.0);
        let mut v = Vec3::new(x as f32 + offset_x, y as f32 + offset_y, 1.0);
        self.camera.vertex_screen_to_world_space(&mut v);
        let direction = (v - origin).normalized();

        Ray::new(origin, direction)
    }

    fn get_rand_ray_at_pixel_with_defocus(&self, x: usize, y: usize) -> Ray {
        let defocus_disk_radius = (0.5 * self.defocus_angle).tan() * self.focus_dist;
        let (disk_x, disk_y) = sample_circle(defocus_disk_radius);
        let (offset_x, offset_y) = sample_square(1.0);


        let mut point_on_focus_plane = Vec3::new(x as f32 + offset_x, y as f32 + offset_y, self.focus_dist);
        self.camera.vertex_screen_to_camera_space(&mut point_on_focus_plane);

        let mut point_on_defocus_disk = Vec3::new(0.0, disk_x, disk_y);

        self.camera.vertex_camera_to_world_space(&mut point_on_focus_plane);
        self.camera.vertex_camera_to_world_space(&mut point_on_defocus_disk);

        let ray_dir = (point_on_focus_plane - point_on_defocus_disk).normalized();

        return Ray::new(point_on_defocus_disk, ray_dir);
    }

    // pub fn create_bvh_from_scene_objs(&mut self) {
    //     let hittables = self.scene_objects
    //         .borrow()
    //         .iter()
    //         .flat_map(|o| o.hittables.iter().map(|h| h.clone_box()))
    //         .collect();
    //     self.bvh = Some(BVHNode::new(hittables));
    // }

    pub fn create_rt_test_scene_spheres(&mut self) {

        self.scene_objects.write().unwrap().clear();

        self.ray_samples = 10;
        self.ray_max_depth = 10;

        let ground_material = Lambertian::default();
        let sphere = SceneObject::new_sphere(
            Vec3::new(0.0, 0.0, -1000.0),
            1000.0,
            Vec3::new(0.5, 0.5, 0.5),
            4,
            SceneObject::new_diffuse_mat(),
        );
        self.add_scene_object(sphere);

        let lim = 11; // 11

        for a in -lim..lim {
            for b in -lim..lim {
                let choose_mat = random_float();
                let center = Vec3::new(a as f32 + 0.9*random_float(), b as f32 + 0.9*random_float(), 0.2);

                if (center - Vec3::new(4.0, 0.0, 0.2)).len() > 0.9 {
                    let sphere_material: Box<dyn Material>;
                    let color;

                    let unified_mat;

                    if choose_mat < 0.8 {
                        // diffuse
                        color = Vec3::random().mul_elementwise(Vec3::random());
                        sphere_material = Lambertian::default().clone_box();
                        unified_mat = SceneObject::new_diffuse_mat();
                    } else if choose_mat < 0.95 {
                        // metal
                        color = Vec3::random() * 0.5 + Vec3::new(0.5, 0.5, 0.5);
                        let fuzz = random_range(0.0, 0.5);
                        sphere_material = Metal::new(fuzz).clone_box();
                        unified_mat = SceneObject::new_glossy_mat(fuzz)
                    } else {
                        // glass
                        color = Vec3::new(1.0, 1.0, 1.0);
                        sphere_material = Dielectric::new(1.5).clone_box();
                        unified_mat = SceneObject::new_glass_mat(0.5, 1.5);
                    }
                    
                    // let sphere = Sphere::new(center, 0.2, color, 2, sphere_material);
                    // self.add_mesh(sphere.to_mesh(2, PhongProperties::default()));
                    
                    let sphere = SceneObject::new_sphere(center, 0.2, color, 2, unified_mat);
                    self.add_scene_object(sphere);
                }
            }
        }

        // glass
        let sphere = SceneObject::new_sphere(
            Vec3::new(0.0, 0.0, 1.0), 
            1.0, 
            Vec3::new(1.0, 1.0, 1.0), 
            4,
            SceneObject::new_glass_mat(0.5, 1.5)
        );
        self.add_scene_object(sphere);

        // lambertian
        let sphere = SceneObject::new_sphere(
            Vec3::new(-4.0, 0.0, 1.0), 
            1.0 ,
            Vec3::new(0.4, 0.2, 0.1), 
            4,
            SceneObject::new_diffuse_mat(),
        );
        self.add_scene_object(sphere);

        // metal
        let sphere = SceneObject::new_sphere(
            Vec3::new(4.0, 0.0, 1.0), 
            1.0, 
            Vec3::new(0.7, 0.6, 0.5), 
            4,
            SceneObject::new_glossy_mat(0.0)
        );
        self.add_scene_object(sphere);

        self.camera.set_fov(degrees_to_radians(20.0));
        self.camera.pos = Vec3::new(13.0, 3.0, 2.0);
        self.camera.look_at(&Vec3::zero());

        self.defocus_angle = degrees_to_radians(0.6);
        self.focus_dist = 10.0;

        // set up bvh
        // self.create_bvh_from_scene_objs();
        self.js_update_ui();
        self.rebuild_bvh();
    
    }

    pub fn create_rt_test_scene_quads(&mut self) {

        self.ray_samples = 10;
        self.ray_max_depth = 10;

        let origins = vec![
            Vec3::new(-3.0, 5.0, -2.0),
            Vec3::new(-2.0, 0.0, -2.0),
            Vec3::new(3.0, 1.0, -2.0),
            Vec3::new(-2.0, 1.0, 3.0),
            Vec3::new(-2.0, 5.0, -3.0),
        ];
        let u_vectors = vec![
            Vec3::new(0.0, -4.0, 0.0),
            Vec3::new(4.0, 0.0, 0.0),
            Vec3::new(0.0, 4.0, 0.0),
            Vec3::new(4.0, 0.0, 0.0),
            Vec3::new(4.0, 0.0, 0.0),
        ];
        let v_vectors = vec![
            Vec3::new(0.0, 0.0, 4.0),
            Vec3::new(0.0, 0.0, 4.0),
            Vec3::new(0.0, 0.0, 4.0),
            Vec3::new(0.0, 4.0, 0.0),
            Vec3::new(0.0, -4.0, 0.0),
        ];
        let colors = vec![
            Vec3::new(1.0, 0.2, 0.2),
            Vec3::new(0.2, 1.0, 0.2),
            Vec3::new(0.2, 0.2, 1.0),
            Vec3::new(1.0, 0.5, 0.0),
            Vec3::new(0.2, 0.8, 0.8),
        ];

        let mut mesh_vertices = Vec::new();
        let mut mesh_colors = Vec::new();

        for i in 0..origins.len() {
            let origin = origins[i];
            let u = u_vectors[i];
            let v = v_vectors[i];
            let color = colors[i];

            mesh_vertices.push(origin);
            mesh_vertices.push(origin+u);
            mesh_vertices.push(origin+v);

            mesh_vertices.push(origin+u);
            mesh_vertices.push(origin+u+v);
            mesh_vertices.push(origin+v);

            mesh_colors.push(color);
            mesh_colors.push(color);

            // let t1 = Triangle::new_from_directions(origin, u, v, Vec3::new(1.0, 0.2, 0.2), &lambertian);
            // let t2 = Triangle::new_from_directions(origin+u+v, -u, -v, Vec3::new(1.0, 0.2, 0.2), &lambertian);
        }

        let mesh = Mesh::new_from_non_indexed(mesh_vertices, mesh_colors, PhongProperties::default());
        let scene_obj = SceneObject::new_from_mesh(mesh, Lambertian::default().clone_box(), true);
        self.add_scene_object(scene_obj);

        // self.create_bvh_from_scene_objs();
        self.rebuild_bvh();

        self.camera.set_fov(degrees_to_radians(80.0));
        self.camera.pos = Vec3::new(0.0, 9.0, 0.0);
        self.camera.look_at(&Vec3::zero());
    }

    pub fn create_rt_test_scene_simple_light(&mut self) {

        self.ray_samples = 100;
        self.ray_max_depth = 50;

        let sphere = SceneObject::new_sphere(
            Vec3::new(0.0, 0.0, -1000.0), 
            1000.0, 
            Vec3::new(0.9, 0.9, 0.9), 
            4,
            SceneObject::new_diffuse_mat()
        );
        self.add_scene_object(sphere);

        let sphere = SceneObject::new_sphere(
            Vec3::new(0.0, 0.0, 2.0), 
            2.0, 
            Vec3::new(0.9, 0.9, 0.9), 
            4,
            SceneObject::new_diffuse_mat()
        );
        self.add_scene_object(sphere);

        // let (t1, t2) = Triangle::new_quad(
        //     Vec3::new(3.0, -2.0, 1.0),
        //     Vec3::new(0.0, 0.0, 2.0),
        //     Vec3::new(2.0, 0.0, 0.0),
        //     Vec3::new(4.0, 4.0, 4.0), 
        //     &DiffuseLight::default()
        // );
        // self.add_mesh(t1.to_mesh(PhongProperties::default()));
        // self.add_mesh(t2.to_mesh(PhongProperties::default()));
        // rt_objects.push(Box::new(t1.clone()));
        // rt_objects.push(Box::new(t2.clone()));
        // self.rt_lights.push(Box::new(t1));
        // self.rt_lights.push(Box::new(t2));
        let light_rec = SceneObject::new_rectangle_light(
            Vec3::new(3.0, -2.0, 1.0),
            Vec3::new(0.0, 0.0, 2.0),
            Vec3::new(2.0, 0.0, 0.0),
            Vec3::new(4.0, 4.0, 4.0),
            0.1, 1000);
        self.add_scene_object(light_rec);

        self.extract_raster_lights_from_scene_objects();
        self.recalculate_shadow_maps();

        // self.create_bvh_from_scene_objs();
        self.rebuild_bvh();

        self.max_sky_color = Vec3::new(0.01, 0.01, 0.01);
        self.min_sky_color = Vec3::zero();

        self.camera.pos = Vec3::new(26.0, 6.0, 3.0);
        self.camera.look_at(&Vec3::new(0.0, 0.0, 2.0));

        self.camera.set_fov(degrees_to_radians(20.0));
        self.defocus_angle = 0.0;
    }

    pub fn create_rt_test_scene_cornell(&mut self) {
        self.ray_samples = 50;
        self.ray_max_depth = 10;
        self.max_sky_color = Vec3::new(0.1, 0.1, 0.1);
        self.min_sky_color = Vec3::zero();
    
        let red_color = Vec3::new(0.65, 0.05, 0.05);
        let green_color = Vec3::new(0.12, 0.45, 0.15);
        let white_color = Vec3::new(0.73, 0.73, 0.73);
        let light_color = Vec3::new(15.0, 15.0, 15.0);
    
        let lambert_mat = SceneObject::new_diffuse_mat();
    
        self.scene_objects = RwLock::new(vec![
            SceneObject::new_rectangle(
                Vec3::new(0.0, 0.0, 0.0),
                Vec3::new(0.0, 55.5, 0.0),
                Vec3::new(0.0, 0.0, 55.5),
                green_color,
                lambert_mat.clone(),
                true,
            ),
            SceneObject::new_rectangle(
                Vec3::new(55.5, 0.0, 0.0),
                Vec3::new(0.0, 0.0, 55.5),
                Vec3::new(0.0, 55.5, 0.0),
                red_color,
                lambert_mat.clone(),
                true,
            ),
            SceneObject::new_rectangle_light(
                Vec3::new(36.3, 35.2, 55.4),
                Vec3::new(0.0, -14.5, 0.0),
                Vec3::new(-17.0, 0.0, 0.0),
                light_color,
                0.1,
                1000,
            ),
            SceneObject::new_rectangle(
                Vec3::new(0.0, 0.0, 0.0),
                Vec3::new(55.5, 0.0, 0.0),
                Vec3::new(0.0, 55.5, 0.0),
                white_color,
                lambert_mat.clone(),
                true,
            ),
            SceneObject::new_rectangle(
                Vec3::new(55.5, 55.5, 55.5),
                Vec3::new(0.0, -55.5, 0.0),
                Vec3::new(-55.5, 0.0, 0.0),
                white_color,
                lambert_mat.clone(),
                true,
            ),
            SceneObject::new_rectangle(
                Vec3::new(0.0, 55.5, 0.0),
                Vec3::new(55.5, 0.0, 0.0),
                Vec3::new(0.0, 0.0, 55.5),
                white_color,
                lambert_mat.clone(),
                true,
            ),
        ]);
    
        // Left box
        let mut left_box = SceneObject::new_box_from_corners(
            Vec3::new(27.1, 29.5, 0.0),
            Vec3::new(10.6, 46.0, 33.0),
            white_color,
            lambert_mat.clone(),
        );
        left_box.rotate_around_center(degrees_to_radians(15.0), 0.0);
        self.add_scene_object(left_box);
    
        // Right box
        let mut right_box = SceneObject::new_box_from_corners(
            Vec3::new(45.1, 6.5, 0.0),
            Vec3::new(28.6, 23.0, 16.5),
            white_color,
            lambert_mat.clone(),
        );
        right_box.rotate_around_center(degrees_to_radians(-18.0), 0.0);
        self.add_scene_object(right_box);
    
        // RT Hittables
        // self.create_bvh_from_scene_objs();
        self.rebuild_bvh();
    
        // Lights for RT and rasterization
        self.extract_raster_lights_from_scene_objects();
    
        // Camera
        self.camera.set_fov(degrees_to_radians(40.0));
        self.camera.pos = Vec3::new(27.8, -80.0, 27.8);
        self.camera.look_at(&Vec3::new(27.8, 0.0, 27.8));
        self.defocus_angle = 0.0;
    
        // Shadow maps
        self.recalculate_shadow_maps();
    }


    pub fn create_rt_test_scene_cornell_metal(&mut self) {
        self.ray_samples = 50;
        self.ray_max_depth = 10;
        self.max_sky_color = Vec3::new(0.1, 0.1, 0.1);
        self.min_sky_color = Vec3::zero();
    
        let red_color = Vec3::new(0.65, 0.05, 0.05);
        let green_color = Vec3::new(0.12, 0.45, 0.15);
        let white_color = Vec3::new(0.73, 0.73, 0.73);
        let light_color = Vec3::new(25.0,  25.0, 25.0);
    
        let lambert_mat = SceneObject::new_diffuse_mat();
        let metal_mat = SceneObject::new_metal_mat(0.0);
    
        self.scene_objects = RwLock::new(vec![
            SceneObject::new_rectangle(
                Vec3::new(0.0, 0.0, 0.0),
                Vec3::new(0.0, 55.5, 0.0),
                Vec3::new(0.0, 0.0, 55.5),
                green_color,
                metal_mat.clone(),
                true,
            ),
            SceneObject::new_rectangle(
                Vec3::new(55.5, 0.0, 0.0),
                Vec3::new(0.0, 0.0, 55.5),
                Vec3::new(0.0, 55.5, 0.0),
                red_color,
                metal_mat.clone(),
                true,
            ),
            SceneObject::new_rectangle_light(
                Vec3::new(36.3, 35.2, 55.4),
                Vec3::new(0.0, -14.5, 0.0),
                Vec3::new(-17.0, 0.0, 0.0),
                light_color,
                0.1,
                1000,
            ),
            SceneObject::new_rectangle(
                Vec3::new(0.0, 0.0, 0.0),
                Vec3::new(55.5, 0.0, 0.0),
                Vec3::new(0.0, 55.5, 0.0),
                white_color,
                lambert_mat.clone(),
                true,
            ),
            SceneObject::new_rectangle(
                Vec3::new(55.5, 55.5, 55.5),
                Vec3::new(0.0, -55.5, 0.0),
                Vec3::new(-55.5, 0.0, 0.0),
                white_color,
                lambert_mat.clone(),
                true,
            ),
            SceneObject::new_rectangle(
                Vec3::new(0.0, 55.5, 0.0),
                Vec3::new(55.5, 0.0, 0.0),
                Vec3::new(0.0, 0.0, 55.5),
                white_color,
                lambert_mat.clone(),
                true,
            ),
        ]);
    
        // Left box
        let mut left_box = SceneObject::new_box_from_corners(
            Vec3::new(27.1, 29.5, 0.0),
            Vec3::new(10.6, 46.0, 33.0),
            white_color,
            metal_mat.clone(),
        );
        left_box.rotate_around_center(degrees_to_radians(15.0), 0.0);
        self.add_scene_object(left_box);
    
        // Right box
        let glass_mat = SceneObject::new_glass_mat(0.5, 1.5);
        let mut right_box = SceneObject::new_box_from_corners(
            Vec3::new(45.1, 6.5, 0.0),
            Vec3::new(28.6, 23.0, 16.5),
            white_color,
            glass_mat.clone(),
        );
        right_box.rotate_around_center(degrees_to_radians(-18.0), 0.0);
        self.add_scene_object(right_box);
    
        // RT Hittables
        // self.create_bvh_from_scene_objs();
        self.rebuild_bvh();
    
        // Lights for RT and rasterization
        self.extract_raster_lights_from_scene_objects();
    
        // Camera
        self.camera.set_fov(degrees_to_radians(40.0));
        self.camera.pos = Vec3::new(27.8, -80.0, 27.8);
        self.camera.look_at(&Vec3::new(27.8, 0.0, 27.8));
        self.defocus_angle = 0.0;
    
        // Shadow maps
        self.recalculate_shadow_maps();
    }
}