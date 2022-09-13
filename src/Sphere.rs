use std::ops::Deref;

use crate::{Point3, Ray, Vec3};
use crate::hittable::{HitRecord, Hittable};
use crate::material::{DEFAULT_LAMBERTIAN, Material};

pub struct Sphere<'a> {
    center: Point3,
    radius: f32,
    material: Box<dyn Material + 'a>,
}

impl<'a> Sphere<'_> {
    pub fn new(center: Point3, radius: f32, material: Box<dyn Material + 'a>) -> Sphere<'a> {
        Sphere { center, radius, material }
    }
}

impl Hittable for Sphere<'_> {
    fn hit<'a>(self: &'a Self, ray: &Ray, t_min: f32, t_max: f32) -> HitRecord {
        let oc = &ray.orig - &self.center;
        let a = ray.dir.length_squared();
        let half_b = oc.dot(&ray.dir);
        let c = oc.length_squared() - self.radius * self.radius;

        let discriminant = half_b * half_b - a * c;
        if discriminant < 0.0 {
            return HitRecord {
                p: Vec3::origin(),
                normal: Vec3::origin(),
                material: &DEFAULT_LAMBERTIAN,
                t: 0.0,
                front_face: false,
                is_hit: false,
            };
        }

        let sqrtd = discriminant.sqrt();

        // Find the nearest root that lies in the acceptable range.
        let mut root = (-half_b - sqrtd) / a;
        if root < t_min || t_max < root {
            root = (-half_b + sqrtd) / a;
            if root < t_min || t_max < root {
                return HitRecord {
                    p: Vec3::origin(),
                    normal: Vec3::origin(),
                    material: &DEFAULT_LAMBERTIAN,
                    t: 0.0,
                    front_face: false,
                    is_hit: false,
                };
            }
        }

        let t = root;
        let p = ray.at(t);
        let outward_normal = (&p - &self.center) / self.radius;

        let (front_face, normal) = HitRecord::get_face_normal(ray, outward_normal);

        HitRecord {
            t,
            p,
            material: self.material.deref(),
            front_face,
            normal,
            is_hit: true,
        }
    }
}

