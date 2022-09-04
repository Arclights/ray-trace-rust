use crate::{Point3, Ray, Vec3};

pub struct Camera {
    origin: Point3,
    lower_left_corner: Point3,
    horizontal: Vec3,
    vertical: Vec3,
}

impl Camera {
    pub fn new() -> Camera {
        let aspect_ratio: f32 = 16.0 / 9.0;
        let viewport_height: f32 = 2.0;
        let viewport_width: f32 = aspect_ratio * viewport_height;
        let focal_length: f32 = 1.0;

        let origin = Point3::origin();
        let horizontal = Vec3::new(viewport_width, 0.0, 0.0);
        let vertical = Vec3::new(0.0, viewport_height, 0.0);
        Camera {
            origin:origin.clone(),
            horizontal:horizontal.clone(),
            vertical:vertical.clone(),
            lower_left_corner: origin - &horizontal / 2.0 - vertical / 2.0 - Vec3::new(0.0, 0.0, focal_length),
        }
    }

    pub fn get_ray(&self, u: f32, v: f32) -> Ray {
        Ray {
            orig: self.origin.clone(),
            dir: &self.lower_left_corner + &self.horizontal * u + &self.vertical * v - &self.origin,
        }
    }
}