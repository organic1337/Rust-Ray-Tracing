/// Color struct represents an RGB (Red, Green, Blue) trio. Each color is somewhere
/// between 0 - 255.
#[derive(Copy, Clone)]
pub struct Color {
    pub red: f32,
    pub green: f32,
    pub blue: f32,
}


impl Color {
    pub fn new(red: f32, green: f32, blue: f32) -> Color {
        Color { red, green, blue }
    }
}

implement_vector_functions!(Color, f32, red, green, blue);
