use orbclient::Color;
use orbgl::Canvas;

use crate::core::{Brush, ImageElement, Shape2D};

pub enum Instruction2D {
    /// Creates a circular arc centered at (x, y) with a radius of radius. The path starts at startAngle and ends at endAngle.
    Arc {
        x: f64,
        y: f64,
        radius: f64,
        start_angle: f64,
        end_engle: f64,
    },

    /// Adds a circular arc to the current sub-path, using the given control points and radius. The arc is automatically connected to the path's latest point with a straight line, if necessary for the specified parameters.
    ArcTo {
        x1: f64,
        y1: f64,
        x2: f64,
        y2: f64,
        radius: f64,
    },

    /// Starts a new path by emptying the list of sub-paths. Call this when you want to create a new path.
    BeginPath(),

    /// Adds a cubic Bézier curve to the current sub-path. It requires three points: the first two are control points and the third one is the end point. The starting point is the latest point in the current path, which can be changed using MoveTo{} before creating the Bézier curve.
    BezierCurveTo {
        cp1x: f64,
        cp1y: f64,
        cp2x: f64,
        cp2y: f64,
        x: f64,
        y: f64,
    },

    /// Attempts to add a straight line from the current point to the start of the current sub-path. If the shape has already been closed or has only one point, this function does nothing.
    ClosePath(),

    /// Erases the pixels in a rectangular area by setting them to transparent black.
    ClearRect {
        x: f64,
        y: f64,
        width: f64,
        height: f64,
    },

    /// Draws an image.
    DrawImage { image_element: ImageElement },

    /// Fills the current or given path with the current file style.
    Fill(),

    /// Draws a filled rectangle whose starting point is at the coordinates {x, y} with the specified width and height and whose style is determined by the fillStyle attribute.
    FillRect {
        x: f64,
        y: f64,
        width: f64,
        height: f64,
    },

    /// Draws a text string at the specified coordinates, filling the string's characters with the current foreground color. An optional parameter allows specifying a maximum width for the rendered text, which the user agent will achieve by condensing the text or by using a lower font size.
    FillText {
        text: String,
        x: f64,
        y: f64,
        max_width: Option<f64>,
    },

    /// Adds a straight line to the current sub-path by connecting the sub-path's last point to the specified {x, y} coordinates.
    LineTo { x: f64, y: f64 },

    /// Begins a new sub-path at the point specified by the given {x, y} coordinates.
    MoveTo { x: f64, y: f64 },

    /// Restores the most recently saved canvas state by popping the top entry in the drawing state stack. If there is no saved state, this method does nothing.
    Restore(),

    /// Adds a rotation to the transformation matrix.
    Rotate { angle: f64 },

    /// Specifies the brush to use inside shapes.
    SetFillStyleBrush { brush: Brush },

    /// Specifies the current text style being used when drawing text.
    SetFont { font: String },

    /// Sets the thickness of lines.
    SetLineWidth { width: f64 },

    /// Specifies the amount of blur applied to shadows. The default is 0 {no blur}.
    SetShadowBlur { blur: f64 },

    /// Specifies the color of shadows.
    SetShadowColor { color: String },

    /// Specifies the distance that shadows will be offset horizontally.
    SetShadowOffsetX { x: f64 },

    /// Specifies the distance that shadows will be offset vertically.
    SetShadowOffsetY { y: f64 },

    /// Specifies the color or style to use for the lines around shapes. The default is #000 {black}.
    SetStrokeStyleColor { color: String },

    /// Saves the entire state of the canvas by pushing the current state onto a stack.
    Save(),

    /// Adds a scaling transformation to the canvas units horizontally and/or vertically.
    Scale { x: f64, y: f64 },

    /// Strokes {outlines} the current or given path with the current stroke style.
    Stroke(),

    /// Multiplies the current transformation with the matrix described by the arguments of this method. You are able to scale, rotate, move and skew the context.
    Transform {
        a: f64,
        b: f64,
        c: f64,
        d: f64,
        e: f64,
        f: f64,
    },

