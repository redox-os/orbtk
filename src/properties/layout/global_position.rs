use crate::structs::{Point, Position};

// todo: docu, tests, use it!!!
pub struct GlobalPosition {
    value: Point,
}

// --- Trait implementations ---

impl Position for GlobalPosition {
    fn x(&self) -> f64 {
        self.value.x
    }

    fn set_x(&mut self, x: f64) {
        self.value.x = x;
    }

    fn y(&self) -> f64 {
        self.value.y
    }

    fn set_y(&mut self, y: f64) {
        self.value.y = y;
    }

    fn position(&self) -> (f64, f64) {
        (self.value.x, self.value.y)
    }

    fn set_position(&mut self, x: f64, y: f64) {
        self.value.x = x;
        self.value.y = y;
    }
}
