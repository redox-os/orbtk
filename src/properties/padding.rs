/// The `Padding` is used to define space around a widget.
#[derive(Default, Clone, Copy)]
pub struct Padding {
    pub left: i32,
    pub top: i32,
    pub right: i32,
    pub bottom: i32,
}

impl Padding {
    pub fn with(mut self, all: i32) -> Self {
        self.left = all;
        self.top = all;
        self.right = all;
        self.bottom = all;
        self
    }

    pub fn with_left(mut self, left: i32) -> Self {
        self.left = left;
        self
    }

    pub fn with_top(mut self, top: i32) -> Self {
        self.top = top;
        self
    }

    pub fn with_right(mut self, right: i32) -> Self {
        self.right = right;
        self
    }

    pub fn with_bottom(mut self, bottom: i32) -> Self {
        self.bottom = bottom;
        self
    }
}
