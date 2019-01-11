use crate::core::orbrender::{Position, Rect, Size};

// todo: ImageElement and Font as Trait. Implementation by backend wrapping native images and fonts.

pub struct ImageElementBuilder {
    pub path: String,
    pub rect: Rect,
    pub source_rect: Option<Rect>,
}

impl ImageElementBuilder {
    pub fn new<S: Into<String>>(path: S) -> Self {
        ImageElementBuilder {
            path: path.into(),
            rect: Rect::default(),
            source_rect: None,
        }
    }

    pub fn with_position(mut self, x: f64, y: f64) -> Self {
        self.rect.x = x;
        self.rect.y = y;
        self
    }

    pub fn with_size(mut self, width: f64, height: f64) -> Self {
        self.rect.width = width;
        self.rect.height = height;
        self
    }

    pub fn with_rect(self, x: f64, y: f64, width: f64, height: f64) -> Self {
        self.with_position(x, y).with_size(width, height)
    }

    pub fn with_source_rect(
        mut self,
        source_x: f64,
        source_y: f64,
        source_width: f64,
        source_height: f64,
    ) -> Self {
        self.source_rect = Some(Rect {
            x: source_x,
            y: source_y,
            width: source_width,
            height: source_height,
        });
        self
    }

    pub fn build(self) -> ImageElement {
        ImageElement {
            path: self.path,
            rect: self.rect,
            source_rect: self.source_rect,
        }
    }
}

#[derive(Clone, PartialEq, Debug, Default)]
pub struct ImageElement {
    path: String,
    rect: Rect,
    source_rect: Option<Rect>,
}

impl ImageElement {
    pub fn set_path<S: Into<String>>(&mut self, path: S) {
        self.path = path.into();
    }

    pub fn get_path(&self) -> &str {
        &self.path
    }

    pub fn set_source_rect(&mut self, source_rect: Rect) {
        self.source_rect = Some(source_rect);
    }

    pub fn get_source_rect(&self) -> &Option<Rect> {
        &self.source_rect
    }
}

impl Size for ImageElement {
    fn set_with(&mut self, width: f64) {
        self.rect.width = width;
    }

    fn get_width(&self) -> f64 {
        self.rect.height
    }

    fn set_height(&mut self, height: f64) {
        self.rect.height = height;
    }

    fn get_height(&self) -> f64 {
        self.rect.height
    }

    fn set_size(&mut self, width: f64, height: f64) {
        self.rect.width = width;
        self.rect.height = height;
    }

    fn get_size(&self) -> (f64, f64) {
        (self.rect.width, self.rect.height)
    }
}

impl Position for ImageElement {
    fn set_x(&mut self, x: f64) {
        self.rect.x = x;
    }

    fn get_x(&self) -> f64 {
        self.rect.y
    }

    fn set_y(&mut self, y: f64) {
        self.rect.y = y;
    }

    fn get_y(&self) -> f64 {
        self.rect.y
    }

    fn set_position(&mut self, x: f64, y: f64) {
        self.rect.x = x;
        self.rect.y = y;
    }

    fn get_position(&self) -> (f64, f64) {
        (self.rect.x, self.rect.y)
    }
}
