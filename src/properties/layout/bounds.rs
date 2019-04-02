use crate::prelude::*;

property!(
    /// `Bounds` describes the actual bounds (position and size) of a widget.
    Bounds(Rect)
);

// --- Trait implementations ---

/// Contains different methods to check the bounds.
pub trait BoundsExtension {
    /// Check if this rect contains the given `point`.
    fn contains(&self, point: (f64, f64)) -> bool;

    /// Check if this rect contains another `rect`.
    fn contains_rect(&self, rect: &Bounds) -> bool;

    /// Check if this rect intersects another `rect`.
    fn intersects(&self, rect: &Bounds) -> bool;
}

impl BoundsExtension for Bounds {
    fn contains(&self, point: (f64, f64)) -> bool {
        point.0 >= self.0.x
            && point.0 < self.0.x + self.0.width
            && point.1 >= self.0.y
            && point.1 < self.0.y + self.0.height
    }

    fn contains_rect(&self, rect: &Bounds) -> bool {
        let p1 = rect.position();
        let p2 = (p1.0 + rect.width(), p1.1 + rect.height());
        self.contains(p1) && self.contains(p2)
    }

    fn intersects(&self, rect: &Bounds) -> bool {
        !(rect.x() >= (self.0.x + self.0.width)
            || self.0.x >= (rect.x() + rect.width())
            || rect.y() >= (self.0.y + self.0.height)
            || self.0.y >= (rect.y() + rect.height()))
    }
}

impl Size for Bounds {
    fn width(&self) -> f64 {
        self.0.width
    }

    fn set_width(&mut self, width: f64) {
        self.0.width = width;
    }

    fn height(&self) -> f64 {
        self.0.height
    }

    fn set_height(&mut self, height: f64) {
        self.0.height = height;
    }

    fn size(&self) -> (f64, f64) {
        (self.0.width, self.0.height)
    }

    fn set_size(&mut self, width: f64, height: f64) {
        self.0.width = width;
        self.0.height = height;
    }
}

impl Position for Bounds {
    fn x(&self) -> f64 {
        self.0.x
    }

    fn set_x(&mut self, x: f64) {
        self.0.x = x;
    }

    fn y(&self) -> f64 {
        self.0.y
    }

    fn set_y(&mut self, y: f64) {
        self.0.y = y;
    }

    fn position(&self) -> (f64, f64) {
        (self.0.x, self.0.y)
    }

    fn set_position(&mut self, x: f64, y: f64) {
        self.0.x = x;
        self.0.y = y;
    }
}

// --- Conversions ---

impl From<(f64, f64, f64, f64)> for Bounds {
    fn from(t: (f64, f64, f64, f64)) -> Self {
        Bounds::from(Rect::new(t.0, t.1, t.2, t.3))
    }
}
