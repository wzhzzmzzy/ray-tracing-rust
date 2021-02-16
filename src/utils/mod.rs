use std::f64::consts::PI;
use rand::Rng;

pub mod hittable;
pub mod material;
pub mod hittable_list;
pub mod sphere;

pub fn degrees_to_radians (degrees: f64) -> f64 {
    degrees * PI / 180.0
}

pub fn clamp(x: f64, min: f64, max: f64) -> f64 {
    if x < min {
        min
    } else if x > max {
        max
    } else {
        x
    }
}

pub fn random_f64_01 () -> f64 {
    let mut rng = rand::thread_rng();
    rng.gen()
}

pub fn random_f64 (min: f64, max: f64) -> f64 {
    min + (max-min) * random_f64_01()
}
