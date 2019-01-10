use crate::{Instruction, FillRule, Shape2D};

pub struct Rectangle {
    instructions: Vec<Instruction>,
    // todo: option instruction without border /
}

impl Rectangle {
    pub fn new() -> Self {
        Rectangle {
            instructions: vec![
                Instruction::SetFillStyleColor(String::from("")),
                Instruction::BeginPath(),
                Instruction::ArcTo(0.0, 0.0),
                Instruction::ArcTo(0.0, 0.0),
                Instruction::ArcTo(0.0, 0.0),
                Instruction::ArcTo(0.0, 0.0),
                Instruction::ClosePath(),
                Instruction::Fill(FillRule::default()),
            ],
        }
    }

    pub fn set_size(&mut self, width: f64, height: f64) {
        self.instructions[2] = Instruction::ArcTo(0.0, 0.0);
    }
}

impl Shape2D for Rectangle {
    fn instructions(&self) -> &[Instruction] {
        &self.instructions
    }
}