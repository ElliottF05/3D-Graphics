use std::{fmt::Debug, future::poll_fn};

use crate::{console_log, graphics::game::GameStatus, utils::{math::{degrees_to_radians, Vec3}, utils::{get_time, random_float, random_range, sample_circle, sample_square}}};

use super::{game::Game, scene::{MaterialProperties, SceneObject, Sphere}};

const SAMPLES: usize = 10;
const MAX_DEPTH: usize = 10;

const DEFOCUS_ANGLE: f32 = degrees_to_radians(0.6);
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

        let spheres = self.spheres.take();
        let vertex_objects = self.vertex_objects.take();

        while depth > 0 {
            depth -= 1;
            let mut hit_record = HitRecord::default();
            let mut hit_anything = false;
            let mut closest_so_far = 1000.0;

            // for obj in objects.iter() {
            //     if obj.hit(&ray, 0.001, closest_so_far, &mut hit_record) {
            //         closest_so_far = hit_record.t;
            //         hit_anything = true;
            //     }
            // }

            for sphere in spheres.iter() {
                if sphere.hit(&ray, 0.001, closest_so_far, &mut hit_record) {
                    closest_so_far = hit_record.t;
                    hit_anything = true;
                }
            }

            for vertex_obj in vertex_objects.iter() {
                if vertex_obj.hit(&ray, 0.001, closest_so_far, &mut hit_record) {
                    closest_so_far = hit_record.t;
                    hit_anything = true;
                }
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

        self.spheres.replace(spheres);
        self.vertex_objects.replace(vertex_objects);

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

        let ground_material = Lambertian::default();
        let sphere = Sphere::build_sphere(
            Vec3::new(0.0, 0.0, -1000.0), 
            1000.0, 4, Vec3::new(0.5, 0.5, 0.5), 
            MaterialProperties::default(), Box::new(ground_material.clone()));
        self.add_scene_object(sphere);
        // self.spheres.push(sphere);

        for a in -11..11 {
            for b in -11..11 {
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
                    
                    let sphere = Sphere::build_sphere(
                        center, 0.2, 2, color, MaterialProperties::default(), sphere_material);
                    self.add_scene_object(sphere);
                    // self.spheres.push(sphere);
                }
            }
        }

        let material1 = Dielectric::new(1.5);
        let sphere = Sphere::build_sphere(
            Vec3::new(0.0, 0.0, 1.0), 1.0, 4, 
            Vec3::new(1.0, 1.0, 1.0), MaterialProperties::default(), 
            Box::new(material1));
        self.add_scene_object(sphere);
        // self.spheres.push(sphere);

        let material2 = Lambertian::default();
        let sphere = Sphere::build_sphere(
            Vec3::new(-4.0, 0.0, 1.0), 1.0, 4, 
            Vec3::new(0.4, 0.2, 0.1), MaterialProperties::default(), 
            Box::new(material2));
        self.add_scene_object(sphere);
        // self.spheres.push(sphere);

        let material3 = Metal::new(0.0);
        let sphere = Sphere::build_sphere(
            Vec3::new(4.0, 0.0, 1.0), 1.0, 4, 
            Vec3::new(0.7, 0.6, 0.5), MaterialProperties::default(), 
            Box::new(material3));
        self.add_scene_object(sphere);
        // self.spheres.push(sphere);

        self.camera.set_fov(degrees_to_radians(20.0));
        self.camera.pos = Vec3::new(13.0, 3.0, 2.0);
        self.camera.look_at(&Vec3::zero());
    }
}


pub trait Material: Debug + Send + Sync {
    /// scatters the inbound ray and returns a tuple of the the attenuation color and the new ray.
    fn scatter(&self, ray: &Ray, hit_record: &HitRecord) -> (bool, Vec3, Ray);

    /// util for cloning Box<dyn Material> trait objects
    fn clone_box(&self) -> Box<dyn Material>;
}

// implement the clone trait for Box<dyn Material>
impl Clone for Box<dyn Material> {
    fn clone(&self) -> Box<dyn Material> {
        self.clone_box()
    }
}

