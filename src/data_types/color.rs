/// Color struct represents an RGB (Red, Green, Blue) trio. Each color is somewhere
/// between 0 - 255.
pub struct Color {
    pub red: u8,
    pub green: u8,
    pub blue: u8,
}


impl Color {
    pub fn new(red: u8, green: u8, blue: u8) -> Color {
        Color { red, green, blue }
    }
}

implement_vector_functions!(Color, u8, red, green, blue);
