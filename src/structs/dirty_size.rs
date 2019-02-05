use super::Size;

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

    /// Gets the dirty flag.
    pub fn dirty(&self) -> bool {
        self.dirty
    }

    /// Sets the dirty flag.
    pub fn set_dirty(&mut self, dirty: bool) {
        self.dirty = dirty;
    }
}

impl Size for DirtySize {
    fn width(&self) -> f64 {
        self.width
    }

    fn set_width(&mut self, width: f64) {
        if self.width != width {
            self.dirty = true;
        }

        self.width = width;
    }

    fn height(&self) -> f64 {
        self.height
    }

    fn set_height(&mut self, height: f64) {
        if self.height != height {
            self.dirty = true;
        }

        self.height = height;
    }

    fn size(&self) -> (f64, f64) {
        (self.width, self.height)
    }

    fn set_size(&mut self, width: f64, height: f64) {
        if self.width != width && self.height != height {
            self.dirty = true
        } 

        self.width = width;
        self.height = height;
    }
}
