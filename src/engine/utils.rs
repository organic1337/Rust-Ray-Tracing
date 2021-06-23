use std::f64::consts::PI;
use rand::Rng;

pub fn degrees_to_radians(degrees: f64) -> f64 {
    degrees * PI / 180.0
}

pub fn random_float(min_value: f64, max_value: f64) -> f64 {
    rand::thread_rng().gen_range(min_value..max_value)
}