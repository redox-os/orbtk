use std::f64;

/// This struct is used to add bound constraints to a widget.
#[derive(Clone, Copy)]
pub struct Constraint {
    width: f64,
    height: f64,
    min_width: f64,
    min_height: f64,
    max_width: f64,
    max_height: f64,
}

impl Default for Constraint {
    fn default() -> Self {
        Constraint {
            width: 0.0,
            height: 0.0,
            min_width: 0.0,
            min_height: 0.0,
            max_width: f64::MAX,
            max_height: f64::MAX,
        }
    }
}

impl Constraint {
    /// Gets the width.
    pub fn width(&self) -> f64 {
        self.width
    }

    /// Sets the width.
    pub fn set_width(&mut self, width: f64) {
        self.width = width;
    }

    /// Gets the height.
    pub fn height(&self) -> f64 {
        self.height
    }

    /// Sets the height.
    pub fn set_height(&mut self, height: f64) {
        self.height = height;
    }

    /// Gets the size.
    pub fn size(&self) -> (f64, f64) {
        (self.width, self.height)
    }

    /// Sets the size.
    pub fn set_size(&mut self, width: f64, height: f64) {
        self.width = width;
        self.height = height;
    }

    /// Gets the min width.
    pub fn min_width(&self) -> f64 {
        self.min_width
    }

    /// Sets the min width.
    pub fn set_min_width(&mut self, min_width: f64) {
        self.min_width = min_width;
    }

    /// Gets the min height.
    pub fn min_height(&self) -> f64 {
        self.min_height
    }

    /// Sets the min height.
    pub fn set_min_height(&mut self, min_height: f64) {
        self.min_height = min_height;
    }

    /// Gets the min size.
    pub fn min_size(&self) -> (f64, f64) {
        (self.min_width, self.min_height)
    }

    /// Sets the min size.
    pub fn set_min_size(&mut self, min_width: f64, min_height: f64) {
        self.min_width = min_width;
        self.min_height = min_height;
    }

    /// Gets the maximum width.
    pub fn max_width(&self) -> f64 {
        self.max_width
    }

    /// Sets the maximum width.
    pub fn set_max_width(&mut self, max_width: f64) {
        self.max_width = max_width;
    }

    /// Gets the maximum height.
    pub fn max_height(&self) -> f64 {
        self.max_height
    }

    /// Sets the maximum height.
    pub fn set_max_height(&mut self, max_height: f64) {
        self.max_height = max_height;
    }

    /// Gets the maximum size.
    pub fn max_size(&self) -> (f64, f64) {
        (self.max_width, self.max_height)
    }

    /// Sets the maximum size.
    pub fn set_max_size(&mut self, max_width: f64, max_height: f64) {
        self.max_width = max_width;
        self.max_height = max_height;
    }

    /// Adjust the given `size` to match the constraint.
    pub fn perform(&self, size: (f64, f64)) -> (f64, f64) {
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
