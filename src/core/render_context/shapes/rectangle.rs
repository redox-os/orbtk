use crate::core::{
    Border, Bordered, Brush, FillRule, Instruction2D, Position, Rect, Shape2D, Size, Thickness,
};

#[derive(Default)]
pub struct RectangleBuilder {
    pub background: Brush,
    pub rect: Rect,
    pub border: Border,
    pub radius: f64,
}

impl RectangleBuilder {
    pub fn new() -> Self {
        RectangleBuilder::default()
    }

    pub fn with_background(mut self, background: Brush) -> Self {
        self.background = background;
        self
    }

    pub fn with_rect(mut self, rect: Rect) -> Self {
        self.rect = rect;
        self
    }

    pub fn with_border(mut self, border: Border) -> Self {
        self.border = border;
        self
    }

    pub fn with_radius(mut self, radius: f64) -> Self {
        self.radius = radius;
        self
    }

    pub fn build(self) -> Rectangle {
        let mut rect = Rectangle {
            instructions: vec![],
            rect: self.rect,
            border: self.border,
            radius: self.radius,
            background: self.background,
        };
        rect.create_shape();
        rect
    }
}

#[derive(Default)]
pub struct Rectangle {
    instructions: Vec<Instruction2D>,
    rect: Rect,
    border: Border,
    radius: f64,
    background: Brush,
}

impl Rectangle {
    pub fn get_background(&self) -> &Brush {
        &self.background
    }

    pub fn set_background(&mut self, background: Brush) {
        self.background = background;
    }

    pub fn create_shape(&mut self) {
        self.instructions.clear();
        let has_thickness = self.border.thickness.left > 0.0
            || self.border.thickness.top > 0.0
            || self.border.thickness.right > 0.0
            || self.border.thickness.bottom > 0.0;

        if self.radius > 0.0 {
            if has_thickness {
                self.create_rounded_bordered_rect_shape();
            } else {
                self.create_rounded_rect_shape(
                    self.rect.x,
                    self.rect.y,
                    self.rect.width,
                    self.rect.height,
                    self.radius,
                    self.background.clone(),
                );
            }
        } else {
            if has_thickness {
                self.create_bordered_rect_shape();
            } else {
                self.create_rect_shape(
                    self.rect.x,
                    self.rect.y,
                    self.rect.width,
                    self.rect.height,
                    self.background.clone(),
                );
            }
        }
    }

    fn create_rect_shape(&mut self, x: f64, y: f64, width: f64, height: f64, brush: Brush) {
        self.instructions
            .push(Instruction2D::SetFillStyleBrush{brush});
        self.instructions
            .push(Instruction2D::FillRect{x, y, width, height});
    }

    fn create_bordered_rect_shape(&mut self) {
        // border
        self.create_rect_shape(
            self.rect.x,
            self.rect.y,
            self.rect.width,
            self.rect.height,
            self.border.brush.clone(),
        );

        // content
        self.create_rect_shape(
            self.rect.x,
            self.rect.y,
            self.rect.width,
            self.rect.height,
            self.border.brush.clone(),
        );

        self.instructions
            .push(Instruction2D::SetFillStyleBrush{brush: self.border.brush.clone()});
        self.instructions.push(Instruction2D::FillRect{
            x: self.rect.x,
            y: self.rect.y,
            width: self.rect.width,
            height: self.rect.height,
        });
        self.instructions
            .push(Instruction2D::SetFillStyleBrush{brush: self.background.clone()});
        self.instructions.push(Instruction2D::FillRect{
            x: self.rect.x + self.border.thickness.left,
            y: self.rect.y + self.border.thickness.top,
            width: self.rect.width - self.border.thickness.left - self.border.thickness.right,
            height: self.rect.height - self.border.thickness.top - self.border.thickness.right,
        });
    }

    fn create_rounded_rect_shape(
        &mut self,
        x: f64,
        y: f64,
        width: f64,
        height: f64,
        radius: f64,
        brush: Brush,
    ) {
        let mut radius = radius;

        if width < 2.0 * radius {
            radius = width / 2.0;
        }
        if height < 2.0 * radius {
            radius = height / 2.0;
        }

        self.instructions
            .push(Instruction2D::SetFillStyleBrush{brush});
        self.instructions.push(Instruction2D::BeginPath());
        self.instructions.push(Instruction2D::MoveTo{x: x + radius, y});
        self.instructions.push(Instruction2D::ArcTo{
            x1: x + width,
            y1: y,
            x2: x + width,
            y2: y + height,
            radius,
        });
        self.instructions.push(Instruction2D::ArcTo{
            x1: x + width,
            y1: y + height,
            x2: x,
            y2: y + height,
            radius,
        });
        self.instructions
            .push(Instruction2D::ArcTo{x1: x, y1: y + height, x2: x, y2: y, radius});
        self.instructions
            .push(Instruction2D::ArcTo{x1: x, y1: y, x2: x + width, y2: y, radius});
        self.instructions.push(Instruction2D::ClosePath());
        self.instructions
            .push(Instruction2D::Fill{rule: FillRule::default()});
    }

    fn create_rounded_bordered_rect_shape(&mut self) {
        // border
        self.create_rounded_rect_shape(
            self.rect.x,
            self.rect.y,
            self.rect.width,
            self.rect.height,
            self.radius,
            self.border.brush.clone(),
        );

        // content
        self.create_rounded_rect_shape(
            self.rect.x + self.border.thickness.left,
            self.rect.y + self.border.thickness.top,
            self.rect.width - self.border.thickness.left - self.border.thickness.right,
            self.rect.height - self.border.thickness.top - self.border.thickness.right,
            self.radius,
            self.background.clone(),
        );
    }
}

impl Shape2D for Rectangle {
    fn instructions(&self) -> &[Instruction2D] {
        &self.instructions
    }
}

impl Size for Rectangle {
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

impl Position for Rectangle {
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

impl Bordered for Rectangle {
    fn get_border_thickness(&self) -> &Thickness {
        &self.border.thickness
    }

    fn set_border_thickness(&mut self, thickness: Thickness) {
        self.border.thickness = thickness;
    }

    fn get_border_brush(&self) -> &Brush {
        &self.border.brush
    }

    fn set_border_brush(&mut self, brush: Brush) {
        self.border.brush = brush;
    }

    fn get_border(&self) -> &Border {
        &self.border
    }

    fn set_border(&mut self, border: Border) {
        self.border = border;
    }
}
