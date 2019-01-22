/// This struct is used to add bounds constraints to a widget.
#[derive(Default, Clone, Copy)]
pub struct Constraint {
    width: f64,
    height: f64,
    min_width: f64,
    min_height: f64,
    max_width: f64,
    max_height: f64,
}

// todo: documentation!!!

impl Constraint {
    pub fn width(&self) -> f64 {
        self.width
    }

    pub fn set_width(&mut self, width: f64) {
        self.width = width;
    }

    pub fn height(&self) -> f64 {
        self.height
    }

    pub fn set_height(&mut self, height: f64) {
        self.height = height;
    }

    pub fn size(&self) -> (f64, f64) {
        (self.width, self.height)
    }

    pub fn set_size(&mut self, width: f64, height: f64) {
        self.width = width;
        self.height = height;
    }

    pub fn min_width(&self) -> f64 {
        self.min_width
    }

    pub fn set_min_width(&mut self, min_width: f64) {
        self.min_width = min_width;
    }

    pub fn min_height(&self) -> f64 {
        self.min_height
    }

    pub fn set_min_height(&mut self, min_height: f64) {
        self.min_height = min_height;
    }

    pub fn min_size(&self) -> (f64, f64) {
        (self.min_width, self.min_height)
    }

    pub fn set_min_size(&mut self, min_width: f64, min_height: f64) {
        self.min_width = min_width;
        self.min_height = min_height;
    }

    fn max_width(&self) -> f64 {
        self.max_width
    }

    pub fn set_max_width(&mut self, max_width: f64) {
        self.max_width = max_width;
    }

    pub fn max_height(&self) -> f64 {
        self.max_height
    }

    pub fn set_max_height(&mut self, max_height: f64) {
        self.max_height = max_height;
    }

    pub fn max_size(&self) -> (f64, f64) {
        (self.max_width, self.max_height)
    }

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
