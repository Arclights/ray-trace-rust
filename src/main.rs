mod vec3;
mod ray;
mod hittable;
mod sphere;
mod hittable_list;

use crate::hittable::{HitRecord, Hittable};
use crate::hittable_list::HittableList;
use crate::ray::Ray;
use crate::sphere::Sphere;
use crate::vec3::Vec3;

// Image
const ASPECT_RATIO: f32 = 16.0 / 9.0;
const IMAGE_WIDTH: i32 = 400;
const IMAGE_HEIGHT: i32 = (IMAGE_WIDTH as f32 / ASPECT_RATIO) as i32;

//Camera
const VIEWPORT_HEIGHT: f32 = 2.0;
const VIEWPORT_WIDTH: f32 = ASPECT_RATIO * VIEWPORT_HEIGHT;
const FOCAL_LENGTH: f32 = 1.0;

type Point3 = Vec3;
type Color = Vec3;

impl Color {
    fn write_color(&self) {
        println!("{} {} {}", (255.999 * self.x()) as i32, (255.999 * self.y()) as i32, (255.999 * self.z()) as i32)
    }
}

fn ray_color(ray: &Ray, world: &dyn Hittable) -> Color {
    let mut rec: HitRecord = HitRecord {
        p: Vec3::origin(),
        normal: Vec3::origin(),
        t: 0.0,
        front_face: false,
    };
    if world.hit(ray, 0.0, f32::INFINITY, &mut rec) {
        return (rec.normal + Color::new(1.0, 1.0, 1.0)) * 0.5;
    }

    let unit_direction = ray.dir.unit_vector();
    let t = 0.5 * (unit_direction.y() + 1.0);
    Color::new(1.0, 1.0, 1.0) * (1.0 - t) + Color::new(0.5, 0.7, 1.0) * t
}

fn main() {
    // World
    let world = HittableList::new()
        .add(Box::new(Sphere::new(Point3::new(0.0, 0.0, -1.0), 0.5)))
        .add(Box::new(Sphere::new(Point3::new(0.0, -100.5, -1.0), 100.0)));

    // Camera
    let origin: Point3 = Point3::origin();
    let horizontal: Vec3 = Vec3::new(VIEWPORT_WIDTH, 0.0, 0.0);
    let vertical: Vec3 = Vec3::new(0.0, VIEWPORT_HEIGHT, 0.0);
    let lower_left_corner: Vec3 = &origin - &(&horizontal / 2.0) - &vertical / 2.0 - Vec3::new(0.0, 0.0, FOCAL_LENGTH);

    println!("P3");
    println!("{} {}", IMAGE_WIDTH, IMAGE_HEIGHT);
    println!("255");

    for j in (0..=(IMAGE_HEIGHT - 1)).rev() {
        eprint!("\rScanlines remaining: {} ", j);
        for i in 0..IMAGE_WIDTH {
            let u = (i as f32) / (IMAGE_WIDTH - 1) as f32;
            let v = (j as f32) / (IMAGE_HEIGHT - 1) as f32;

            let r = Ray { orig: &origin, dir: &lower_left_corner + &horizontal * u + &vertical * v - &origin };

            let pixel_color = ray_color(&r, &world);
            pixel_color.write_color()
        }
    }
    eprintln!("\nDone!")
}
