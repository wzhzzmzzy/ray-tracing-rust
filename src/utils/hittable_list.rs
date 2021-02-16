use crate::utils::hittable::{Hittable, HitRecord};
use crate::common::ray::Ray;
use std::sync::Arc;

pub struct HittableList {
    pub objects: Vec<Arc<dyn Hittable>>
}

impl HittableList {
    pub fn add (&mut self, object: Arc<dyn Hittable>) {
        self.objects.push(object);
    }
}

impl Hittable for HittableList {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let mut hit_anything: Option<HitRecord> = None;
        let mut closest_so_far = t_max;

        for object in self.objects.iter() {
            if let Some(rec) = object.hit(r, t_min, closest_so_far) {
                closest_so_far = rec.t;
                hit_anything = Some(rec);
            }
        }

        hit_anything
    }
}
