use std::{fmt::Debug};

use crate::{console_log, graphics::game::GameStatus, utils::{math::{degrees_to_radians, Vec3}, utils::{get_time, random_float, random_range, sample_circle, sample_square}}};

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
                for _ in 0..self.ray_samples {
                    let ray = self.get_rand_ray_at_pixel_with_defocus(x, y);
                    // let ray_color = self.ray_trace(ray, self.ray_max_depth);
                    let ray_color = self.ray_trace_mis(ray, self.ray_max_depth);
                    pixel_color += ray_color;
                }
                pixel_color /= self.ray_samples as f32;
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
            // 55.79s without, 18.89 seconds with
            let max_comp = (throughput.x).max(throughput.y).max(throughput.z).clamp(0.05, 1.0);
            if random_float() > max_comp {
                break;
            }
            throughput /= max_comp;

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
                    (successful_scatter, attenuation, scattered_ray, light_sampling_option) = material.scatter_mis(&ray, &hit_record, &self.rt_lights);
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

    pub fn create_rt_test_scene_spheres(&mut self) {

        let mut rt_objects = Vec::new();

        self.ray_samples = 10;
        self.ray_max_depth = 10;

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
                        sphere_material = Lambertian::default().clone_box();
                    } else if choose_mat < 0.95 {
                        // metal
                        color = Vec3::random() * 0.5 + Vec3::new(0.5, 0.5, 0.5);
                        let fuzz = random_range(0.0, 0.5);
                        sphere_material = Metal::new(fuzz).clone_box();
                    } else {
                        // glass
                        color = Vec3::new(1.0, 1.0, 1.0);
                        sphere_material = Dielectric::new(1.5).clone_box();
                    }
                    
                    let sphere = Sphere::new(center, 0.2, color, sphere_material);
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

        self.defocus_angle = degrees_to_radians(0.6);
        self.focus_dist = 10.0;


        // set up bvh
        let bvh_objects: Vec<Box<dyn Hittable>> = rt_objects
            .iter()
            .cloned()
            .map(|o| Box::new(o) as Box<dyn Hittable>)
            .collect();

        self.bvh = Some(BVHNode::new(bvh_objects));
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
        let rt_triangles = mesh.to_rt_triangles(&Lambertian::default());

        self.add_mesh(mesh);

        let rt_objects = rt_triangles.into_iter().map(|t| Box::new(t) as Box<dyn Hittable>).collect();
        // let triangle = Triangle::new_from_vertices(Vec3::new(0.0, 0.0, 5.0), Vec3::new(0.0, 0.0, 0.0), Vec3::new(5.0, 0.0, 0.0), Vec3::new(1.0, 0.0, 0.0), &Lambertian::default());
        // let rt_objects = vec![Box::new(triangle) as Box<dyn Hittable>];

        self.bvh = Some(BVHNode::new(rt_objects));

        self.camera.set_fov(degrees_to_radians(80.0));
        self.camera.pos = Vec3::new(0.0, 9.0, 0.0);
        self.camera.look_at(&Vec3::zero());
    }

    pub fn create_rt_test_scene_simple_light(&mut self) {

        self.ray_samples = 100;
        self.ray_max_depth = 50;
        
        let mut rt_objects: Vec<Box<dyn Hittable>> = Vec::new();

        let sphere = Sphere::new(
            Vec3::new(0.0, 0.0, -1000.0), 
            1000.0, 
            Vec3::new(0.9, 0.9, 0.9), 
            Lambertian::default().clone_box()
        );
        self.add_mesh(sphere.to_mesh(4, PhongProperties::default()));
        rt_objects.push(Box::new(sphere));

        let sphere = Sphere::new(
            Vec3::new(0.0, 0.0, 2.0), 
            2.0, 
            Vec3::new(0.9, 0.9, 0.9), 
            Lambertian::default().clone_box()
        );
        self.add_mesh(sphere.to_mesh(4, PhongProperties::default()));
        rt_objects.push(Box::new(sphere));

        let (t1, t2) = Triangle::new_quad(
            Vec3::new(3.0, -2.0, 1.0),
            Vec3::new(0.0, 0.0, 2.0),
            Vec3::new(2.0, 0.0, 0.0),
            Vec3::new(4.0, 4.0, 4.0), 
            &DiffuseLight::default()
        );
        self.add_mesh(t1.to_mesh(PhongProperties::default()));
        self.add_mesh(t2.to_mesh(PhongProperties::default()));
        rt_objects.push(Box::new(t1.clone()));
        rt_objects.push(Box::new(t2.clone()));
        self.rt_lights.push(Box::new(t1));
        self.rt_lights.push(Box::new(t2));

        self.bvh = Some(BVHNode::new(rt_objects));

        self.max_sky_color = Vec3::new(0.01, 0.01, 0.01);
        // self.max_sky_color = Vec3::zero();
        self.min_sky_color = Vec3::zero();

        self.camera.pos = Vec3::new(26.0, 6.0, 3.0);
        self.camera.look_at(&Vec3::new(0.0, 0.0, 2.0));

        self.camera.set_fov(degrees_to_radians(20.0));
        self.defocus_angle = 0.0;
    }


    pub fn create_rt_test_scene_cornell(&mut self) {
        self.ray_samples = 50; // 200 (tested at 1000 samples, 50 depth)
        self.ray_max_depth = 20; // 50
        // self.max_sky_color = Vec3::zero();
        self.max_sky_color = Vec3::new(0.1, 0.1, 0.1);
        self.min_sky_color = Vec3::zero();
    
        let red_color = Vec3::new(0.65, 0.05, 0.05);
        let green_color = Vec3::new(0.12, 0.45, 0.15);
        let white_color = Vec3::new(0.73, 0.73, 0.73);
        let light_color = Vec3::new(15.0, 15.0, 15.0);
    
        let mut rt_triangles: Vec<Triangle> = vec![];

        // Green wall (left)
        let (t1, t2) = Triangle::new_quad(
            Vec3::new(0.0, 0.0, 0.0),
            Vec3::new(0.0, 555.0, 0.0),
            Vec3::new(0.0, 0.0, 555.0),
            green_color,
            &Lambertian::default(),
        );
        rt_triangles.push(t1);
        rt_triangles.push(t2);
    
        // Red wall (right)
        let (t1, t2) = Triangle::new_quad(
            Vec3::new(555.0, 0.0, 0.0),
            Vec3::new(0.0, 0.0, 555.0),
            Vec3::new(0.0, 555.0, 0.0),
            red_color,
            &Lambertian::default(),
        );
        rt_triangles.push(t1);
        rt_triangles.push(t2);
    
        // Light (on ceiling)
        let (t1, t2) = Triangle::new_quad(
            Vec3::new(343.0, 332.0, 554.0),
            Vec3::new(0.0, -105.0, 0.0),
            Vec3::new(-130.0, 0.0, 0.0),
            light_color,
            &DiffuseLight::default(),
        );
        rt_triangles.push(t1.clone());
        rt_triangles.push(t2.clone());
        self.rt_lights.push(Box::new(t1));
        self.rt_lights.push(Box::new(t2));
    
        // Floor
        let (t1, t2) = Triangle::new_quad(
            Vec3::new(0.0, 0.0, 0.0),
            Vec3::new(555.0, 0.0, 0.0),
            Vec3::new(0.0, 555.0, 0.0),
            white_color,
            &Lambertian::default(),
        );
        rt_triangles.push(t1);
        rt_triangles.push(t2);
    
        // Ceiling
        let (t1, t2) = Triangle::new_quad(
            Vec3::new(555.0, 555.0, 555.0),
            Vec3::new(0.0, -555.0, 0.0),
            Vec3::new(-555.0, 0.0, 0.0),
            white_color,
            &Lambertian::default(),
        );
        rt_triangles.push(t1);
        rt_triangles.push(t2);
    
        // Back wall
        let (t1, t2) = Triangle::new_quad(
            Vec3::new(0.0, 555.0, 0.0),
            Vec3::new(555.0, 0.0, 0.0),
            Vec3::new(0.0, 0.0, 555.0),
            white_color,
            &Lambertian::default(),
        );
        rt_triangles.push(t1);
        rt_triangles.push(t2);

        // Left box
        let mut mesh = Mesh::build_box_from_corners(Vec3::new(271.0, 295.0, 0.0), 
        Vec3::new(106.0, 460.0, 330.0), 
        white_color, 
        PhongProperties::default());
        mesh.rotate_around_center(degrees_to_radians(15.0), 0.0);
        rt_triangles.append(&mut mesh.to_rt_triangles(&Lambertian::default()));

        // Right box
        let mut mesh = Mesh::build_box_from_corners(Vec3::new(451.0, 65.0, 0.0), 
        Vec3::new(286.0, 230.0, 165.0), 
        white_color, 
        PhongProperties::default());
        mesh.rotate_around_center(degrees_to_radians(-18.0), 0.0);
        rt_triangles.append(&mut mesh.to_rt_triangles(&Lambertian::default()));
    
        // Convert to rasterization meshes
        for tri in rt_triangles.iter() {
            self.add_mesh(tri.to_mesh(PhongProperties::default()));
        }
        let rt_objects: Vec<Box<dyn Hittable>> = rt_triangles
            .iter()
            .map(|t| Box::new(t.clone()) as Box<dyn Hittable>)
            .collect();
    
        self.bvh = Some(BVHNode::new(rt_objects));
    
        self.camera.set_fov(degrees_to_radians(40.0));
        self.camera.pos = Vec3::new(278.0, -800.0, 278.0); // looking along +y now
        self.camera.look_at(&Vec3::new(278.0, 0.0, 278.0));
        self.defocus_angle = 0.0;
    }
}