use std::{fmt::Debug};

use crate::{console_log, graphics::game::GameStatus, utils::{math::{degrees_to_radians, Vec3}, utils::{get_time, random_float, random_range, sample_circle, sample_square}}};

use super::{super::{game::Game, mesh::{Mesh, PhongProperties}}, bvh::BVHNode, hittable::{Hittable, Sphere}, material::{Dielectric, Lambertian, Material, Metal}};

const SAMPLES: usize = 10;
const MAX_DEPTH: usize = 10;

const DEFOCUS_ANGLE: f32 = degrees_to_radians(0.6); // 0.6
const FOCUS_DIST: f32 = 10.0;

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
        // scene: create_rt_test_scene()
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


        self.status = GameStatus::RayTracing;
        console_log!("Rendering ray tracing");

        let start_time = get_time();
        let start_row = self.rt_row;

        for y in start_row..self.camera.height {
            for x in 0..self.camera.width {

                let mut pixel_color = Vec3::zero();
                for _ in 0..SAMPLES {
                    let ray = self.get_rand_ray_at_pixel_with_defocus(x, y);
                    let ray_color = self.ray_trace(ray, MAX_DEPTH);
                    pixel_color += ray_color;
                }
                pixel_color /= SAMPLES as f32;
                self.pixel_buf.set_pixel(x, y, pixel_color);
            }

            let curr_time = get_time();
            let elapsed = curr_time - start_time;
            if elapsed >= 1000.0 {
                self.rt_row = y + 1;
                return;
            }
        }

        self.rt_row = 0;
        self.status = GameStatus::Paused;
        self.apply_post_processing_effects();

        let curr_time = get_time();
        let total_time = curr_time - self.rt_start_time;
        console_log!("Finished ray tracing in {} seconds", 0.001 * total_time);

        // self.apply_post_processing_effects();
    }

    fn ray_trace(&self, mut ray: Ray, mut depth: usize) -> Vec3 {

        // start with identity for multiplication = 1
        let mut pixel_color = Vec3::new(1.0, 1.0, 1.0);

        while depth > 0 {
            depth -= 1;
            let mut hit_record = HitRecord::default();
            let mut hit_anything = false;
            let mut closest_so_far = 1000.0;

            // for sphere in spheres.iter() {
            //     if sphere.hit(&ray, 0.001, closest_so_far, &mut hit_record) {
            //         closest_so_far = hit_record.t;
            //         hit_anything = true;
            //     }
            // }
            // for vertex_obj in vertex_objects.iter() {
            //     if vertex_obj.hit(&ray, 0.001, closest_so_far, &mut hit_record) {
            //         closest_so_far = hit_record.t;
            //         hit_anything = true;
            //     }
            // }

            if self.bvh.as_ref().unwrap().hit(&ray, 0.001, closest_so_far, &mut hit_record) {
                hit_anything = true;
            }

            if hit_anything {
                let successful_scatter;
                let attenuation;
                let scattered_ray;

                if let Some(material) = hit_record.material {
                    (successful_scatter, attenuation, scattered_ray) = material.scatter(&ray, &hit_record);
                    if successful_scatter {
                        pixel_color.mul_elementwise_inplace(attenuation);
                        ray = scattered_ray;
                    } else {
                        pixel_color.mul_elementwise_inplace(Vec3::zero());
                        break; // no scatter, return black
                    }
                } else {
                    pixel_color.mul_elementwise_inplace(Vec3::zero());
                    break; // no material, return black
                }

            } else { // if hit_anything == false, then ray hit nothing, goes off into sky
                let sky_color = self.get_sky_color(&ray.direction);
                pixel_color.mul_elementwise_inplace(sky_color);
                break;
            }
        }

        return pixel_color;
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
        let defocus_disk_radius = (0.5 * DEFOCUS_ANGLE).tan() * FOCUS_DIST;
        let (disk_x, disk_y) = sample_circle(defocus_disk_radius);
        let (offset_x, offset_y) = sample_square(1.0);


        let mut point_on_focus_plane = Vec3::new(x as f32 + offset_x, y as f32 + offset_y, FOCUS_DIST);
        self.camera.vertex_screen_to_camera_space(&mut point_on_focus_plane);

        let mut point_on_defocus_disk = Vec3::new(0.0, disk_x, disk_y);

        self.camera.vertex_camera_to_world_space(&mut point_on_focus_plane);
        self.camera.vertex_camera_to_world_space(&mut point_on_defocus_disk);

        let ray_dir = (point_on_focus_plane - point_on_defocus_disk).normalized();

        return Ray::new(point_on_defocus_disk, ray_dir);

    }

    pub fn create_rt_test_scene(&mut self) {

        let mut rt_objects = Vec::new();

        let ground_material = Lambertian::default();
        let sphere = Sphere::new(
            Vec3::new(0.0, 0.0, -1000.0),
            1000.0,
            Vec3::new(0.5, 0.5, 0.5),
            ground_material.clone_box(),
        );
        self.add_mesh(sphere.to_mesh(4, PhongProperties::default()));
        rt_objects.push(sphere);

        let lim = 11; // 11

        for a in -lim..lim {
            for b in -lim..lim {
                let choose_mat = random_float();
                let center = Vec3::new(a as f32 + 0.9*random_float(), b as f32 + 0.9*random_float(), 0.2);

                if (center - Vec3::new(4.0, 0.0, 0.2)).len() > 0.9 {
                    let sphere_material: Box<dyn Material>;
                    let color;

                    if choose_mat < 0.8 {
                        // diffuse
                        color = Vec3::random().mul_elementwise(Vec3::random());
                        sphere_material = Box::new(Lambertian::default());
                    } else if choose_mat < 0.95 {
                        // metal
                        color = Vec3::random() * 0.5 + Vec3::new(0.5, 0.5, 0.5);
                        let fuzz = random_range(0.0, 0.5);
                        sphere_material = Box::new(Metal::new(fuzz));
                    } else {
                        // glass
                        color = Vec3::new(1.0, 1.0, 1.0);
                        sphere_material = Box::new(Dielectric::new(1.5));
                    }
                    
                    let sphere = Sphere::new(center, 0.2, color, sphere_material.clone_box());
                    self.add_mesh(sphere.to_mesh(2, PhongProperties::default()));
                    rt_objects.push(sphere);

                    // self.spheres.push(sphere);
                }
            }
        }

        let material1 = Dielectric::new(1.5);
        let sphere = Sphere::new(
            Vec3::new(0.0, 0.0, 1.0), 
            1.0, 
            Vec3::new(1.0, 1.0, 1.0), 
            material1.clone_box()
        );
        self.add_mesh(sphere.to_mesh(4, PhongProperties::default()));
        rt_objects.push(sphere);

        let material2 = Lambertian::default();
        let sphere = Sphere::new(
            Vec3::new(-4.0, 0.0, 1.0), 
            1.0 ,
            Vec3::new(0.4, 0.2, 0.1), 
            material2.clone_box()
        );
        self.add_mesh(sphere.to_mesh(4, PhongProperties::default()));
        rt_objects.push(sphere);

        let material3 = Metal::new(0.0);
        let sphere = Sphere::new(
            Vec3::new(4.0, 0.0, 1.0), 
            1.0, 
            Vec3::new(0.7, 0.6, 0.5), 
            material3.clone_box()
        );
        self.add_mesh(sphere.to_mesh(4, PhongProperties::default()));
        rt_objects.push(sphere);

        self.camera.set_fov(degrees_to_radians(20.0));
        self.camera.pos = Vec3::new(13.0, 3.0, 2.0);
        self.camera.look_at(&Vec3::zero());


        // set up bvh
        let bvh_objects: Vec<Box<dyn Hittable>> = rt_objects
            .iter()
            .cloned()
            .map(|o| Box::new(o) as Box<dyn Hittable>)
            .collect();

        self.bvh = Some(BVHNode::new(bvh_objects));
    }

    pub fn create_rt_test_scene_2(&mut self) {
        
    }
}