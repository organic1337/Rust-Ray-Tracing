use crate::vectors::{Color, Vector};
use crate::engine::materials::material::{Material, ScatterResult};
use crate::engine::Ray;
use crate::engine::hittables::hittable::HitRecord;

pub struct Lambertian {
    albedo: Color
}


impl Lambertian {
    pub fn new(albedo: Color) -> Lambertian {
        Lambertian {albedo}
    }
}


impl Material for Lambertian {
    fn scatter(&self, _ray: &Ray, hit_record: &HitRecord) -> Option<ScatterResult> {
        let mut scatter_direction = hit_record.normal + Vector::random_unit_vector();

        if scatter_direction.near_zero() {
            scatter_direction = hit_record.normal;
        }

        let scattered = Ray::new(hit_record.point, scatter_direction);
        let attenuation = self.albedo;

        Some(ScatterResult::new(scattered, attenuation))
    }
}