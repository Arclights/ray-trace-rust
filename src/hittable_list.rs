use crate::hittable::{HitRecord, Hittable};
use crate::{Ray, Vec3};

pub struct HittableList<H: Hittable> {
    objects: Vec<Box<H>>,
}

impl<H: Hittable> HittableList<H> {
    pub fn new() -> HittableList<H> {
        HittableList { objects: vec![] }
    }

    pub fn add(mut self, object: Box<H>) -> HittableList<H> {
        self.objects.push(object);
        self
    }
}

impl<H: Hittable> Hittable for HittableList<H> {
    fn hit<'a>(self: &'a Self, ray: &Ray, t_min: f32, t_max: f32, rec: &mut HitRecord<'a>) -> bool {
        let mut tmp_rec: HitRecord<'a> = HitRecord {
            p: Vec3::origin(),
            normal: Vec3::origin(),
            material: rec.material,
            t: 0.0,
            front_face: false,
        };
        let mut hit_anything = false;
        let mut closest_so_far = t_max;

        for object in self.objects.iter() {
            if object.hit(ray, t_min, closest_so_far, &mut tmp_rec) {
                hit_anything = true;
                closest_so_far = tmp_rec.t;
                *rec = tmp_rec.clone();
            }
        }

        hit_anything
    }
}