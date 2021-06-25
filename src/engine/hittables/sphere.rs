use crate::engine::hittables::hittable::{HitRecord, Hittable};
use crate::engine::Ray;
use crate::vectors::Point;
use crate::engine::materials::material::Material;

pub struct Sphere<'a> {
    center: Point,
    radius: f64,
    material: &'a Box<dyn Material>
}


impl<'a> Sphere<'a> {
    pub fn new(center: Point, radius: f64, material: &'a Box<dyn Material>) -> Sphere<'a> {
        Sphere {center, radius, material}
    }
}


impl<'a> Hittable<'a> for Sphere<'a> {
    fn hit(self: &Self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord<'a>> {
        let distance_from_center = ray.origin - self.center;
        let a = ray.direction.size_squared();
        let half_b = distance_from_center.dot(ray.direction);
        let c = distance_from_center.size_squared() - self.radius * self.radius;

        let discriminant = half_b * half_b - a * c;
        if discriminant < 0.0 {
            return None;
        }

        // Find the nearest root that lies in the mentioned range
        let discriminant_sqrt = discriminant.sqrt();
        let mut root = (-half_b - discriminant_sqrt) / a;
        if root < t_min || root > t_max {
            root = (-half_b + discriminant_sqrt) / a;
            if root < t_min || root > t_max {
                return None;
            }
        }

        let hit_point = ray.at(root);
        let normal = (hit_point - self.center) / self.radius;
        Some(HitRecord::from_ray(ray, root, normal, self.material))
    }
}