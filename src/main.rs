use std::fs::File;
use std::io::Write;
use rust_ray_tracing::data_types::Color;
use rust_ray_tracing::utils::ppm_writer::PPMWriter;
use rust_ray_tracing::engine::Ray;


fn ray_color(ray: &Ray) -> Color {
    let unit_direction = ray.direction.unit_vector();
    let t = 0.5 * (unit_direction.y + 1.0);

    let unit_color = Color::new(1.0, 1.0, 1.0);
    let direction_color = Color::new(0.5, 0.7, 0.1);

    (1.0 - t) * unit_color + t * direction_color
}


fn main() {
    // Image
    let aspect_ratio = 16.0 / 9.0;
    let image_width: usize = 400;
    let image_height: usize = image_width / (aspect_ratio as usize);

    // Camera
    let viewport_height = 2.0;
    let viewport_width = aspect_ratio * viewport_height;
    let focal_length = 1.0;

    let mut writer = PPMWriter::get_file_writer("test.ppm");
    writer.write_size(image_height as usize, image_width as usize);

    // for i in 0..bitmap_height {
    //     for j in 0..bitmap_width {
    //         let color = Color::new(i as f64, 20.0, j as f64);
    //         writer.write_color(&color);
    //     }
    // }

}
