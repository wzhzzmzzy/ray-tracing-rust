use crate::common::vec3::{Point3, Vec3};
use crate::utils::{degrees_to_radians, random_f64};
use crate::common::vec3_opts::{unit_vector, cross, random_in_unit_sphere, random_in_unit_disk};
use crate::common::ray::Ray;

pub struct Camera {
    origin: Point3,
    lower_left_corner: Point3,
    horizontal: Vec3,
    vertical: Vec3,
    u: Vec3, v: Vec3, w: Vec3,
    lens_radius: f64,
    time0: f64,
    time1: f64
}

impl Camera {
    pub fn new (
        lookfrom: Point3,
        lookat: Point3,
        vup: Vec3,
        vfov: f64,
        aspect_ratio: f64,
        aperture: f64,
        focus_dist: f64,
        _time0: f64,
        _time1: f64
    ) -> Self {
        let theta = degrees_to_radians(vfov);
        let h = (theta / 2.0).tan();
        let viewport_height = 2.0 * h;
        let viewport_width = aspect_ratio * viewport_height;

        let w = unit_vector(&(lookfrom - lookat));
        let u = unit_vector(&cross(&vup, &w));
        let v = cross(&w, &u);

        let origin = lookfrom;
        let horizontal = u * focus_dist * viewport_height;
        let vertical = v * focus_dist * viewport_height;
        let lower_left_corner = origin - horizontal/2.0 - vertical/2.0 - w * focus_dist;
        let lens_radius = aperture / 2.0;
        let time0 = _time0;
        let time1 = _time1;

        Self {
            u, v, w,
            origin,
            horizontal,
            vertical,
            lower_left_corner,
            lens_radius,
            time0,
            time1
        }
    }

    pub fn get_ray (&self, s: f64, t: f64) -> Ray {
        let rd = random_in_unit_disk() * self.lens_radius;
        let offset = self.u * rd.x + self.v * rd.y;
        return Ray {
            origin: self.origin + offset,
            direction: self.lower_left_corner + self.horizontal * s + self.vertical * t - self.origin - offset,
            time: random_f64( self.time0, self.time1)
        };
    }
}
