use crate::engine::utils::{random_float};
use std::cmp::min;


/// Represents a vector in the 3D space.
#[derive(Copy, Clone, Debug)]
pub struct Vector {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Vector {
    pub fn new(x: f64, y: f64, z: f64) -> Vector {
        Vector { x, y, z }
    }

    /// Generate a vector with all elements equal to 0
    pub fn zeroes() -> Vector {
        Vector::new(0.0, 0.0, 0.0)
    }

    /// Generate a vector with random elements between 0 - 1
    pub fn random(min_value: f64, max_value: f64) -> Vector {
        Vector::new(
            random_float(min_value, max_value),
            random_float(min_value, max_value),
            random_float(min_value, max_value)
        )
    }

    pub fn random_in_unit_sphere() -> Vector {
        loop {
            let random_vector = Vector::random(-1.0, 1.0);
            if random_vector.size_squared() <= 1.0 {
                return random_vector;
            }
        }
    }

    pub fn reflect(self, normal: Vector) -> Vector {
        self - 2.0 * self.dot(normal) * normal
    }

    pub fn random_unit_vector() -> Vector {
        Vector::random_in_unit_sphere().unit()
    }

    pub fn refract(self, normal: Vector, refraction_ratio: f64) -> Vector {
        let cos_theta = f64::min(((-1.0) * self).dot(normal), 1.0);
        let perp_ray = refraction_ratio * (self + cos_theta * normal);
        let parallel_ray = (-1.0) * (1.0 - perp_ray.size_squared()).abs().sqrt() * normal;

        perp_ray + parallel_ray
    }
}

implement_common_vector_functions!(Vector, f64, x, y, z);
implement_cross_function!(Vector, x, y, z);
implement_unit_function!(Vector, x, y, z);