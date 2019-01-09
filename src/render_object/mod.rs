//! This module contains all render objects used in OrbTk. Render objects are used to define how to draw parts of a widget.

use std::collections::BTreeMap;

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

// pub struct Gradient {
//     color_stops: BTreeMap<f64, String>,
// }

// impl Default for Gradient {
//     fn default() -> Self {
//         Gradient {
//             color_stops: BTreeMap::new()
//         }
//     }
// }

// impl Gradient {
//     pub fn new() -> Self {
//         Gradient::default()
//     }

//     pub fn add_color_stop(&mut self)
// }


pub enum Brush {
    SolidColor(String),
    // Gradient([i32])
}

#[derive(Copy, Clone, PartialEq, Debug)]
pub struct ImageElement;

/// Represents a render instruction. Used for render abstraction.
#[derive(PartialEq, Debug, Clone)]
pub enum Instruction {
    /// Creates a circular arc centered at (x, y) with a radius of radius. The path starts at startAngle and ends at endAngle, and travels in the direction given by anticlockwise (defaulting to clockwise).
    Arc(f64, f64, f64, f64, f64, bool),

    /// Adds a circular arc to the current sub-path, using the given control points and radius. The arc is automatically connected to the path's latest point with a straight line, if necessary for the specified parameters.
    ArcTo(f64, f64),

    /// Starts a new path by emptying the list of sub-paths. Call this when you want to create a new path.
    BeginPath,

    /// Attempts to add a straight line from the current point to the start of the current sub-path. If the shape has already been closed or has only one point, this function does nothing.
    ClosePath,

    /// Draws an image on (x, y).
    DrawImage(ImageElement, f64, f64),

    /// Draws an image on (x, y) with (width, height).
    DrawImageD(ImageElement, f64, f64, f64, f64),

    /// Draws a part of the image with the given (source_x, source_y, source_width, source_height) on (x, y) with (width, height).
    DrawImageS(ImageElement, f64, f64, f64, f64, f64, f64, f64, f64),

    /// Fills the current or given path with the current file style.
    Fill(FillRule),

    /// Draws a filled rectangle whose starting point is at the coordinates (x, y) with the specified width and height and whose style is determined by the fillStyle attribute.
    FillRect(f64, f64, f64, f64),

    /// Draws a text string at the specified coordinates, filling the string's characters with the current foreground color. An optional parameter allows specifying a maximum width for the rendered text, which the user agent will achieve by condensing the text or by using a lower font size.
    FillText(String, f64, f64, Option<f64>),

    /// Begins a new sub-path at the point specified by the given (x, y) coordinates.
    MoveTo(f64, f64),

    /// Specifies the color to use inside shapes.
    SetFillStyleColor(String),

    /// Specifies the current text style being used when drawing text. 
    SetFont(String),
}

/// translater
pub trait RenderContext {
    /// Creates a circular arc centered at (x, y) with a radius of radius. The path starts at startAngle and ends at endAngle, and travels in the direction given by anticlockwise (defaulting to clockwise).
    fn arc(
        &mut self,
        x: f64,
        y: f64,
        radius: f64,
        start_angle: f64,
        end_engle: f64,
        anti_clockwise: bool,
    );

    /// Adds a circular arc to the current sub-path, using the given control points and radius. The arc is automatically connected to the path's latest point with a straight line, if necessary for the specified parameters.
    fn arc_to(&mut self, x: f64, y: f64);

    /// Starts a new path by emptying the list of sub-paths. Call this when you want to create a new path.
    fn begin_path(&mut self);

    /// Attempts to add a straight line from the current point to the start of the current sub-path. If the shape has already been closed or has only one point, this function does nothing.
    fn close_path(&mut self);

    /// Draws an image on (x, y).
    fn draw_image(&mut self, image_element: ImageElement, x: f64, y: f64);

    /// Draws an image on (x, y) with (width, height).
    fn draw_image_d(
        &mut self,
        image_element: ImageElement,
        x: f64,
        y: f64,
        width: f64,
        height: f64,
    );

    /// Draws a part of the image with the given (source_x, source_y, source_width, source_height) on (x, y) with (width, height).
    fn draw_image_s(
        &mut self,
        image_element: ImageElement,
        source_x: f64,
        source_y: f64,
        source_width: f64,
        source_height: f64,
        x: f64,
        y: f64,
        width: f64,
        height: f64,
    );

    /// Fills the current or given path with the current file style.
    fn fill(&mut self, fill_rule: FillRule);

    /// Draws a filled rectangle whose starting point is at the coordinates (x, y) with the specified width and height and whose style is determined by the fillStyle attribute.
    fn fill_rect(&mut self, x: f64, y: f64, width: f64, height: f64);

    /// Draws a text string at the specified coordinates, filling the string's characters with the current foreground color. An optional parameter allows specifying a maximum width for the rendered text, which the user agent will achieve by condensing the text or by using a lower font size.
    fn fill_text(&mut self, text: &str, x: f64, y: f64, max_width: Option<f64>);

    /// Begins a new sub-path at the point specified by the given (x, y) coordinates.
    fn move_to(&mut self, x: f64, y: f64);

    /// Specifies the color to use inside shapes.
    fn set_fill_style_color(&mut self, color: &str);

    /// Specifies the current text style being used when drawing text. 
    fn set_font(&mut self, font: &str);

    /// Translates the render instructions of a `Shape2D` to render functions of `RenderContext`.
    fn render_shape(&mut self, instructions: &[Instruction]) {
        for instruction in instructions {
            match instruction {
                Instruction::Arc(x, y, radius, start_angle, end_engle, anti_clockwise) => {
                    self.arc(*x, *y, *radius, *start_angle, *end_engle, *anti_clockwise)
                }
                Instruction::ArcTo(x, y) => self.arc_to(*x, *y),
                Instruction::BeginPath => self.begin_path(),
                Instruction::ClosePath => self.close_path(),
                Instruction::DrawImage(image, x, y) => self.draw_image(*image, *x, *y),
                Instruction::DrawImageD(image, x, y, width, height) => {
                    self.draw_image_d(*image, *x, *y, *width, *height)
                }
                Instruction::DrawImageS(
                    image,
                    s_x,
                    s_y,
                    s_width,
                    s_height,
                    x,
                    y,
                    width,
                    height,
                ) => self.draw_image_s(
                    *image, *s_x, *s_y, *s_width, *s_height, *x, *y, *width, *height,
                ),
                Instruction::Fill(file_rule) => self.fill(*file_rule),
                Instruction::FillRect(x, y, width, height) => {
                    self.fill_rect(*x, *y, *width, *height)
                }
                Instruction::FillText(text, x, y, max_width) => self.fill_text(text, *x, *y, *max_width),
                Instruction::MoveTo(x, y) => self.move_to(*x, *y),
                Instruction::SetFillStyleColor(color) => self.set_fill_style_color(&color),
                Instruction::SetFont(font) => self.set_font(font),
            }
        }
    }
}

pub struct Text {
    instructions: Vec<Instruction>,
}

pub struct Rectangle {
    instructions: Vec<Instruction>,
    // todo: option instruction without border /
}

pub trait Shape2D {
    fn instructions(&self) -> &[Instruction];
}

pub trait Shape2DBuilder {
    fn build(&self) -> Box<Shape2D>;
}

pub struct RectangleBuilder {}

// zoom, rotate, ...

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
