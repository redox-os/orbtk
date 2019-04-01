// use std::ops::{Add, Sub};

// /// This struct represents a non visual point.
// #[derive(Copy, Clone, Debug, Default)]
// pub struct Point {
//     pub x: f64,
//     pub y: f64,
// }

// impl Point {
//     /// Creates a new point.
//     pub fn new(x: f64, y: f64) -> Self {
//         Point { x: x, y: y }
//     }
// }

// impl Add for Point {
//     type Output = Point;

//     /// Adds the given `other` point to self point.
//     fn add(self, other: Point) -> Self::Output {
//         Point {
//             x: self.x + other.x,
//             y: self.y + other.y,
//         }
//     }
// }

// impl Sub for Point {
//     type Output = Point;

//     /// Subs the given `other` point from self point.
//     fn sub(self, other: Point) -> Self::Output {
//         Point {
//             x: self.x - other.x,
//             y: self.y - other.y,
//         }
//     }
// }
