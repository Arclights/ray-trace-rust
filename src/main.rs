mod vec3;
mod ray;
mod hittable;
mod sphere;
mod hittable_list;
mod utils;
mod camera;
mod color;
mod material;

use crate::camera::Camera;
use crate::color::Color;
use crate::hittable::{HitRecord, Hittable};
use crate::hittable_list::HittableList;
use crate::material::{Lambertian, Metal};
use crate::ray::Ray;
use crate::sphere::Sphere;
use crate::utils::random_float;
use crate::vec3::Vec3;

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

    let mut rec: HitRecord = HitRecord {
        p: Vec3::origin(),
        normal: Vec3::origin(),
        material: &Lambertian::new(Color::new(0.0, 0.0, 0.0)),
        t: 0.0,
        front_face: false,
    };
    if world.hit(ray, 0.001, f32::INFINITY, &mut rec) {
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
    let world = HittableList::new()
        .add(Box::new(Sphere::new(
            Point3::new(0.0, -100.5, -1.0),
            100.0,
            Box::new(Lambertian::new(Color::new(0.8, 0.8, 0.0))),
        )))
        .add(Box::new(Sphere::new(
            Point3::new(0.0, 0.0, -1.0),
            0.5,
            Box::new(Lambertian::new(Color::new(0.7, 0.3, 0.3))),
        )))
        .add(Box::new(Sphere::new(
            Point3::new(-1.0, 0.0, -1.0),
            0.5,
            Box::new(Metal::new(Color::new(0.8, 0.8, 0.8))),
        )))
        .add(Box::new(Sphere::new(
            Point3::new(1.0, 0.0, -1.0),
            0.5,
            Box::new(Metal::new(Color::new(0.8, 0.6, 0.2))),
        )));

    let camera = Camera::new();

    println!("P3");
    println!("{} {}", IMAGE_WIDTH, IMAGE_HEIGHT);
    println!("255");

    for j in (0..=(IMAGE_HEIGHT - 1)).rev() {
        eprint!("\rScanlines remaining: {} ", j);
        for i in 0..IMAGE_WIDTH {
            let mut pixel_color = Color::origin();
            for s in 0..SAMPLES_PER_PIXEL {
                let u = ((i as f32) + random_float()) / (IMAGE_WIDTH - 1) as f32;
                let v = ((j as f32) + random_float()) / (IMAGE_HEIGHT - 1) as f32;

                let r = camera.get_ray(u, v);

                pixel_color += ray_color(&r, &world, MAX_DEPTH);
            }
            pixel_color.write_color(SAMPLES_PER_PIXEL)
        }
    }
    eprintln!("\nDone!")
}
