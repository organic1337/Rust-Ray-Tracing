use crate::data_types::{Point, Vector};

pub struct Ray {
    pub origin: Point,
    pub direction: Vector
}


impl Ray {
    /// This fuctions work as follows: P(t) = origin + t * direction.
    /// t determines how far did the ray go from the direction.
    fn at(self, t: f64) -> Point {
        self.origin + t * self.direction
    }
}