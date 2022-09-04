use crate::hittable::{HitRecord, Hittable};
use crate::{Point3, Ray};
use crate::material::Material;

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
    fn hit<'a>(self:&'a Self, ray: &Ray, t_min: f32, t_max: f32, rec: &mut HitRecord<'a>) -> bool {
        let oc = &ray.orig - &self.center;
        let a = ray.dir.length_squared();
        let half_b = oc.dot(&ray.dir);
        let c = oc.length_squared() - self.radius * self.radius;

        let discriminant = half_b * half_b - a * c;
        if discriminant < 0.0 {
            return false;
        }

        let sqrtd = discriminant.sqrt();

        // Find the nearest root that lies in the acceptable range.
        let mut root = (-half_b - sqrtd) / a;
        if root < t_min || t_max < root {
            root = (-half_b + sqrtd) / a;
            if root < t_min || t_max < root {
                return false;
            }
        }

        rec.t = root;
        rec.p = ray.at(rec.t);
        let outward_normal = (&rec.p - &self.center) / self.radius;
        rec.set_face_normal(ray, outward_normal);
        rec.material = self.material.as_ref();

        return true;
    }
}

