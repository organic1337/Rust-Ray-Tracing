use crate::vectors::{Point, Vector};
use crate::engine::Ray;

/// Includes information about where a ray hits a hittable object.
pub struct HitRecord {
    pub point: Point,
    pub normal: Vector,
    pub t: f64,
}

/// Should be implemented by types which represent hittable objects.
/// For example: sphere, qube, etc...
pub trait Hittable {
    fn hit(self: &Self, ray: &Ray, t_min: f64, t_max: f64) -> Result<HitRecord, ()>;
}