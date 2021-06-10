/// Size with width, height and dirty flag.
#[derive(Copy, Clone, PartialEq)]
pub struct DirtySize {
    width: f64,
    height: f64,
    dirty: bool,
}

impl Default for DirtySize {
    fn default() -> Self {
        DirtySize {
            width: 0.0,
            height: 0.0,
            dirty: true,
        }
    }
}

impl DirtySize {
    /// Creates a new dirty size with default values.
    pub fn new() -> Self {
        DirtySize::default()
    }

    pub fn width(&self) -> f64 {
        self.width
    }

    pub fn set_width(&mut self, width: f64) {
        if (self.width - width).abs() > std::f64::EPSILON {
            self.dirty = true;
        }

        self.width = width;
    }

    pub fn height(&self) -> f64 {
        self.height
    }

    pub fn set_height(&mut self, height: f64) {
        if (self.height - height).abs() > std::f64::EPSILON {
            self.dirty = true;
        }

        self.height = height;
    }

    pub fn size(&self) -> (f64, f64) {
        (self.width, self.height)
    }

    pub fn set_size(&mut self, width: f64, height: f64) {
        if (self.width - width).abs() > std::f64::EPSILON
            && (self.height - height).abs() > std::f64::EPSILON
        {
            self.dirty = true
        }

        self.width = width;
        self.height = height;
    }

    /// Gets the dirty flag.
    pub fn dirty(&self) -> bool {
        self.dirty
    }

    /// Sets the dirty flag.
    pub fn set_dirty(&mut self, dirty: bool) {
        self.dirty = dirty;
    }
}

#[cfg(test)]
mod tests {
    use crate::prelude::*;

    #[test]
    fn test_set_width() {
        let width = 10.0;

        let mut dirty_size = DirtySize::default();

        dirty_size.set_width(width);

        assert!(crate::f64_cmp(dirty_size.width(), width));
        assert!(dirty_size.dirty());
    }

    #[test]
    fn test_set_height() {
        let height = 10.0;

        let mut dirty_size = DirtySize::default();
        dirty_size.set_height(height);

        assert!(crate::f64_cmp(dirty_size.height(), height));
        assert!(dirty_size.dirty());
    }

    #[test]
    fn test_set_size() {
        let size = (10.0, 20.0);

        let mut dirty_size = DirtySize::default();

        dirty_size.set_size(size.0, size.1);

        assert_eq!(dirty_size.size(), size);
        assert!(dirty_size.dirty());
    }

    #[test]
    fn test_set_dirty() {
        let mut dirty_size = DirtySize::default();

        dirty_size.set_dirty(false);

        assert!(!dirty_size.dirty());
    }
}
