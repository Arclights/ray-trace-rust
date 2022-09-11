use std::f32::consts::PI;

pub fn random_float() -> f32 {
    rand::random::<f32>()
}

pub fn random_float_range(min: f32, max: f32) -> f32 {
    min + (max - min) * random_float()
}

pub fn clamp(x: f32, min: f32, max: f32) -> f32 {
    if x < min {
        min
    } else if x > max {
        max
    } else {
        x
    }
}

pub fn degrees_to_radians(degrees: f32) -> f32 {
    degrees * PI / 180.0
}