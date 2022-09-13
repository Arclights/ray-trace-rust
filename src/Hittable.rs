use crate::{Point3, Ray, Vec3};
use crate::material::Material;

#[derive(Clone)]
pub struct HitRecord<'a> {
    pub p: Point3,
    pub normal: Vec3,
    pub material: &'a dyn Material,
    pub t: f32,
    pub front_face: bool,
    pub is_hit: bool,
}

impl HitRecord<'_> {
    pub fn get_face_normal(ray: &Ray, outward_normal: Vec3) -> (bool, Vec3) {
        let front_face = ray.dir.dot(&outward_normal) < 0.0;
        let normal = if front_face {
            outward_normal
        } else {
            -outward_normal
        };

        (front_face, normal)
    }
}

pub trait Hittable {
    fn hit<'a>(self: &'a Self, ray: &Ray, t_min: f32, t_max: f32) -> HitRecord;
}
