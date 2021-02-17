use std::io::prelude::*;
use std::fs::File;
use std::path::Path;
use std::sync::{Arc, Mutex};
use std::f64::INFINITY;

use crate::common::vec3::{Vec3,Color, Point3};
use crate::common::ray::Ray;
use crate::common::vec3_opts::unit_vector;
use crate::common::color::format_color;
use crate::common::camera::Camera;

use crate::one_week::hittable::{HitRecord, Hittable};
use crate::one_week::hittable_list::HittableList;
use crate::one_week::material::{Lambertian, Material, Metal, Dielectric};
use crate::one_week::{random_f64, random_f64_01};
use crate::one_week::sphere::Sphere;
use std::thread::{spawn, sleep};
use std::collections::HashMap;
use std::time::Duration;

fn ray_color (r: &Ray, world: &Arc<HittableList>, depth: i32) -> Color {
    let rec: HitRecord;

    if depth <= 0 {
        return Color::default();
    }

    if let Some(rec) = world.hit(r, 0.001, INFINITY) {
        if let Some((attenuation, scattered)) = rec.material.scatter(r, &rec) {
            return attenuation * ray_color(&scattered, world, depth - 1);
        }
        return Color::default();
    }

    let unit_direction = unit_vector(&r.direction);
    let t = unit_direction.y * 0.5 + f64::from(1);
    Color::new(1.0, 1.0, 1.0) * (1.0 - t) + Color::new(0.5, 0.7, 1.0) * t
}

fn random_scene () -> HittableList {
    let mut world = HittableList {
        objects: Vec::<Arc<dyn Hittable>>::new()
    };

    // ground material
    world.add(Arc::new(Sphere::new(
        Point3::new(0.0, -1000.0, 0.0),
        1000.0,
        Box::new(Lambertian {
            albedo: Color::new(0.5, 0.5, 0.5)
        })
    )));

    for a in -11..11 {
        for b in -11..11 {
            let choose_mat = random_f64_01();
            let center = Point3::new(
                f64::from(a) + 0.9 * random_f64_01(),
                0.2,
                f64::from(b) + 0.9 * random_f64_01()
            );

            if (center - Point3::new(4.0, 0.2, 0.0)).length() > 0.9 {
                if choose_mat < 0.8 {
                    let albedo = Color::random_01() * Color::random_01();
                    world.add(Arc::new(Sphere::new(
                        center, 0.2, Box::new(Lambertian { albedo })
                    )));
                } else if choose_mat < 0.95 {
                    let albedo = Color::random(0.5, 1.0);
                    let fuzz = random_f64(0.0, 0.5);
                    world.add(Arc::new(Sphere::new(
                        center, 0.2, Box::new(Metal::new(albedo, fuzz))
                    )));
                } else {
                    world.add(Arc::new(Sphere::new(
                        center, 0.2, Box::new(Dielectric::new(1.5))
                    )));
                }
            }
        }
    }

    world.add(Arc::new(Sphere::new(
        Point3::new(0.0, 1.0, 0.0),
        1.0,
        Box::new(Dielectric::new(1.5))
    )));

    world.add(Arc::new(Sphere::new(
        Point3::new(-4.0, 1.0, 0.0),
        1.0,
        Box::new(Lambertian { albedo: Color::new(0.4, 0.2, 0.1) })
    )));

    world.add(Arc::new(Sphere::new(
        Point3::new(4.0, 1.0, 0.0),
        1.0,
        Box::new(Metal::new(Color::new(0.7, 0.6, 0.5), 0.0))
    )));

    world
}

pub fn entry() -> std::io::Result<()> {
    let image_path = Path::new(".");
    let mut image_file = File::create(image_path.join("dist").join("temp.ppm"))?;

    // Image
    let aspect_ratio = 3.0 / 2.0;
    let image_width = 1200;
    let image_height = (f64::from(image_width) / aspect_ratio) as i32;
    let samples_per_pixel = 200;
    let max_depth = 50;

    // World
    let world = Arc::new(random_scene());

    // Camera
    let camera = Arc::new(Camera::new(
        Point3::new(13.0, 2.0, 3.0),
        Point3::new(0.0, 0.0, 0.0),
        Vec3::new(0.0, 1.0, 0.0),
        20.0,
        aspect_ratio,
        0.1,
        10.0,
    ));

    write!(image_file, "P3\n{} {}\n255\n", image_width, image_height)?;

    let thread_num = 4;
    let mut thread_handlers = vec![];

    // Prepare tasks
    let mut tasks = vec![];
    for j in (0..image_height).rev() {
        for i in 0..image_width {
            tasks.push((i, j, tasks.len()));
        }
    }
    let task_count = Arc::new(tasks.len());
    let tasks_mutex = Arc::new(Mutex::new(tasks));

    let pixels_mutex = Arc::new(Mutex::new(HashMap::new()));

    for _ in 0..thread_num {
        let world_arc = Arc::clone(&world);
        let camera_arc = Arc::clone(&camera);
        let pixels_arc = Arc::clone(&pixels_mutex);
        let mut tasks_arc = Arc::clone(&tasks_mutex);
        let task_count_arc = Arc::clone(&task_count);

        let render_handler = spawn(move || {
            loop {
                let mut tasks_guard = tasks_arc.lock().unwrap();
                if let Some((i, j, u)) = tasks_guard.pop() {
                    let mut pixel_color = Color::default();
                    for _ in 0..samples_per_pixel {
                        let u = (f64::from(i) + random_f64_01()) / f64::from(image_width - 1);
                        let v = (f64::from(j) + random_f64_01()) / f64::from(image_height - 1);
                        let r = camera_arc.get_ray(u, v);
                        pixel_color += ray_color(&r, &world_arc, max_depth);
                    }
                    let mut pixels_guard = pixels_arc.lock().unwrap();
                    pixels_guard.insert(u, format_color(&pixel_color, samples_per_pixel));
                    eprint!("\r{}/{}", tasks_guard.len(), task_count_arc);
                } else {
                    break
                }
            }
        });
        thread_handlers.push(render_handler);
    }

    for handler in thread_handlers {
        handler.join().unwrap();
    }

    let pixels_final = pixels_mutex.lock().unwrap();
    for u in 0..*task_count {
        if let Some(s) = pixels_final.get(&u) {
            write!(image_file, "{}\n", s);
        }
    }

    Ok(())
}