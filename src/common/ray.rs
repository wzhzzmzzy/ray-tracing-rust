use crate::common::vec3::{Point3, Vec3};

pub struct Ray {
    pub origin: Point3,
    pub direction: Vec3,
    pub time: f64
}

impl Ray {
    pub fn new(origin: &Point3, direction: &Vec3) -> Self {
        Ray {
            origin: origin.clone(),
            direction: direction.clone(),
            time: 0.0
        }
    }

    pub fn at(&self, t: f64) -> Point3 {
        self.origin.clone() + self.direction.clone() * t
    }
}