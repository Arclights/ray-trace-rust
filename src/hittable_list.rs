use crate::{Ray, Vec3};
use crate::hittable::{HitRecord, Hittable};
use crate::material::DEFAULT_LAMBERTIAN;

pub struct HittableList<H: Hittable> {
    objects: Vec<H>,
}

impl<H: Hittable> HittableList<H> {
    pub fn new() -> HittableList<H> {
        HittableList { objects: vec![] }
    }

    pub fn add(mut self, hittable: H) -> HittableList<H> {
        self.objects.push(hittable);
        self
    }
}

impl<H: Hittable> Hittable for HittableList<H> {
    fn hit<'a>(self: &'a Self, ray: &Ray, t_min: f32, t_max: f32) -> HitRecord {
        let mut closest_so_far = t_max;
        let mut tmp_rec = HitRecord {
            p: Vec3::origin(),
            normal: Vec3::origin(),
            material: &DEFAULT_LAMBERTIAN,
            t: 0.0,
            front_face: false,
            is_hit: false,
        };

        for object in self.objects.iter() {
            let rec = object.hit(ray, t_min, closest_so_far);
            if rec.is_hit {
                closest_so_far = rec.t;
                tmp_rec = rec;
            }
        }

        return tmp_rec;
    }
}