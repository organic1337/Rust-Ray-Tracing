use std::f64::consts::PI;
use rand::Rng;

pub fn degrees_to_radians(degrees: f64) -> f64 {
    degrees * PI / 180.0
}

/// Generate a random float between 0 to 1.
pub fn random_float() -> f64 {
    rand::thread_rng().gen_range(0.0..1.0)
}