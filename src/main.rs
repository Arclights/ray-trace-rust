use std::time::Instant;

use crate::camera::Camera;
use crate::color::Color;
use crate::hittable::{HitRecord, Hittable};
use crate::hittable_list::HittableList;
use crate::material::{Dielectric, Lambertian, Metal};
use crate::ray::Ray;
use crate::sphere::Sphere;
use crate::utils::{random_float, random_float_range};
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

type Point3 = Vec3;

struct ImageProperties {
    image_height: i32,
    image_width: i32,
}

struct RenderProperties {
    samples_per_pixel: i32,
    max_depth: i8,
}

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
    let (image_properties, render_properties, camera, world) = setup_final_scene();

    println!("P3");
    println!("{} {}", image_properties.image_width, image_properties.image_height);
    println!("255");

    let start = Instant::now();

    for j in (0..=(image_properties.image_height - 1)).rev() {
        eprint!("\rScanlines remaining: {} ", j);
        for i in 0..image_properties.image_width {
            let mut pixel_color = Color::origin();
            for _s in 0..render_properties.samples_per_pixel {
                let u = ((i as f32) + random_float()) / (image_properties.image_width - 1) as f32;
                let v = ((j as f32) + random_float()) / (image_properties.image_height - 1) as f32;

                let r = camera.get_ray(u, v);

                pixel_color += ray_color(&r, &world, render_properties.max_depth);
            }
            pixel_color.write_color(render_properties.samples_per_pixel)
        }
    }

    let elapsed_time = start.elapsed();

    eprintln!("\nDone!");
    eprintln!("Took {}s", elapsed_time.as_secs())
}

fn setup_sample_scene() -> (ImageProperties, RenderProperties, Camera, HittableList<Sphere<'static>>) {
    let aspect_ratio: f32 = 16.0 / 9.0;
    let image_width: i32 = 400;
    let image_height: i32 = (image_width as f32 / aspect_ratio) as i32;
    (
        ImageProperties {
            image_height,
            image_width,
        },
        RenderProperties {
            samples_per_pixel: 500,
            max_depth: 50,
        },
        setup_sample_camera(aspect_ratio),
        setup_sample_world()
    )
}

fn setup_sample_camera(aspect_ratio: f32) -> Camera {
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
        aspect_ratio,
        aperture,
        dist_to_focus,
    )
}

fn setup_sample_world() -> HittableList<Sphere<'static>> {
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

fn setup_final_scene() -> (ImageProperties, RenderProperties, Camera, HittableList<Sphere<'static>>) {
    let aspect_ratio: f32 = 3.0 / 2.0;
    let image_width: i32 = 1200;
    let image_height: i32 = (image_width as f32 / aspect_ratio) as i32;
    (
        ImageProperties {
            image_height,
            image_width,
        },
        RenderProperties {
            samples_per_pixel: 500,
            max_depth: 50,
        },
        setup_final_camera(aspect_ratio),
        setup_final_world()
    )
}

fn setup_final_camera(aspect_ratio: f32) -> Camera {
    let look_from = Point3::new(12.0, 2.0, 3.0);
    let look_at = Point3::new(0.0, 0.0, 0.0);
    let vertical_up = Vec3::new(0.0, 1.0, 0.0);
    let dist_to_focus = 10.0;
    let aperture = 0.1;

    Camera::new(
        look_from,
        look_at,
        vertical_up,
        20.0,
        aspect_ratio,
        aperture,
        dist_to_focus,
    )
}

fn setup_final_world() -> HittableList<Sphere<'static>> {
    let ground_material = Lambertian::new(Color::new(0.5, 0.5, 0.5));
    let ground = Sphere::new(
        Point3::new(0.0, -1000.0, 0.0),
        1000.0,
        Box::new(ground_material),
    );

    let material_1 = Dielectric::new(1.5);
    let object_1 = Sphere::new(
        Point3::new(0.0, 1.0, 0.0),
        1.0,
        Box::new(material_1),
    );

    let material_2 = Lambertian::new(Color::new(0.4, 0.2, 0.1));
    let object_2 = Sphere::new(
        Point3::new(-4.0, 1.0, 0.0),
        1.0,
        Box::new(material_2),
    );

    let material_3 = Metal::new(Color::new(0.7, 0.6, 0.5), 0.0);
    let object_3 = Sphere::new(
        Point3::new(4.0, 1.0, 0.0),
        1.0,
        Box::new(material_3),
    );

    let mut world = HittableList::new()
        .add(ground)
        .add(object_1)
        .add(object_2)
        .add(object_3);

    for a in -11..11 {
        for b in -11..11 {
            let choose_material = random_float();
            let center = Point3::new(a as f32 + 0.9 * random_float(), 0.2, b as f32 + 0.9 * random_float());

            if (&center - Point3::new(4.0, 0.2, 0.0)).length() > 0.9 {
                if choose_material < 0.8 {
                    // Diffuse
                    let albedo = Color::random() * Color::random();
                    let sphere_material = Lambertian::new(albedo);
                    world = world.add(Sphere::new(center, 0.2, Box::new(sphere_material)));
                } else if choose_material < 0.95 {
                    // Metal
                    let albedo = Color::random_range(0.5, 1.0);
                    let fuzz = random_float_range(0.0, 0.5);
                    let sphere_material = Metal::new(albedo, fuzz);
                    world = world.add(Sphere::new(center, 0.2, Box::new(sphere_material)));
                } else {
                    // Glass
                    let sphere_material = Dielectric::new(1.5);
                    world = world.add(Sphere::new(center, 0.2, Box::new(sphere_material)));
                }
            }
        }
    }

    world
}
