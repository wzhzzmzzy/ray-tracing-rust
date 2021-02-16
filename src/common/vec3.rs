use std::default::Default;
use std::fmt;
use crate::utils::{random_f64_01, random_f64};

#[derive(Debug, Copy, Clone)]
pub struct Vec3 {
    pub x: f64,
    pub y: f64,
    pub z: f64
}

pub type Point3 = Vec3;
pub type Color = Vec3;

impl Default for Vec3 {
    fn default() -> Self {
        Self{ x: 0.0, y: 0.0, z: 0.0 }
    }
}

impl Vec3 {
    pub fn new (x: f64, y: f64, z: f64) -> Self {
        Self { x, y, z }
    }

    pub fn random_01 () -> Vec3 {
        Vec3 {
            x: random_f64_01(),
            y: random_f64_01(),
            z: random_f64_01()
        }
    }

    pub fn random (min: f64, max: f64) -> Vec3 {
        Vec3 {
            x: random_f64(min, max),
            y: random_f64(min, max),
            z: random_f64(min, max),
        }
    }

    pub fn length_squared(&self) -> f64 {
        self.x * self.x + self.y *self.y + self.z * self.z
    }

    pub fn length(&self) -> f64 {
        self.length_squared().sqrt()
    }

    pub fn near_zero(&self) -> bool {
        let s = 1e-8;
        self.x.abs() < s && self.y.abs() < s && self.z.abs() < s
    }
}

impl fmt::Display for Vec3 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} {} {}", self.x, self.y, self.z)
    }
}