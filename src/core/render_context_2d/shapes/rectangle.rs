use crate::{
    core::{Border, Rect, Shape2D},
    Brush, FillRule, ImageElement, ImageElementBuilder, Instruction
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
                    ImageElementBuilder::new("res/orbtk-space.png".to_string()).with_position( 10.0,
                    10.0).build()
                   
                ),
                Instruction::SetFillStyleBrush(Brush::from("#6195ED")),
                Instruction::FillRect(10.0, 10.0, 100.0, 200.0),
               
                Instruction::SetFillStyleBrush(Brush::from("#80ED61")),
                Instruction::FillRect(20.0, 20.0, 80.0, 180.0),
                 Instruction::DrawImage(
                   ImageElementBuilder::new("res/orbtk-space.png".to_string()).with_position( 60.0,
                    60.0).with_source_rect(50.0, 50.0, 150.0, 150.0).build()
                ),
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
