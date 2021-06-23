use std::io::Write;
use crate::vectors::Color;
use std::fs::File;
use std::cmp::max;

const WRITE_ERROR_MESSAGE: &str = "Could not write to buffer.";
const CREATE_FILE_ERROR_MESSAGE: &str = "Failed to create file";

/// Used for writing data in Portable Pixel Map format.
/// The output format is as follows:
///
/// P3
/// 3 2
/// 255
/// 0 0 0 255 255 255 0 0 0
/// 20 20 20 255 255 255 0 0 0
///
/// P3 => Means that pixels are represented by ascii numbers.
/// 3 2 => 3 columns and 2 rows.
/// 255 => Maximum color value.
/// ... => RGB triplets.
pub struct PPMWriter {
    buffer: Box<dyn Write>,
    is_size_written: bool,
}


impl PPMWriter {
    pub fn get_file_writer(file_path: &str) -> PPMWriter {
        let mut file_writer = File::create(file_path).expect(CREATE_FILE_ERROR_MESSAGE);
        PPMWriter::new(Box::new(file_writer))
    }

    pub fn new(mut buffer: Box<dyn Write>) -> PPMWriter {
        // 'P3' means the colors are represented by ASCII numbers
        buffer.write(b"P3\n").expect(WRITE_ERROR_MESSAGE);

        PPMWriter {
            buffer,
            is_size_written: false,
        }
    }

    pub fn write_size(&mut self, height: usize, width: usize) {
        if self.is_size_written {
            panic!("Size has already been written and should not be written more than once.")
        }

        write!(self.buffer, "{} {}\n", width, height).expect(WRITE_ERROR_MESSAGE);

        // 255 represents the maximum color
        write!(self.buffer, "255\n").expect(WRITE_ERROR_MESSAGE);
        self.is_size_written = true;
    }

    pub fn clamp(&self, number: f64, min_value: f64, max_value: f64) -> f64 {
        if number > max_value {
            return max_value;
        }
        if number < min_value {
            return min_value
        }

        number
    }

    /// Write a single pixel to the final image
    pub fn write_color(&mut self, color: Color, samples_count: usize) {
        if !self.is_size_written {
            panic!("Size hasn't been written yet. Please call the 'write_size' method before \
            calling 'write_color' method.");
        }

        let color = color / samples_count as f64;
        let clamped_red = self.clamp(color.red, 0.0, 0.999);
        let clamped_green = self.clamp(color.green, 0.0, 0.999);
        let clamped_blue = self.clamp(color.blue, 0.0, 0.999);

        let formatted_color = format!(
            "{} {} {}",
            (clamped_red * 256.0) as usize,
            (clamped_green * 256.0) as usize,
            (clamped_blue * 256.0)  as usize
        );

        write!(self.buffer, "{}\n", formatted_color).expect(WRITE_ERROR_MESSAGE);
    }
}