#[derive(Debug, Clone, Default)]
pub struct Lambertian {
}

impl Lambertian {
}

impl Material for Lambertian {
    fn scatter(&self, ray: &Ray, hit_record: &HitRecord) -> (bool, Vec3, Ray) {
        let reflected_dir = hit_record.normal + Vec3::random_on_hemisphere(hit_record.normal);

        if reflected_dir.near_zero() {
            console_log!("reflected_dir near zero");
        }

        let reflected_ray = Ray::new(hit_record.pos, reflected_dir);
        let attenuation = hit_record.surface_color;
        return (true, attenuation, reflected_ray)
    }

    fn clone_box(&self) -> Box<dyn Material> {
        Box::new(self.clone())
    }
}

#[derive(Debug, Clone, Default)]
pub struct Metal {
    fuzz: f32,
}

impl Metal {
    pub fn new(fuzz: f32) -> Self {
        Self { fuzz }
    }
}

impl Material for Metal {
    fn scatter(&self, ray: &Ray, hit_record: &HitRecord) -> (bool, Vec3, Ray) {
        let mut reflected_dir = ray.direction.reflect(hit_record.normal);
        reflected_dir.normalize();
        reflected_dir += self.fuzz * Vec3::random_on_unit_sphere();

        if reflected_dir.dot(hit_record.normal) < 0.0 {
            return (false, Vec3::zero(), Ray::default());
        } else  {
            let reflected_ray = Ray::new(hit_record.pos, reflected_dir);
            let attenuation = hit_record.surface_color;
            return (true, attenuation, reflected_ray)
        }
    }

    fn clone_box(&self) -> Box<dyn Material> {
        Box::new(self.clone())
    }
}


#[derive(Debug, Clone, Default)]
pub struct Dielectric {
    index_of_refrac: f32,
    // TODO: should i add color here? Doesn't really make sense for dielectric to have multiple colors
}

impl Dielectric {
    pub fn new(index_of_refrac: f32) -> Dielectric {
        return Dielectric { index_of_refrac };
    }

    fn reflectance(&self, cos_theta: f32, n1: f32, n2: f32) -> f32 {
        // use Schlick's approximation: https://en.wikipedia.org/wiki/Schlick%27s_approximation
        let mut r_0 = (n1 - n2) / (n1 + n2);
        r_0 = r_0 * r_0;

        return r_0 + (1.0 - r_0) * (1.0 - cos_theta).powi(5);
    }
}

impl Material for Dielectric {
    fn scatter(&self, ray: &Ray, hit_record: &HitRecord) -> (bool, Vec3, Ray) {

        let attenuation;
        let n1;
        let n2;

        if hit_record.front_face {
            n1 = 1.0;
            n2 = self.index_of_refrac;
            attenuation = hit_record.surface_color;
        } else {
            // default to index of refraction of air (1.0) if exiting a dielectric
            // also default to no attenuation (check this)
            n1 = self.index_of_refrac;
            n2 = 1.0;
            attenuation = Vec3::new(1.0, 1.0, 1.0);
        }

        let n1_over_n2 = n1 / n2;
        
        let ray_dir = ray.direction.normalized();

        let mut cos_theta = -ray_dir.dot(hit_record.normal);
        cos_theta = cos_theta.min(1.0);
        let sin_theta = (1.0 - cos_theta * cos_theta).sqrt();

        // total internal reflection
        let cannot_refract = n1_over_n2 * sin_theta > 1.0;
        let reflectance = self.reflectance(cos_theta, n1, n2);

        let refracted_dir = if cannot_refract || reflectance > random_float()  {
            ray_dir.reflect(hit_record.normal)
        } else {
            ray_dir.refract(hit_record.normal, n1_over_n2)
        };

        let refracted_ray = Ray::new(hit_record.pos, refracted_dir);
        return (true, attenuation, refracted_ray);
    }
    fn clone_box(&self) -> Box<dyn Material> {
        return Box::new(self.clone());
    }
}