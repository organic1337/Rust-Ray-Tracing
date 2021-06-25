use crate::vectors::{Point, Vector};
use crate::engine::Ray;
use crate::engine::utils::degrees_to_radians;
use crate::consts::{FOCAL_LENGTH, VIEWPORT_HEIGHT, ASPECT_RATIO};

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
        look_from: Point,
        look_at: Point,
        vup: Vector,
        vertical_fov: f64,
        aspect_ratio: f64
    ) -> Camera {
        let theta = degrees_to_radians(vertical_fov);
        let h = (theta / 2.0).tan();

        let viewport_height = VIEWPORT_HEIGHT * h;
        let viewport_width = viewport_height * aspect_ratio;


        let w = (look_from - look_at).unit();
        let u = vup.cross(w).unit();
        let v =  w.cross(u);

        let origin = look_from;
        let horizontal = viewport_width * u;
        let vertical = viewport_height * v;
        let focal_vector = Vector::new(0.0, 0.0, FOCAL_LENGTH);
        let lower_left_corner = origin - horizontal / 2.0 - vertical / 2.0 - w;

        Camera {
            aspect_ratio,
            origin: look_from,
            viewport_height,
            focal_length: FOCAL_LENGTH,
            horizontal,
            vertical,
            lower_left_corner,
            viewport_center: focal_vector,
            viewport_width,
        }
    }

    /// Get a ray from the camera to the (s, t) location in the viewport.
    pub fn get_ray(&self, s: f64, t: f64) -> Ray {
        let direction = self.lower_left_corner + s * self.horizontal + t * self.vertical - self.origin;

        Ray::new(self.origin, direction)
    }
}