use crate::{Point3, Ray, Vec3};
use crate::utils::degrees_to_radians;

pub struct Camera {
    origin: Point3,
    lower_left_corner: Point3,
    horizontal: Vec3,
    vertical: Vec3,
    u: Vec3,
    v: Vec3,
    lens_radius: f32,
}

impl Camera {
    pub fn new(
        look_from: Point3,
        look_at: Point3,
        view_up: Vec3,
        vertical_fov: f32,
        aspect_ratio: f32,
        aperture: f32,
        focus_distance: f32,
    ) -> Camera {
        let theta = degrees_to_radians(vertical_fov);
        let h = (theta / 2.0).tan();
        let viewport_height = 2.0 * h;
        let viewport_width = aspect_ratio * viewport_height;

        let w = (&look_from - &look_at).unit_vector();
        let u = view_up.cross(&w).unit_vector();
        let v = w.cross(&u);

        let horizontal = &u * viewport_width * focus_distance;
        let vertical = &v * viewport_height * focus_distance;
        Camera {
            lower_left_corner: &look_from - &horizontal / 2.0 - &vertical / 2.0 - &w * focus_distance,
            origin: look_from,
            horizontal,
            vertical,
            u,
            v,
            lens_radius: aperture / 2.0,
        }
    }

    pub fn get_ray(&self, s: f32, t: f32) -> Ray {
        let rd = Vec3::random_in_unit_disc() * self.lens_radius;
        let offset = &self.u * rd.x() + &self.v * rd.y();

        Ray::new(
            &self.origin + &offset,
            &self.lower_left_corner + &self.horizontal * s + &self.vertical * t - &self.origin - offset,
        )
    }
}