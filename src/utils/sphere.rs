use crate::common::vec3::{Point3, Vec3};
use crate::utils::material::Material;
use crate::common::ray::Ray;
use crate::utils::hittable::{HitRecord, Hittable};
use crate::common::vec3_opts::dot;
use std::ops::Deref;
use std::sync::Arc;

pub struct Sphere {
    pub center: Point3,
    pub radius: f64,
    pub material: Arc<dyn Material>,
}

impl Sphere {
    pub fn new (center: Point3, radius: f64, material: Box<dyn Material>) -> Self {
        Self {
            center,
            radius,
            material: Arc::from(material)
        }
    }
}

impl Hittable for Sphere {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let oc = r.origin - self.center;
        let a = r.direction.length_squared();
        let half_b = dot(&oc, &r.direction);
        let c = oc.length_squared() - self.radius * self.radius;

        let discriminant = half_b * half_b - a * c;
        if discriminant < 0.0 {
            return None;
        }
        let sqrtd = discriminant.sqrt();

        // Find the nearest root that lies in the acceptable range.
        let mut root = (-half_b - sqrtd) / a;
        if root < t_min || t_max < root {
            root = (-half_b + sqrtd) / a;
            if root < t_min || t_max < root {
                return None;
            }
        }

        let mut rec = HitRecord {
            t: root,
            p: r.at(root),
            normal: Vec3::default(),
            front_face: false,
            material: self.material.clone()
        };

        let outward_normal = (rec.p - self.center) / self.radius;
        rec.set_face_normal(r, &outward_normal);

        Some(rec)
    }
}