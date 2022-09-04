use crate::utils::clamp;
use crate::Vec3;

pub type Color = Vec3;

impl Color {
    pub fn write_color(&self, sample_per_pixel: i32) {
        let mut r = self.x();
        let mut g = self.y();
        let mut b = self.z();

        let scale = 1.0 / (sample_per_pixel as f32);
        r = (scale * r).sqrt();
        g = (scale * g).sqrt();
        b = (scale * b).sqrt();

        println!("{} {} {}",
                 (256.0 * clamp(r, 0.0, 0.999)) as i32,
                 (256.0 * clamp(g, 0.0, 0.999)) as i32,
                 (256.0 * clamp(b, 0.0, 0.999)) as i32
        )
    }
}
