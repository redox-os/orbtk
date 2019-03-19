use std::f64;

/// `BoxConstraint` describes a box constraint.
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct BoxConstraint {
    width: f64,
    height: f64,
    min_width: f64,
    min_height: f64,
    max_width: f64,
    max_height: f64,
}

impl Default for BoxConstraint {
    fn default() -> Self {
        BoxConstraint {
            width: 0.0,
            height: 0.0,
            min_width: 0.0,
            min_height: 0.0,
            max_width: f64::MAX,
            max_height: f64::MAX,
        }
    }
}

property!(
    /// `Constraint` describes the box constraint (min, max size) of a widget.
    Constraint(BoxConstraint)
);

// --- Trait implementations ---

/// Provides operations on a box constraint.
pub trait ConstraintExtension {
    /// Gets width.
    fn width(&self) -> f64;

    /// Sets width.
    fn set_width(&mut self, width: f64);

    /// Gets height.
    fn height(&self) -> f64;

    /// Sets height.
    fn set_height(&mut self, height: f64);

    /// Gets min_width.
    fn min_width(&self) -> f64;

    /// Sets min_width.
    fn set_min_width(&mut self, min_width: f64);

    /// Gets min_height.
    fn min_height(&self) -> f64;

    /// Sets min_height.
    fn set_min_height(&mut self, min_height: f64);

    /// Gets max_width.
    fn max_width(&self) -> f64;

    /// Sets max_width.
    fn set_max_width(&mut self, max_width: f64);

    /// Gets max_height.
    fn max_height(&self) -> f64;

    /// Sets max_height.
    fn set_max_height(&mut self, max_height: f64);

    /// Adjust the given `size` to match the constraint.
    fn perform(&self, size: (f64, f64)) -> (f64, f64);
}

impl ConstraintExtension for BoxConstraint {
    fn width(&self) -> f64 {
        self.width
    }

    fn set_width(&mut self, width: f64) {
        self.width = width;
    }

    fn height(&self) -> f64 {
        self.height
    }

    fn set_height(&mut self, height: f64) {
        self.height = height;
    }

    fn min_width(&self) -> f64 {
        self.min_width
    }

    fn set_min_width(&mut self, min_width: f64) {
        self.min_width = min_width;
    }

    fn min_height(&self) -> f64 {
        self.min_height
    }

    fn set_min_height(&mut self, min_height: f64) {
        self.min_height = min_height;
    }

    fn max_width(&self) -> f64 {
        self.max_width
    }

    fn set_max_width(&mut self, max_width: f64) {
        self.max_width = max_width;
    }

    fn max_height(&self) -> f64 {
        self.max_height
    }

    fn set_max_height(&mut self, max_height: f64) {
        self.max_height = max_height;
    }

    fn perform(&self, size: (f64, f64)) -> (f64, f64) {
        let size = {
            let width = if self.width > 0.0 { self.width } else { size.0 };
            let height = if self.height > 0.0 {
                self.height
            } else {
                size.1
            };

            (width, height)
        };

        (
            constrain(size.0, self.min_width, self.max_width, self.width),
            constrain(size.1, self.min_height, self.max_height, self.height),
        )
    }
}

impl ConstraintExtension for Constraint {
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

    fn min_width(&self) -> f64 {
        self.0.min_width
    }

    fn set_min_width(&mut self, min_width: f64) {
        self.0.min_width = min_width;
    }

    fn min_height(&self) -> f64 {
        self.0.min_height
    }

    fn set_min_height(&mut self, min_height: f64) {
        self.0.min_height = min_height;
    }

    fn max_width(&self) -> f64 {
        self.0.max_width
    }

    fn set_max_width(&mut self, max_width: f64) {
        self.0.max_width = max_width;
    }

    fn max_height(&self) -> f64 {
        self.0.max_height
    }

    fn set_max_height(&mut self, max_height: f64) {
        self.0.max_height = max_height;
    }

    fn perform(&self, size: (f64, f64)) -> (f64, f64) {
        self.0.perform(size)
    }
}

// Check constraint for the given
fn constrain(val: f64, min: f64, max: f64, size: f64) -> f64 {
    if min == 0.0 && max == 0.0 && size > 0.0 {
        size
    } else if val < min && min > 0.0 {
        min
    } else if val > max && max > 0.0 {
        max
    } else {
        val
    }
}
