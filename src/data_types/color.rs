use std::fmt::{Display, Formatter, Error};

pub struct Color {
    red: u8,
    green: u8,
    blue: u8
}

impl Display for Color {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        write!(f, "{} {} {}", self.red, self.green, self.blue)
    }
}

impl Color {
    pub fn new(red: u8, green: u8, blue: u8) -> Color {
        Color {red, green, blue}
    }
}