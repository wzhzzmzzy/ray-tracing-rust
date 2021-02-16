use crate::common::ray::Ray;
use crate::common::vec3::{Color, Vec3};
use crate::utils::hittable::HitRecord;
use crate::common::vec3_opts::{random_unit_vector, unit_vector, dot, reflect, refract, random_in_unit_sphere};
use crate::utils::random_f64_01;

pub trait Material {
    fn scatter (&self, r_in: &Ray, rec: &HitRecord) -> Option<(Color, Ray)>;
}

pub struct NullMaterial;

impl Material for NullMaterial {
    fn scatter (&self, r_in: &Ray, rec: &HitRecord) -> Option<(Color, Ray)> {
        None
    }
}

pub struct Lambertian {
    pub albedo: Color
}

impl Material for Lambertian {
    fn scatter (&self, r_in: &Ray, rec: &HitRecord) -> Option<(Color, Ray)> {
        let mut scatter_direction = rec.normal + random_unit_vector();

        if scatter_direction.near_zero() {
            scatter_direction = rec.normal;
        }

        Some((
            self.albedo.clone(),
            Ray::new(&rec.p, &scatter_direction)
        ))
    }
}

pub struct Metal {
    pub albedo: Color,
    pub fuzz: f64
}

impl Metal {
    pub fn new (albedo: Color, fuzz: f64) -> Self {
        Self {
            albedo,
            fuzz: fuzz.min(1.0)
        }
    }
}

impl Material for Metal {
    fn scatter (&self, r_in: &Ray, rec: &HitRecord) -> Option<(Color, Ray)> {
        let reflected = reflect(&unit_vector(&r_in.direction), &rec.normal);
        Some((
            self.albedo.clone(),
            Ray::new(&rec.p, &(reflected + random_in_unit_sphere() * self.fuzz)),
        ))
    }
}

pub struct Dielectric {
    pub ir: f64
}

impl Dielectric {
    pub fn new (index_of_refraction: f64) -> Self {
        Self {
            ir: index_of_refraction
        }
    }

    fn reflectance (cosine: f64, ref_idx: f64) -> f64 {
        let mut r0 = (1.0 - ref_idx) - (1.0 + ref_idx);
        r0 = r0 * r0;
        r0 + (1.0 - r0) * (1.0 - cosine).powi(5)
    }
}

impl Material for Dielectric {
    fn scatter (&self, r_in: &Ray, rec: &HitRecord) -> Option<(Color, Ray)> {
        let refraction_ratio = if rec.front_face {
            1.0 / self.ir
        } else {
            self.ir
        };

        let unit_direction = unit_vector(&r_in.direction);
        let cos_theta = dot(&-unit_direction, &rec.normal).min(1.0);
        let sin_theta = (1.0 - cos_theta * cos_theta).sqrt();
        let cannot_refract = refraction_ratio * sin_theta > 1.0;
        let direction: Vec3;

        if cannot_refract || Self::reflectance(cos_theta, refraction_ratio) > random_f64_01() {
            direction = reflect(&unit_direction, &rec.normal);
        } else {
            direction = refract(&unit_direction, &rec.normal, refraction_ratio);
        }

        Some((
            Color::new(1.0, 1.0, 1.0),
            Ray::new(&rec.p, &direction)
        ))
    }
}

