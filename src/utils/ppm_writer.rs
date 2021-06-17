use std::io::Write;
use crate::data_types::Color;

const WRITE_ERROR_MESSAGE: &str = "Could not write to buffer.";

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
struct PPMWriter {
    buffer: Box<dyn Write>,
    is_size_written: bool,
}


impl PPMWriter {
    pub fn new(mut buffer: Box<dyn Write>) -> PPMWriter {
        // 'P3' means the colors are represented by ASCII numbers
        buffer.write(b"P3\n").expect(WRITE_ERROR_MESSAGE);

        PPMWriter {
            buffer,
            is_size_written: false,
        }
    }

    pub fn write_size(&mut self, rows: usize, columns: usize) {
        if self.is_size_written {
            panic!("Size has already been written and should not be written more than once.")
        }

        write!(self.buffer, "{} {}\n", columns, rows).expect(WRITE_ERROR_MESSAGE);

        // 255 represents the maximum color
        write!(self.buffer, "255\n").expect(WRITE_ERROR_MESSAGE);
        self.is_size_written = true;
    }

    /// Write a single pixel to the final image
    pub fn write_color(&mut self, color: &Color) {
        if !self.is_size_written {
            panic!("Size hasn't been written yet. Please call the 'write_size' method before \
            calling 'write_color' method.");
        }

        write!(self.buffer, "{}", color.to_string()).expect(WRITE_ERROR_MESSAGE);
    }
}