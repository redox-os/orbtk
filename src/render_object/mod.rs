//! This module contains all render objects used in OrbTk. Render objects are used to define how to draw parts of a widget.

use crate::backend::Renderer;
use crate::properties::Point;
use crate::widget::Context;

pub use self::font_icon::FontIconRenderObject;
pub use self::image::ImageRenderObject;
pub use self::rectangle::RectangleRenderObject;
pub use self::text::TextRenderObject;

mod font_icon;
mod image;
mod rectangle;
mod text;

/// The algorithm by which to determine if a point is inside or outside the filling region.
#[derive(PartialEq, Debug, Copy, Clone)]
pub enum FillRule {
    /// The non-zero winding rule. Default rule.
    NonZero,

    /// The even-odd winding rule.
    EvenOdd,
}

impl Default for FillRule {
    fn default() -> Self {
        FillRule::NonZero
    }
}

/// Represents a render instruction. Used for render abstraction.
#[derive(PartialEq, Debug, Clone)]
pub enum Instruction {
    /// Starts a new path by emptying the list of sub-paths. Call this when you want to create a new path.
    BeginPath,

    /// Begins a new sub-path at the point specified by the given (x, y) coordinates.
    MoveTo(f64, f64),

    /// Adds a circular arc to the current sub-path, using the given control points and radius. The arc is automatically connected to the path's latest point with a straight line, if necessary for the specified parameters.
    ArcTo(f64, f64),

    /// Attempts to add a straight line from the current point to the start of the current sub-path. If the shape has already been closed or has only one point, this function does nothing.
    ClosePath,

    /// Fills the current or given path with the current file style.
    Fill(FillRule),

    /// Specifies the color to use inside shapes.
    SetFillStyleColor(String),

    /// Draws a filled rectangle whose starting point is at the coordinates (x, y) with the specified width and height and whose style is determined by the fillStyle attribute.
    FillRect(f64, f64, f64, f64),
}

/// translater
pub trait RenderContext {
    /// Starts a new path by emptying the list of sub-paths. Call this when you want to create a new path.
    fn begin_path(&mut self);

    /// Begins a new sub-path at the point specified by the given (x, y) coordinates.
    fn move_to(&mut self, x: f64, y: f64);

    /// Adds a circular arc to the current sub-path, using the given control points and radius. The arc is automatically connected to the path's latest point with a straight line, if necessary for the specified parameters.
    fn arc_to(&mut self, x: f64, x: f64);

    /// Attempts to add a straight line from the current point to the start of the current sub-path. If the shape has already been closed or has only one point, this function does nothing.
    fn close_path(&mut self);

    /// Fills the current or given path with the current file style.
    fn fill(&mut self, fill_rule: FillRule);

    /// Specifies the color to use inside shapes.
    fn set_fill_style_color(&mut self, color: &str);

    /// Draws a filled rectangle whose starting point is at the coordinates (x, y) with the specified width and height and whose style is determined by the fillStyle attribute.
    fn fill_rect(&mut self, x: f64, y: f64, width: f64, height: f64);

    /// Translates the render instructions of a `Shape2D` to render functions of `RenderContext`.
    fn draw_shape(&mut self, instructions: &[Instruction]) {
        for instruction in instructions {
            match instruction {
                Instruction::BeginPath => self.begin_path(),
                Instruction::MoveTo(x, y) => self.move_to(*x, *y),
                Instruction::ArcTo(x, y) => self.arc_to(*x, *y),
                Instruction::SetFillStyleColor(color) => self.set_fill_style_color(&color),
                Instruction::FillRect(x, y, width, height) => self.fill_rect(*x, *y, *width, *height),
                _ => {}
            }
        }
    }
}

pub struct Rectangle {
    instructions: Vec<Instruction>,
    // todo: option instruction without border /
}

pub trait Shape2D {
    fn instructions(&self) -> &[Instruction];
}

pub struct RectangleBuilder {}

impl Rectangle {
    pub fn new() -> Self {
        Rectangle {
            instructions: vec![
                Instruction::SetFillStyleColor(String::from("")),
                Instruction::BeginPath,
                Instruction::ArcTo(0.0, 0.0),
                Instruction::ArcTo(0.0, 0.0),
                Instruction::ArcTo(0.0, 0.0),
                Instruction::ArcTo(0.0, 0.0),
                Instruction::ClosePath,
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

pub trait RenderObject {
    fn render(
        &self,
        renderer: &mut dyn Renderer,
        context: &mut Context<'_>,
        global_position: &Point,
    );
}
