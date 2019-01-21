/// This struct is used to add bounds constraints to a widget.
#[derive(Default, Clone, Copy)]
pub struct Constraint {
    pub min_width: u32,
    pub max_width: u32,
    pub min_height: u32,
    pub max_height: u32,
    pub width: u32,
    pub height: u32,
}

impl Constraint {
    pub fn with_min_width(mut self, min_width: i32) -> Self {
        self.min_width = min_width.max(0) as u32;
        self
    }

    pub fn with_max_width(mut self, max_width: i32) -> Self {
        self.max_width = max_width.max(0) as u32;
        self
    }

    pub fn with_min_height(mut self, min_height: i32) -> Self {
        self.min_height = min_height.max(0) as u32;
        self
    }

    pub fn with_max_height(mut self, max_height: i32) -> Self {
        self.max_height = max_height.max(0) as u32;
        self
    }

    pub fn with_width(mut self, width: i32) -> Self {
        self.width = width.max(0) as u32;
        self
    }

    pub fn with_height(mut self, height: i32) -> Self {
        self.height = height.max(0) as u32;
        self
    }
    /// Adjust the given `size` to match the constraint.
    pub fn perform(&self, size: (u32, u32)) -> (u32, u32) {
        (
            constrain(size.0, self.min_width, self.max_width, self.width),
            constrain(size.1, self.min_height, self.max_height, self.height),
        )
    }
}

// Check constraint for the given value.
fn constrain(val: u32, min: u32, max: u32, size: u32) -> u32 {
    if min == 0 && max == 0 && size > 0 {
        size
    } else if val < min && min > 0 {
        min
    } else if val > max && max > 0 {
        max
    } else {
        val
    }
}
