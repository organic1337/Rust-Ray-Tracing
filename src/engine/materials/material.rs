use crate::engine::Ray;
use crate::vectors::Color;
use crate::engine::hittables::hittable::HitRecord;


pub struct ScatterResult {
    pub scattered: Ray,
    pub attenuation: Color,
}

impl ScatterResult {
    pub fn new(scattered: Ray, attenuation: Color) -> ScatterResult {
        ScatterResult {scattered, attenuation}
    }
}

pub trait Material {
    fn scatter(&self, ray: &Ray, hit_record: &HitRecord) -> Option<ScatterResult>;
}