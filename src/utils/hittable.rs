use crate::common::vec3::{Point3, Vec3};
use crate::utils::material::{Material, NullMaterial};
use crate::common::ray::Ray;
use crate::common::vec3_opts::dot;
use std::sync::Arc;

pub struct HitRecord {
    pub p: Point3,
    pub normal: Vec3,
    pub material: Arc<dyn Material>,
    pub t: f64,
    pub front_face: bool
}

impl Default for HitRecord {
    fn default () -> Self {
        Self {
            p: Point3::default(),
            normal: Vec3::default(),
            material: Arc::new(NullMaterial {}),
            t: 0.0,
            front_face: false
        }
    }
}

impl HitRecord {
    pub fn set_face_normal(&mut self, r: &Ray, outward_normal: &Vec3) {
        self.front_face = dot(&r.direction, outward_normal) < 0.0;
        self.normal = if self.front_face {
            outward_normal.clone()
        } else {
            -outward_normal.clone()
        }
    }
}

pub trait Hittable {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord>;
}