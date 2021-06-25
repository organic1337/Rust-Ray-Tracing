use crate::vectors::{Point, Vector};
use crate::engine::Ray;
use crate::engine::materials::material::Material;

/// Includes information about where a ray hits a hittable object.
pub struct HitRecord<'a> {
    pub point: Point,
    pub normal: Vector,
    pub material: &'a Box<dyn Material>,
    pub t: f64,
    pub front_face: bool
}


impl<'a> HitRecord<'a> {
    pub fn new(point: Point, normal: Vector, material: &'a Box<dyn Material>, t: f64, front_face: bool) -> HitRecord {
        HitRecord {point, normal, material, t, front_face}
    }

    pub fn from_ray(ray: &Ray,  t: f64, outward_normal: Vector, material: &'a Box<dyn Material>) -> HitRecord<'a> {
        let front_face = ray.direction.dot(outward_normal) < 0.0;
        let normal = if front_face { outward_normal } else {(-1.0) * outward_normal };
        let hit_point = ray.at(t);

        HitRecord::new(hit_point, normal, material, t, front_face)
   }
}

/// Should be implemented by types which represent hittable objects.
/// For example: sphere, qube, etc...
pub trait Hittable<'a> {
    fn hit(self: &Self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord<'a>>;
}