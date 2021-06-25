use crate::engine::materials::material::{Material, ScatterResult};
use crate::engine::Ray;
use crate::engine::hittables::hittable::HitRecord;
use crate::vectors::Color;
use crate::consts::AIR_REFRACTION_INDEX;
use crate::engine::utils::random_float;

pub struct Dielectric {
    refraction_index: f64
}


impl Dielectric {
    pub fn new(refraction_index: f64) -> Dielectric {
        Dielectric {refraction_index}
    }

    fn get_refraction_ratio(&self, hit_record: &HitRecord) -> f64 {
        if hit_record.front_face {
            return AIR_REFRACTION_INDEX / self.refraction_index
        }

        self.refraction_index
    }

    /// Schlick's approximation for for reflectance
    fn reflectance(cos_theta: f64, refraction_ratio: f64) -> f64 {
        let mut a = (1.0 - refraction_ratio) / (1.0 + refraction_ratio);
        a = a * a;
        a + (1.0 - a) * (1.0 - cos_theta).powf(5.0)
    }
}


impl Material for Dielectric {
    fn scatter<'a, 'b, 'c>(&'a self, ray: &'b Ray, hit_record: &'c HitRecord<'a>) -> Option<ScatterResult> {
        let attenuation = Color::new(1.0, 1.0, 1.0);
        let refraction_ratio = self.get_refraction_ratio(hit_record);

        let unit_direction = ray.direction.unit();
        let refracted_direction = unit_direction.refract(hit_record.normal, refraction_ratio);

        let cos_theta = f64::min(((-1.0) * unit_direction).dot(hit_record.normal), 1.0);
        let sin_theta = (1.0 - cos_theta * cos_theta).sqrt();

        // According to snell's law
        let cannot_refract = refraction_ratio * sin_theta > 1.0;

        let direction;
        if cannot_refract || Dielectric::reflectance(cos_theta, refraction_ratio) > random_float(0.0, 1.0) {
            direction = unit_direction.reflect(hit_record.normal);
        } else {
            direction = unit_direction.refract(hit_record.normal, refraction_ratio);
        }

        let result = ScatterResult::new(
            Ray::new(hit_record.point, refracted_direction),
            attenuation
        );
        Some(result)
    }
}