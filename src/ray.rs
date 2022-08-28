use crate::{Point3, Vec3};

pub struct Ray<'a> {
    pub orig: &'a Point3,
    pub dir: Vec3,
}

impl Ray<'_> {
    fn at(self, t: f32) -> Point3 {
        self.orig + &self.dir * t
    }
}