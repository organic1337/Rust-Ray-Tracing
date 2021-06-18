/// Represents a vector in the 3D space.
#[derive(Copy, Clone, Debug)]
pub struct Vector {
    pub x: f64,
    pub y: f64,
    pub z: f64
}

impl Vector {
    pub fn new(x: f64, y: f64, z: f64) -> Vector {
        Vector {x, y, z}
    }
}


implement_common_vector_functions!(Vector, f64, x, y, z);
implement_cross!(Vector, x, y, z);
implement_unit_vector!(Vector, x, y, z);