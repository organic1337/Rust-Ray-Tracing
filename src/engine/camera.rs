use crate::vectors::{Point, Vector};
use crate::engine::Ray;

pub struct Camera {
    aspect_ratio: f64,
    viewport_height: f64,
    viewport_width: f64,
    focal_length: f64,
    origin: Point,
    horizontal: Vector,
    vertical: Vector,
    viewport_center: Vector,
    lower_left_corner: Vector,
}


impl Camera {
    pub fn new(
        origin: Point,
        aspect_ratio: f64,
        viewport_height: f64,
        focal_length: f64) -> Camera {
        let viewport_width = viewport_height * aspect_ratio;
        let horizontal = Vector::new(viewport_width, 0.0, 0.0);
        let vertical = Vector::new(0.0, viewport_height, 0.0);
        let focal_vector = Vector::new(0.0, 0.0, focal_length);
        let lower_left_corner = origin - horizontal / 2.0 - vertical / 2.0 - focal_vector;

        Camera {
            aspect_ratio,
            origin,
            viewport_height,
            focal_length,
            horizontal,
            vertical,
            lower_left_corner,
            viewport_center: focal_vector,
            viewport_width,
        }
    }

    /// Get a ray from the camera to the (x, y) location in the viewport.
    pub fn get_ray(&self, x: f64, y: f64) -> Ray {
        let direction = self.lower_left_corner + x * self.horizontal +
            y * self.vertical - self.origin;

        Ray::new(self.origin, direction)
    }
}