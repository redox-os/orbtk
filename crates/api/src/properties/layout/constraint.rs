use std::f64;

use crate::prelude::*;

/// Used to build a constraint, specifying additional details.
#[derive(Default)]
pub struct ConstraintBuilder {
    width: f64,
    height: f64,
    min_width: f64,
    min_height: f64,
    max_width: f64,
    max_height: f64,
}

/// Used to build a constraint, specifying additional details.
impl ConstraintBuilder {
    /// Creates a new `ConstraintBuilder` with default values.
    pub fn new() -> Self {
        ConstraintBuilder::default()
    }

    /// Inserts a new width.
    pub fn width(mut self, width: f64) -> Self {
        self.width = width;
        self
    }

    /// Inserts a new height.
    pub fn height(mut self, height: f64) -> Self {
        self.height = height;
        self
    }

    /// Inserts a new size.
    pub fn size(mut self, width: f64, height: f64) -> Self {
        self.width = width;
        self.height = height;
        self
    }

    /// Inserts a new min_width.
    pub fn min_width(mut self, min_width: f64) -> Self {
        self.min_width = min_width;
        self
    }

    /// Inserts a new min_height.
    pub fn min_height(mut self, min_height: f64) -> Self {
        self.min_height = min_height;
        self
    }

    /// Inserts a new min_size.
    pub fn min_size(mut self, min_width: f64, min_height: f64) -> Self {
        self.min_width = min_width;
        self.min_height = min_height;
        self
    }

    /// Inserts a new max_width.
    pub fn max_width(mut self, max_width: f64) -> Self {
        self.max_width = max_width;
        self
    }

    /// Inserts a new max_height.
    pub fn max_height(mut self, max_height: f64) -> Self {
        self.max_height = max_height;
        self
    }

    /// Inserts a new min_size.
    pub fn max_size(mut self, max_width: f64, max_height: f64) -> Self {
        self.max_width = max_width;
        self.max_height = max_height;
        self
    }

    /// Builds the constraint.
    pub fn build(self) -> Constraint {
        Constraint(BoxConstraint {
            width: self.width,
            height: self.height,
            min_width: self.min_width,
            min_height: self.min_height,
            max_width: self.max_width,
            max_height: self.max_height,
        })
    }
}

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
    #[derive(Default)]
    Constraint(BoxConstraint)
);

// --- Trait implementations ---

/// Provides operations on a box constraint.
pub trait ConstraintExt {
    /// Returns a constraint builder.
    fn create() -> ConstraintBuilder;

    /// Gets width.
    fn width(&self) -> f64;

    /// Sets width.
    fn set_width(&mut self, width: f64);

    /// Gets height.
    fn height(&self) -> f64;

    /// Sets height.
    fn set_height(&mut self, height: f64);

    /// Gets the size.
    fn size(&self) -> (f64, f64);

    /// Sets the size.
    fn set_size(&mut self, width: f64, height: f64);

    /// Gets min_width.
    fn min_width(&self) -> f64;

    /// Sets min_width.
    fn set_min_width(&mut self, min_width: f64);

    /// Gets min_height.
    fn min_height(&self) -> f64;

    /// Sets min_height.
    fn set_min_height(&mut self, min_height: f64);

    /// Gets the min_size.
    fn min_size(&self) -> (f64, f64);

    /// Sets the min size.
    fn set_min_size(&mut self, min_width: f64, min_height: f64);

    /// Gets max_width.
    fn max_width(&self) -> f64;

    /// Sets max_width.
    fn set_max_width(&mut self, max_width: f64);

    /// Gets max_height.
    fn max_height(&self) -> f64;

    /// Sets max_height.
    fn set_max_height(&mut self, max_height: f64);

    /// Gets the max_size.
    fn max_size(&self) -> (f64, f64);

    /// Sets the max size.
    fn set_max_size(&mut self, max_width: f64, max_height: f64);

    /// Adjust the given `size` to match the constraint.
    fn perform(&self, size: (f64, f64)) -> (f64, f64);
}

impl ConstraintExt for BoxConstraint {
    fn create() -> ConstraintBuilder {
        ConstraintBuilder::new()
    }

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

    fn size(&self) -> (f64, f64) {
        (self.width, self.height)
    }

    fn set_size(&mut self, width: f64, height: f64) {
        self.width = width;
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

    fn min_size(&self) -> (f64, f64) {
        (self.min_width, self.min_height)
    }

    fn set_min_size(&mut self, min_width: f64, min_height: f64) {
        self.min_width = min_width;
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

    fn max_size(&self) -> (f64, f64) {
        (self.max_width, self.max_height)
    }

    fn set_max_size(&mut self, max_width: f64, max_height: f64) {
        self.max_width = max_width;
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

impl ConstraintExt for Constraint {
    fn create() -> ConstraintBuilder {
        ConstraintBuilder::new()
    }

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

    fn min_size(&self) -> (f64, f64) {
        (self.0.min_width, self.0.min_height)
    }

    fn set_min_size(&mut self, min_width: f64, min_height: f64) {
        self.0.min_width = min_width;
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

    fn max_size(&self) -> (f64, f64) {
        (self.0.max_width, self.0.max_height)
    }

    fn set_max_size(&mut self, max_width: f64, max_height: f64) {
        self.0.max_width = max_width;
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
