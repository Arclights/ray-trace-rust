use std::borrow::Borrow;
use std::ops::Neg;

use crate::{Color, HitRecord, random_float, Ray, Vec3};

pub trait Material {
    fn scatter(&self, ray_in: &Ray, rec: &HitRecord) -> (Ray, Color, bool);
}

pub struct Lambertian {
    albedo: Color,
}

impl Lambertian {
    pub const fn new(color: Color) -> Lambertian {
        Lambertian { albedo: color }
    }
}

pub const DEFAULT_LAMBERTIAN: Lambertian = Lambertian::new(Color::new(0.0, 0.0, 0.0));

impl Material for Lambertian {
    fn scatter(&self, _ray_in: &Ray, rec: &HitRecord) -> (Ray, Color, bool) {
        let scatter_direction = get_scatter_direction(rec);
        (Ray::new(rec.p.clone(), scatter_direction), self.albedo.clone(), true)
    }
}

pub struct Metal {
    albedo: Color,
    fuzz: f32,
}

impl Metal {
    pub fn new(color: Color, f: f32) -> Metal {
        Metal { albedo: color, fuzz: if f < 1.0 { f } else { 1.0 } }
    }
}

impl Material for Metal {
    fn scatter(&self, ray_in: &Ray, rec: &HitRecord) -> (Ray, Color, bool) {
        let reflected = ray_in.dir.unit_vector().reflect(&rec.normal);
        let scattered = Ray::new(rec.p.clone(), reflected + Vec3::random_in_unit_sphere() * self.fuzz);
        let is_scattered = (&scattered.dir).dot(&rec.normal) > 0.0;
        (scattered, self.albedo.clone(), is_scattered)
    }
}

pub struct Dielectric {
    index_of_refraction: f32,
}

impl Dielectric {
    pub fn new(index_of_refraction: f32) -> Dielectric {
        Dielectric { index_of_refraction }
    }
}

impl Material for Dielectric {
    fn scatter(&self, ray_in: &Ray, rec: &HitRecord) -> (Ray, Color, bool) {
        let attenuation = Color::new(1.0, 1.0, 1.0);
        let refraction_ratio = if rec.front_face { 1.0 / self.index_of_refraction } else { self.index_of_refraction };

        let direction = get_direction(ray_in, &rec.normal, refraction_ratio);

        let scattered = Ray::new(rec.p.clone(), direction);

        (scattered, attenuation, true)
    }
}

fn get_direction(ray_in: &Ray, normal: &Vec3, refraction_ratio: f32) -> Vec3 {
    let unit_direction = ray_in.dir.unit_vector();
    let cos_theta = unit_direction.borrow().neg().dot(normal).min(1.0);
    let sin_theta = (1.0 - cos_theta * cos_theta).sqrt();

    let cannot_refract = refraction_ratio * sin_theta > 1.0;
    if cannot_refract || refletance(cos_theta, refraction_ratio) > random_float() {
        unit_direction.borrow().reflect(normal)
    } else {
        unit_direction.borrow().refract(normal, refraction_ratio)
    }
}

fn refletance(cosine: f32, refraction_index: f32) -> f32 {
    let mut r0 = (1.0 - refraction_index) / (1.0 + refraction_index);
    r0 = r0 * r0;
    r0 + (1.0 - r0) * (1.0 - cosine).powi(5)
}

fn get_scatter_direction(rec: &HitRecord) -> Vec3 {
    let scatter_direction = &rec.normal + Vec3::random_unit_vector();

    if scatter_direction.near_zero() {
        return rec.normal.clone();
    }

    scatter_direction
}