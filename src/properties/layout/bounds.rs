use crate::structs::{Point, Position, Rect, Size};

/// Describes the actual bounds of a widget.
#[derive(Default, Copy, Clone)]
pub struct Bounds {
    value: Rect,
}

impl Bounds {
    /// Create a new bounds object with the given values.
    pub fn new(x: f64, y: f64, width: f64, height: f64) -> Self {
        Bounds {
            value: Rect::new(x, y, width, height),
        }
    }

    /// Check if this rect contains the given `point`.
    pub fn contains(&self, point: (f64, f64)) -> bool {
        point.0 >= self.value.x
            && point.0 < self.value.x + self.value.width
            && point.1 >= self.value.y
            && point.1 < self.value.y + self.value.height
    }

    /// Check if this rect contains another `rect`.
    pub fn contains_rect(&self, rect: &Bounds) -> bool {
        let p1 = rect.position();
        let p2 = (p1.0 + rect.width(), p1.1 + rect.height());
        self.contains(p1) && self.contains(p2)
    }

    /// Check if this rect intersects another `rect`.
    pub fn intersects(&self, rect: &Bounds) -> bool {
        !(rect.x() >= (self.value.x + self.value.width)
            || self.value.x >= (rect.x() + rect.width())
            || rect.y() >= (self.value.y + self.value.height)
            || self.value.y >= (rect.y() + rect.height()))
    }
}

// --- Trait implementations ---

impl Size for Bounds {
    fn width(&self) -> f64 {
        self.value.width
    }

    fn set_width(&mut self, width: f64) {
        self.value.width = width;
    }

    fn height(&self) -> f64 {
        self.value.height
    }

    fn set_height(&mut self, height: f64) {
        self.value.height = height;
    }

    fn size(&self) -> (f64, f64) {
        (self.value.width, self.value.height)
    }

    fn set_size(&mut self, width: f64, height: f64) {
        self.value.width = width;
        self.value.height = height;
    }
}

impl Position for Bounds {
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
