use std::time::Instant;

use crate::camera::Camera;
use crate::color::Color;
use crate::hittable::{HitRecord, Hittable};
use crate::hittable_list::HittableList;
use crate::material::{Dielectric, Lambertian, Metal};
use crate::ray::Ray;
use crate::sphere::Sphere;
use crate::utils::random_float;
use crate::vec3::Vec3;

mod vec3;
mod ray;
mod hittable;
mod sphere;
mod hittable_list;
mod utils;
mod camera;
mod color;
mod material;

// Image
const ASPECT_RATIO: f32 = 16.0 / 9.0;
const IMAGE_WIDTH: i32 = 400;
const IMAGE_HEIGHT: i32 = (IMAGE_WIDTH as f32 / ASPECT_RATIO) as i32;
const SAMPLES_PER_PIXEL: i32 = 100;
const MAX_DEPTH: i8 = 50;

type Point3 = Vec3;

fn ray_color<H: Hittable>(ray: &Ray, world: &H, depth: i8) -> Color {
    // If we've exceeded the ray bounce limit, no more light is gathered.
    if depth <= 0 {
        return Color::origin();
    }

    let rec = world.hit(ray, 0.001, f32::INFINITY);
    if rec.is_hit {
        let (scattered, attenuation, was_scattered) = rec.material.scatter(ray, &rec);
        if was_scattered {
            return ray_color(&scattered, world, depth - 1) * attenuation;
        }
        return Color::origin();
    }

    let unit_direction = ray.dir.unit_vector();
    let t = 0.5 * (unit_direction.y() + 1.0);
    Color::new(1.0, 1.0, 1.0) * (1.0 - t) + Color::new(0.5, 0.7, 1.0) * t
}

fn main() {
    let world = setup_sample_scene();

    let camera = setup_camera();

    println!("P3");
    println!("{} {}", IMAGE_WIDTH, IMAGE_HEIGHT);
    println!("255");

    let start = Instant::now();

    for j in (0..=(IMAGE_HEIGHT - 1)).rev() {
        eprint!("\rScanlines remaining: {} ", j);
        for i in 0..IMAGE_WIDTH {
            let mut pixel_color = Color::origin();
            for _s in 0..SAMPLES_PER_PIXEL {
                let u = ((i as f32) + random_float()) / (IMAGE_WIDTH - 1) as f32;
                let v = ((j as f32) + random_float()) / (IMAGE_HEIGHT - 1) as f32;

                let r = camera.get_ray(u, v);

                pixel_color += ray_color(&r, &world, MAX_DEPTH);
            }
            pixel_color.write_color(SAMPLES_PER_PIXEL)
        }
    }

    let elapsed_time = start.elapsed();

    eprintln!("\nDone!");
    eprintln!("Took {}s", elapsed_time.as_secs())
}

fn setup_camera() -> Camera {
    let look_from = Point3::new(3.0, 3.0, 2.0);
    let look_at = Point3::new(0.0, 0.0, -1.0);
    let vertical_up = Vec3::new(0.0, 1.0, 0.0);
    let dist_to_focus = (&look_from - &look_at).length();
    let aperture = 2.0;

    Camera::new(
        look_from,
        look_at,
        vertical_up,
        20.0,
        ASPECT_RATIO,
        aperture,
        dist_to_focus,
    )
}

fn setup_sample_scene() -> HittableList<Sphere<'static>> {
    let material_ground = Lambertian::new(Color::new(0.8, 0.8, 0.0));
    let material_center = Lambertian::new(Color::new(0.1, 0.2, 0.5));
    let material_left = Dielectric::new(1.5);
    let material_right = Metal::new(Color::new(0.8, 0.6, 0.2), 0.0);

    HittableList::new()
        .add(Sphere::new(
            Point3::new(0.0, -100.5, -1.0),
            100.0,
            Box::new(material_ground),
        ))
        .add(Sphere::new(
            Point3::new(0.0, 0.0, -1.0),
            0.5,
            Box::new(material_center),
        ))
        .add(Sphere::new(
            Point3::new(-1.0, 0.0, -1.0),
            0.5,
            Box::new(material_left),
        ))
        .add(Sphere::new(
            Point3::new(-1.0, 0.0, -1.0),
            -0.4,
            Box::new(Dielectric::new(1.5)),
        ))
        .add(Sphere::new(
            Point3::new(1.0, 0.0, -1.0),
            0.5,
            Box::new(material_right),
        ))
}
