use crate::engine::Ray;
use crate::vectors::Color;
use crate::engine::hittables::hittable::HitRecord;

pub struct ScatterResult {
    pub scattered: Ray,
    pub attenuation: Color
}

pub trait Material {
    fn scatter(ray: &Ray, hit_record: &HitRecord) {
        
    }
}