    /// Adds a translation transformation to the current matrix.
    Translate { x: f64, y: f64 },
}

pub trait RenderContext2D {
    fn render_shape(&mut self, shape: &Shape2D) {
        self.render_instructions(shape.instructions());
    }

    fn render_instructions(&mut self, instructions: &[Instruction2D]);
}

impl RenderContext2D for Canvas {
    fn render_instructions(&mut self, instructions: &[Instruction2D]) {
        for instruction in instructions {
            match instruction {
                Instruction2D::Arc{x, y, radius, start_angle, end_engle} => {
                    self.arc(*x, *y, *radius, *start_angle, *end_engle)
                }
                Instruction2D::ArcTo{x1, y1, x2, y2, radius} => println!("RenderContext2D: 'ArcTo' is not implemented."),
                Instruction2D::BeginPath() => self.begin_path(),
                Instruction2D::BezierCurveTo {
                    cp1x,
                    cp1y,
                    cp2x,
                    cp2y,
                    x,
                    y,
                } => self.bezier_curve_to(
                    *cp1x,
                    *cp1y,
                    *cp2x,
                    *cp2y,
                    *x,
                    *y,
                ),
                Instruction2D::ClearRect {
                    x,
                    y,
                    width,
                    height,
                } => self.clear_rect(*x, *y, *width, *height),
                Instruction2D::ClosePath() => self.close_path(),
                Instruction2D::DrawImage { image_element } => println!("RenderContext2D: 'DrawImage' is not implemented."),
                Instruction2D::Fill () => self.fill(),
                Instruction2D::FillRect {
                    x,
                    y,
                    width,
                    height,
                } => self.fill_rect(*x, *y, *width, *height),
                Instruction2D::FillText {
                    text,
                    x,
                    y,
                    max_width,
                } => println!("RenderContext2D: 'FillText' is not implemented."),
                Instruction2D::LineTo { x, y } => self.line_to(*x, *y),
                Instruction2D::MoveTo { x, y } => self.move_to(*x, *y),
                Instruction2D::Restore() => self.restore(),
                Instruction2D::Rotate { angle } => self.rotate(*angle),
                Instruction2D::SetFillStyleBrush { brush } => match brush {
                    Brush::SolidColor(color) => self.set_fill_style(get_color(color)),
                    Brush::Gradient(gradient) => {}
                },
                Instruction2D::SetStrokeStyleColor { color } => println!("RenderContext2D: 'SetStrokeStyleColor' is not implemented."),
                Instruction2D::SetFont { font } => println!("RenderContext2D: 'SetFont' is not implemented."),
                Instruction2D::SetLineWidth { width } => println!("RenderContext2D: 'SetLineWidth' is not implemented."),
                Instruction2D::SetShadowBlur { blur } => println!("RenderContext2D: 'SetShadowBlur' is not implemented."),
                Instruction2D::SetShadowColor { color } => println!("RenderContext2D: 'SetShadowColor' is not implemented."),
                Instruction2D::SetShadowOffsetX { x } => println!("RenderContext2D: 'SetShadowOffsetX' is not implemented."),
                Instruction2D::SetShadowOffsetY { y } => println!("RenderContext2D: 'SetShadowOffsetY' is not implemented."),
                Instruction2D::Save() => self.save(),
                Instruction2D::Scale { x, y } => self.scale(*x, *y),
                Instruction2D::Stroke() => self.stroke(),
                Instruction2D::Transform { a, b, c, d, e, f } => self.transform(
                    *a, *b, *c, *d, *e, *f,
                ),
                Instruction2D::Translate { x, y } => self.translate(*x, *y),
                _ => {}
            }
        }
    }
}

fn get_color(hex: &str) -> Color {
    let clean_hex = hex.trim_start_matches("#");
    match clean_hex.len() {
        6 | 8 => {
            let mut x = match u32::from_str_radix(&clean_hex, 16) {
                Ok(x) => x,
                Err(_) => 0,
            };

            if clean_hex.len() == 6 {
                x |= 0xFF_000_000;
            }

            Color { data: x }
        }
        _ => Color { data: 0 },
    }
}
