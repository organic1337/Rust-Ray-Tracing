use std::ops::{Add, Mul, Div};

pub struct Vector {
    pub x: f32,
    pub y: f32,
    pub z: f32
}

impl Vector {
    pub fn new(x: f32, y: f32, z: f32) -> Vector {
        Vector {x, y, z}
    }
}

implement_vector_functions!(Vector, x, y, z);