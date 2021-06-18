use crate::vectors::{Point, Vector};

pub struct Ray {
    pub origin: Point,
    pub direction: Vector
}


impl Ray {
    pub fn new(origin: Point, direction: Vector) -> Ray {
        Ray {origin, direction}
    }

    /// This fuctions work as follows: P(t) = origin + t * direction.
    /// t determines how far did the ray go from the direction.
    pub fn at(&self, t: f64) -> Point {
        self.origin + t * self.direction
    }
}