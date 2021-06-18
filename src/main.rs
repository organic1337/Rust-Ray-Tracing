use std::fs::File;
use std::io::Write;
use rust_ray_tracing::data_types::Color;
use rust_ray_tracing::utils::ppm_writer::PPMWriter;


fn main() {
    let bitmap_height: u8 = 200;
    let bitmap_width: u8 = 200;

    let mut writer = PPMWriter::get_file_writer("test.ppm");
    writer.write_size(bitmap_height as usize, bitmap_width as usize);

    for i in 0..bitmap_height {
        for j in 0..bitmap_width {
            let color = Color::new(i, 20, j);
            writer.write_color(&color);
        }
    }

}
