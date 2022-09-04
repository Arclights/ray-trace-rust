use crate::{Color, HitRecord, Ray, Vec3};

pub trait Material {
    fn scatter(&self, ray_in: &Ray, rec: &HitRecord) -> (Ray, Color, bool);
}

pub struct Lambertian {
    albedo: Color,
}

impl Lambertian {
    pub fn new(color: Color) -> Lambertian {
        Lambertian { albedo: color }
    }
}

impl Material for Lambertian {
    fn scatter(&self, ray_in: &Ray, rec: &HitRecord) -> (Ray, Color, bool) {
        let scatter_direction = get_scatter_direction(rec);
        (Ray::new(rec.p.clone(), scatter_direction), self.albedo.clone(), true)
    }
}

pub struct Metal {
    albedo: Color,
}

impl Metal {
    pub fn new(color: Color) -> Metal {
        Metal { albedo: color }
    }
}

impl Material for Metal {
    fn scatter(&self, ray_in: &Ray, rec: &HitRecord) -> (Ray, Color, bool) {
        let reflected = ray_in.dir.unit_vector().reflect(&rec.normal);
        let scat = Ray::new(rec.p.clone(), reflected);
        let is_scattered = (&scat.dir).dot(&rec.normal) > 0.0;
        (scat, self.albedo.clone(), is_scattered)
    }
}

fn get_scatter_direction(rec: &HitRecord) -> Vec3 {
    let scatter_direction = &rec.normal + Vec3::random_unit_vector();

    if scatter_direction.near_zero() {
        return rec.normal.clone();
    }

    scatter_direction
}