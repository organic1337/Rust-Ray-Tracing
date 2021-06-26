use crate::vectors::{Point, Vector};
use crate::engine::Ray;
use crate::engine::utils::degrees_to_radians;
use crate::consts::{FOCAL_LENGTH, VIEWPORT_HEIGHT, ASPECT_RATIO};

pub struct Camera {
    aspect_ratio: f64,
    origin: Point,
    horizontal: Vector,
    vertical: Vector,
    lower_left_corner: Vector,
    u: Vector,
    v: Vector,
    w: Vector,
    lens_radius: f64
}


impl Camera {
    pub fn new(
        look_from: Point,
        look_at: Point,
        vup: Vector,
        vertical_fov: f64,
        aspect_ratio: f64,
        aperture: f64,
        focus_distance: f64
    ) -> Camera {
        let theta = degrees_to_radians(vertical_fov);
        let h = (theta / 2.0).tan();

        let viewport_height = VIEWPORT_HEIGHT * h;
        let viewport_width = viewport_height * aspect_ratio;

        let w = (look_from - look_at).unit();
        let u = vup.cross(w).unit();
        let v =  w.cross(u);

        let origin = look_from;
        let horizontal = focus_distance * viewport_width * u;
        let vertical = focus_distance * viewport_height * v;
        let lower_left_corner = origin - horizontal / 2.0 - vertical / 2.0 - focus_distance * w;
        let lens_radius = aperture / 2.0;

        Camera {
            aspect_ratio,
            origin: look_from,
            horizontal,
            vertical,
            lower_left_corner,
            u, v, w,
            lens_radius
        }
    }

    /// Get a ray from the camera to the (s, t) location in the viewport.
    pub fn get_ray(&self, s: f64, t: f64) -> Ray {
        let random_vector = self.lens_radius * Vector::random_in_unit_disk();
        let offset = random_vector.x * self.u + random_vector.y * self.v;

        let origin = self.origin + offset;
        let direction = self.lower_left_corner + s * self.horizontal + t * self.vertical - origin;

        Ray::new(origin, direction)
    }
}