use std::fmt::{Display, Formatter, Error};
use std::ops::{Add, Mul};


/// Color struct represents an RGB (Red, Green, Blue) trio. Each color is somewhere
/// between 0 - 255.
pub struct Color {
    red: u8,
    green: u8,
    blue: u8,
}


impl Color {
    pub fn new(red: u8, green: u8, blue: u8) -> Color {
        Color { red, green, blue }
    }
}


impl Display for Color {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        write!(f, "{} {} {}", self.red, self.green, self.blue)
    }
}
