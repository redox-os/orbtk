use crate::core::{
    Border, Bordered, Brush, PathSegment, Position, Rect, Shape, Size, Thickness,
};

/// Used to build a rectangle, specifying additional details.
#[derive(Default)]
pub struct RectangleBuilder {
    pub background: Brush,
    pub rect: Rect,
    pub border: Border,
}

impl RectangleBuilder {
    /// Creates a new `RectangleBuilder` with default values.
    pub fn new() -> Self {
        RectangleBuilder::default()
    }

    /// Inserts a new background brush.
    pub fn with_background(mut self, background: Brush) -> Self {
        self.background = background;
        self
    }

    /// Inserts a new bounds rect.
    pub fn with_rect(mut self, rect: Rect) -> Self {
        self.rect = rect;
        self
    }

    /// Inserts a new border.
    pub fn with_border(mut self, border: Border) -> Self {
        self.border = border;
        self
    }

    /// Builds the rectangle.
    pub fn build(self) -> Rectangle {
        let mut rect = Rectangle {
            path: vec![],
            rect: self.rect,
            border: self.border,
            background: self.background,
        };
        rect.build_path();
        rect
    }
}

/// The ÌmageElement` is used to display a rectangle on the screen.
#[derive(Default)]
pub struct Rectangle {
    path: Vec<PathSegment>,
    rect: Rect,
    border: Border,
    background: Brush,
}

impl Rectangle {
    /// Creates a new `RectangleBuilder` object with default values.
    pub fn create() -> RectangleBuilder {
        RectangleBuilder::new()
    }

    /// Gets the background brush.
    pub fn background(&self) -> &Brush {
        &self.background
    }

    /// Sets the background brush.
    pub fn set_background(&mut self, background: Brush) {
        self.background = background;
    }

    // Builds rectangle path without border and radius.
    fn build_rect_path(&mut self, x: f64, y: f64, width: f64, height: f64, brush: Brush) {
        self.path
            .push(PathSegment::SetFillStyleBrush { brush });
        self.path.push(PathSegment::FillRect {
            x,
            y,
            width,
            height,
        });
    }

    // Builds rectangle path with border and without radius.
    fn build_bordered_rect_path(&mut self) {
        // border
        self.build_rect_path(
            self.rect.x,
            self.rect.y,
            self.rect.width,
            self.rect.height,
            self.border.brush().clone(),
        );

        // content
        self.build_rect_path(
            self.rect.x + self.border.thickness().left,
            self.rect.y + self.border.thickness().top,
            self.rect.width - self.border.thickness().left - self.border.thickness().right,
            self.rect.height - self.border.thickness().top - self.border.thickness().right,
            self.background.clone(),
        );
    }

    // Builds ractangle path with radius and withoud border.
    fn build_rounded_rect_path(
        &mut self,
        x: f64,
        y: f64,
        width: f64,
        height: f64,
        radius: f64,
        brush: Brush,
    ) {
        let m_pi = 3.14159265;
        let degrees = m_pi / 180.0;

        self.path.push(PathSegment::BeginPath());
        self.path.push(PathSegment::Arc {
            x: x + width - radius,
            y: y + radius,
            radius,
            start_angle: -90.0 * degrees,
            end_engle: 0.0 * degrees,
        });
        self.path.push(PathSegment::Arc {
            x: x + width - radius,
            y: y + height - radius,
            radius,
            start_angle: 0.0 * degrees,
            end_engle: 90.0 * degrees,
        });
        self.path.push(PathSegment::Arc {
            x: x + radius,
            y: y + height - radius,
            radius,
            start_angle: 90.0 * degrees,
            end_engle: 180.0 * degrees,
        });
        self.path.push(PathSegment::Arc {
            x: x + radius,
            y: y + radius,
            radius,
            start_angle: 180.0 * degrees,
            end_engle: 270.0 * degrees,
        });

        self.path
            .push(PathSegment::SetFillStyleBrush { brush });
        self.path.push(PathSegment::ClosePath());
        self.path.push(PathSegment::Fill());
    }

    // Builds rectangle with border and radius.
    fn build_rounded_bordered_rect_path(&mut self) {
        // border
        self.build_rounded_rect_path(
            self.rect.x,
            self.rect.y,
            self.rect.width,
            self.rect.height,
            self.border.radius(),
            self.border.brush().clone(),
        );

        // content
        self.build_rounded_rect_path(
            self.rect.x + self.border.thickness().left,
            self.rect.y + self.border.thickness().top,
            self.rect.width - self.border.thickness().left - self.border.thickness().right,
            self.rect.height - self.border.thickness().top - self.border.thickness().right,
            self.border.radius(),
            self.background.clone(),
        );
    }
}

impl Shape for Rectangle {
    fn path(&mut self) -> &mut [PathSegment] {
        &mut self.path
    }

    fn build_path(&mut self) {
        self.path.clear();
        let has_thickness = self.border.thickness().left > 0.0
            || self.border.thickness().top > 0.0
            || self.border.thickness().right > 0.0
            || self.border.thickness().bottom > 0.0;

        if self.border.radius() > 0.0 {
            if has_thickness {
                self.build_rounded_bordered_rect_path();
            } else {
                self.build_rounded_rect_path(
                    self.rect.x,
                    self.rect.y,
                    self.rect.width,
                    self.rect.height,
                    self.border.radius(),
                    self.background.clone(),
                );
            }
        } else {
            if has_thickness {
                self.build_bordered_rect_path();
            } else {
                self.build_rect_path(
                    self.rect.x,
                    self.rect.y,
                    self.rect.width,
                    self.rect.height,
                    self.background.clone(),
                );
            }
        }
    }
}

impl Size for Rectangle {
    fn set_with(&mut self, width: f64) {
        self.rect.width = width;
    }

    fn width(&self) -> f64 {
        self.rect.height
    }

    fn set_height(&mut self, height: f64) {
        self.rect.height = height;
    }

    fn height(&self) -> f64 {
        self.rect.height
    }

    fn set_size(&mut self, width: f64, height: f64) {
        self.rect.width = width;
        self.rect.height = height;
    }

    fn size(&self) -> (f64, f64) {
        (self.rect.width, self.rect.height)
    }
}

impl Position for Rectangle {
    fn set_x(&mut self, x: f64) {
        self.rect.x = x;
    }

    fn x(&self) -> f64 {
        self.rect.y
    }

    fn set_y(&mut self, y: f64) {
        self.rect.y = y;
    }

    fn y(&self) -> f64 {
        self.rect.y
    }

    fn set_position(&mut self, x: f64, y: f64) {
        self.rect.x = x;
        self.rect.y = y;
    }

    fn position(&self) -> (f64, f64) {
        (self.rect.x, self.rect.y)
    }
}

impl Bordered for Rectangle {
    fn border_thickness(&self) -> Thickness {
        self.border.thickness()
    }

    fn set_border_thickness(&mut self, thickness: Thickness) {
        self.border.set_thickness(thickness);
    }

    fn border_brush(&self) -> &Brush {
        &self.border.brush()
    }

    fn set_border_brush(&mut self, brush: Brush) {
        self.border.set_brush(brush);
    }

    fn border_radius(&self) -> f64 {
        self.border.radius()
    }

    fn set_border_radius(&mut self, radius: f64) {
       self.border.set_radius(radius);
    }

    fn border(&self) -> &Border {
        &self.border
    }

    fn set_border(&mut self, border: Border) {
        self.border = border;
    }
}