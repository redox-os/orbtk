use crate::{core::Shape2D, Brush, FillRule, ImageElement, Instruction};

#[derive(Copy, Clone, Default, Debug, PartialEq)]
pub struct Thickness {
    pub left: f64,
    pub top: f64,
    pub right: f64,
    pub bottom: f64,
}

#[derive(Copy, Clone, Default, Debug, PartialEq)]
pub struct Rect {
    pub x: f64,
    pub y: f64,
    pub width: f64,
    pub height: f64,
}

#[derive(Clone, Default, Debug, PartialEq)]
pub struct Border {
    pub brush: Brush,
    pub thickness: Thickness,
}

#[derive(Default)]
pub struct RectangleBuilder {
    pub background: Brush,
    pub rect: Rect,
    pub border: Border,
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

    pub fn build(self) -> Rectangle {
        Rectangle::new()
    }
}

pub struct Rectangle {
    instructions: Vec<Instruction>,
    // todo: option instruction without border /
}

impl Rectangle {
    pub fn new() -> Self {
        Rectangle {
            instructions: vec![
                Instruction::DrawImage(
                    ImageElement {
                        path: "res/orbtk-space.png".to_string(),
                    },
                    10.0,
                    10.0,
                ),
                Instruction::SetFillStyleBrush(Brush::from("#6195ED")),
                Instruction::FillRect(10.0, 10.0, 100.0, 200.0),
                Instruction::DrawImage(
                    ImageElement {
                        path: "res/orbtk-space.png".to_string(),
                    },
                    60.0,
                    30.0,
                ),
                Instruction::SetFillStyleBrush(Brush::from("#80ED61")),
                Instruction::FillRect(20.0, 20.0, 80.0, 180.0),
            ],
        }
    }

    fn rect(&mut self) {
        self.instructions = vec![
            Instruction::SetFillStyleBrush(Brush::from("#6195ED")),
            Instruction::BeginPath(),
            Instruction::MoveTo(10.0, 20.0),
            Instruction::LineTo(40.0, 20.0),
            Instruction::LineTo(50.0, 30.0),
            Instruction::ClosePath(),
            Instruction::Fill(FillRule::default()),
        ];
    }

    fn rect_with_border(&mut self) {
        self.instructions = vec![
            Instruction::SetFillStyleBrush(Brush::from("#6195ED")),
            Instruction::BeginPath(),
            Instruction::MoveTo(10.0, 20.0),
            Instruction::LineTo(40.0, 20.0),
            Instruction::LineTo(50.0, 30.0),
            Instruction::ClosePath(),
            Instruction::Fill(FillRule::default()),
        ];
    }

    fn rounded_rect(&mut self) {
        self.instructions = vec![
            Instruction::SetFillStyleBrush(Brush::from("#6195ED")),
            Instruction::BeginPath(),
            Instruction::MoveTo(10.0, 20.0),
            Instruction::LineTo(40.0, 20.0),
            Instruction::LineTo(50.0, 30.0),
            Instruction::ClosePath(),
            Instruction::Fill(FillRule::default()),
        ];
    }

    fn rounded_rect_with_border(&mut self) {
        self.instructions = vec![
            Instruction::SetFillStyleBrush(Brush::from("#6195ED")),
            Instruction::BeginPath(),
            Instruction::MoveTo(10.0, 20.0),
            Instruction::LineTo(40.0, 20.0),
            Instruction::LineTo(50.0, 30.0),
            Instruction::ClosePath(),
            Instruction::Fill(FillRule::default()),
        ];
    }

    pub fn set_size(&mut self, width: f64, height: f64) {
        self.instructions[2] = Instruction::ArcTo(0.0, 0.0);
    }

    pub fn set_brush(&mut self, brush: Brush) {
        self.instructions[0] = Instruction::SetFillStyleBrush(brush);
    }
}

impl Shape2D for Rectangle {
    fn instructions(&self) -> &[Instruction] {
        &self.instructions
    }
}
