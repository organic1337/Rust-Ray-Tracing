use std::fs::File;
use std::io::Write;

use rust_ray_tracing::vectors::{Color, Point, Vector};
use rust_ray_tracing::engine::Ray;
use rust_ray_tracing::utils::ppm_writer::PPMWriter;
use rust_ray_tracing::engine::hittables::hittable::{HitRecord, Hittable};
use rust_ray_tracing::engine::hittables::hittable_collection::HittableCollection;
use rust_ray_tracing::engine::hittables::sphere::Sphere;
use rust_ray_tracing::engine::camera::Camera;
use rand::Rng;


fn ray_color<T: Hittable>(ray: &Ray, world: &T) -> Color {
    let record = world.hit(ray, 0.0, f64::INFINITY);
    match record {
        Some(record) => {
            let record_normal = record.normal;
            let ray_color = Color::new(
                record_normal.x + 1.0,
                record_normal.y + 1.0,
                record_normal.z + 1.0
            );

            ray_color / 2.0
        }
        None => {
            let t = (ray.direction.unit().y + 1.0) / 2.0;
            let blue = Color::new(0.5, 0.7, 1.0);
            let white = Color::new(1.0, 1.0, 1.0);

            (1.0 - t) * white  + t * blue
        }
    }
}


fn main() {
    // Image
    let aspect_ratio = 16.0 / 9.0;
    let image_width: usize = 400;
    let image_height: usize = (image_width as f64 / aspect_ratio) as usize;

    // World
    let mut world = HittableCollection::new();
    let sphere = Sphere::new(Point::new(0.0, 0.0, -1.0), 0.5);
    let surface = Sphere::new(Point::new(0.0, -100.5, -1.0), 100.0);
    world.add(Box::new(sphere));
    world.add(Box::new(surface));

    // Camera
    let camera = Camera::new(
        Point::new(0.0, 0.0, 0.0),
        aspect_ratio,
        2.0,
        1.0
    );

    // Render
    let samples_count = 30;

    let mut writer = PPMWriter::get_file_writer("test.ppm");
    writer.write_size(image_height as usize, image_width as usize);

    for j in (0..image_height).rev() {
        for i in 0..(image_width) {
            let mut color = Color::new(0.0, 0.0, 0.0);

            for _ in 0..samples_count {
                let random_bias_x = rand::thread_rng().gen_range(0.0..1.0);
                let random_bias_y = rand::thread_rng().gen_range(0.0..1.0);

                let x = (i as f64 + random_bias_x) / ((image_width - 1) as f64);
                let y = (j as f64 + random_bias_y) / ((image_height - 1) as f64);

                let ray = camera.get_ray(x, y);
                color = color + ray_color(&ray, &world);
            }

            writer.write_color(color, samples_count);
        }
    }
}
