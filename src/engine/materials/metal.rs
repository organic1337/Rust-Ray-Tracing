use crate::vectors::{Color, Vector};
use crate::engine::materials::material::{Material, ScatterResult};
use crate::engine::Ray;
use crate::engine::hittables::hittable::HitRecord;
use crate::engine::utils::random_float;

pub struct Metal {
    albedo: Color,
    fuzz: f64
}

impl Metal {
    pub fn new(albedo: Color, fuzz: f64) -> Metal {
        Metal {albedo, fuzz}
    }
}

impl Material for Metal {
    fn scatter(&self, ray: &Ray, hit_record: &HitRecord) -> Option<ScatterResult> {
        let mut reflected = ray.direction.unit().reflect(hit_record.normal);

        // Add fuzz to the reflected vector
        reflected = reflected + self.fuzz * Vector::random_in_unit_sphere();

        let mut scattered = Ray::new(hit_record.point, reflected);
        let attenuation = self.albedo;

        if scattered.direction.dot(hit_record.normal) > 0.0 {
            return Some(ScatterResult::new(scattered, attenuation));
        }
        None
    }
}