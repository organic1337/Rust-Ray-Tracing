use std::fs::File;
use std::io::Write;

use rust_ray_tracing::vectors::{Color, Point, Vector};
use rust_ray_tracing::engine::Ray;
use rust_ray_tracing::utils::ppm_writer::PPMWriter;

/// Whether a ray hits a sphere with the given center and radius.
fn hit_sphere(ray: &Ray, center: Point, radius: f64) -> f64 {
    let distance_from_center = ray.origin - center;
    let a = ray.direction.size_squared();
    let half_b = distance_from_center.dot(ray.direction);
    let c = distance_from_center.size_squared() - radius * radius;

    let discriminant = half_b * half_b - a * c;
    if discriminant > 0.0 {
        return (-half_b - discriminant.sqrt()) / a;
    }

    return -1.0;
}


/// This function linearly blends white and blue colors depending of
/// the height of y coordinate after scaling the ray direction to unit length.
/// which means -1 <= y <= 1.
///
/// When t equals 1, we receive the blue color (== Color(0.5, 0.7, 1.0))
/// when t equals 0, we receive the white color (== Color(1.0, 1.0, 1.0)).
///
/// This is called Linear Interpolation, and it is always in format of:
///  blended_value = (1 - t) * start_value + t.end_value
fn ray_color(ray: &Ray) -> Color {
    let sphere_center = Point::new(0.0, 0.0, -1.0);
    let sphere_radius = 0.5;
    let mut t = hit_sphere(ray, sphere_center, sphere_radius);

    if t > 0.0 {
        let surface_normal = (ray.at(t) - sphere_center).unit();
        return 0.5 * Color::new(
            surface_normal.x + 1.0,
            surface_normal.y + 1.0,
            surface_normal.z + 1.0
        );
    }

    let unit_direction = ray.direction.unit();
    t = 0.5 * (unit_direction.y + 1.0);

    let unit_color = Color::new(1.0, 1.0, 1.0);
    let direction_color = Color::new(0.5, 0.7, 1.0);

    (1.0 - t) * unit_color + t * direction_color
}


fn main() {
    // Image
    let aspect_ratio = 16.0 / 9.0;
    let image_width: usize = 400;
    let image_height: usize = (image_width as f64 / aspect_ratio) as usize;


    // Camera
    let viewport_height = 2.0;
    let viewport_width = aspect_ratio * viewport_height;
    let focal_length = 1.0;

    let origin = Point::new(0.0, 0.0, 0.0);
    let horizontal = Vector::new(viewport_width, 0.0, 0.0);
    let vertical = Vector::new(0.0, viewport_height, 0.0);
    let focal_vector = Vector::new(0.0, 0.0, focal_length);
    let lower_left_corner = origin - (horizontal / 2.0) - (vertical / 2.0) - focal_vector;


    // Render
    let mut writer = PPMWriter::get_file_writer("test.ppm");
    writer.write_size(image_height as usize, image_width as usize);

    for j in (0..image_height).rev() {
        for i in 0..(image_width) {
            let u = (i as f64) / ((image_width - 1) as f64);
            let v = (j as f64) / ((image_height - 1) as f64);

            let direction = lower_left_corner + u * horizontal + v * vertical - origin;
            let ray = Ray::new(origin, direction);

            let color = ray_color(&ray);
            writer.write_color(color);
        }
    }
}
