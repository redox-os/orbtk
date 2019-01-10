pub use self::shapes::*;
pub use self::structs::*;

mod shapes;
mod structs;

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
    /// Creates a circular arc centered at (x, y) with a radius of radius. The path starts at startAngle and ends at endAngle, and travels in the direction given by anticlockwise (defaulting to clockwise).
    Arc(f64, f64, f64, f64, f64, bool),

    /// Adds a circular arc to the current sub-path, using the given control points and radius. The arc is automatically connected to the path's latest point with a straight line, if necessary for the specified parameters.
    ArcTo(f64, f64),

    /// Starts a new path by emptying the list of sub-paths. Call this when you want to create a new path.
    BeginPath(),

    /// Adds a cubic Bézier curve to the current sub-path. It requires three points: the first two are control points and the third one is the end point. The starting point is the latest point in the current path, which can be changed using MoveTo() before creating the Bézier curve.
    BezierCurveTo(f64, f64, f64, f64, f64, f64),

    /// Attempts to add a straight line from the current point to the start of the current sub-path. If the shape has already been closed or has only one point, this function does nothing.
    ClosePath(),

    /// Erases the pixels in a rectangular area by setting them to transparent black.
    ClearRect(f64, f64, f64, f64),

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

    /// Adds a straight line to the current sub-path by connecting the sub-path's last point to the specified (x, y) coordinates.
    LineTo(f64, f64),

    /// Begins a new sub-path at the point specified by the given (x, y) coordinates.
    MoveTo(f64, f64),

    /// Restores the most recently saved canvas state by popping the top entry in the drawing state stack. If there is no saved state, this method does nothing.
    Restore(),

    /// Adds a rotation to the transformation matrix.
    Rotate(f64),

    /// Specifies the brush to use inside shapes.
    SetFillStyleBrush(Brush),

    /// Specifies the current text style being used when drawing text. 
    SetFont(String),

    /// Sets the thickness of lines.
    SetLineWidth(f64),

    /// Specifies the amount of blur applied to shadows. The default is 0 (no blur).
    SetShadowBlur(f64),

    /// Specifies the color of shadows.
    SetShadowColor(String),

    /// Specifies the distance that shadows will be offset horizontally.
    SetShadowOffsetX(f64),

    /// Specifies the distance that shadows will be offset vertically.
    SetShadowOffsetY(f64),

    /// Specifies the color or style to use for the lines around shapes. The default is #000 (black).
    SetStrokeStyleColor(String),

    /// Saves the entire state of the canvas by pushing the current state onto a stack.
    Save(),

    /// Adds a scaling transformation to the canvas units horizontally and/or vertically.
    Scale(f64, f64),

    /// Strokes (outlines) the current or given path with the current stroke style.
    Stroke(),

    /// Multiplies the current transformation with the matrix described by the arguments of this method. You are able to scale, rotate, move and skew the context.
    Transform(f64, f64, f64, f64, f64, f64),

    /// Adds a translation transformation to the current matrix.
    Translate(f64, f64),
}


/// setTransform

/// translater
pub trait RenderContext2D {
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

    /// Adds a cubic Bézier curve to the current sub-path. It requires three points: the first two are control points and the third one is the end point. The starting point is the latest point in the current path, which can be changed using MoveTo() before creating the Bézier curve.
    fn bezier_curve_to(&mut self, cp1x: f64, cp1y: f64, cp2x: f64, cp2y: f64, x: f64, y: f64);

    /// Erases the pixels in a rectangular area by setting them to transparent black.
    fn clear_rect(&mut self, x: f64, y: f64, width: f64, height: f64);

    /// Attempts to add a straight line from the current point to the start of the current sub-path. If the shape has already been closed or has only one point, this function does nothing.
    fn close_path(&mut self);

    /// Draws an image on (x, y).
    fn draw_image(&mut self, image_element: &ImageElement, x: f64, y: f64);

    /// Draws an image on (x, y) with (width, height).
    fn draw_image_d(
        &mut self,
        image_element: &ImageElement,
        x: f64,
        y: f64,
        width: f64,
        height: f64,
    );

    /// Draws a part of the image with the given (source_x, source_y, source_width, source_height) on (x, y) with (width, height).
    fn draw_image_s(
        &mut self,
        image_element: &ImageElement,
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

    /// Adds a straight line to the current sub-path by connecting the sub-path's last point to the specified (x, y) coordinates.
    fn line_to(&mut self, x: f64, y: f64);

    /// Begins a new sub-path at the point specified by the given (x, y) coordinates.
    fn move_to(&mut self, x: f64, y: f64);

    /// Restores the most recently saved canvas state by popping the top entry in the drawing state stack. If there is no saved state, this method does nothing.
    fn restore(&mut self);

    /// Adds a rotation to the transformation matrix.
    fn rotate(&mut self, angle: f64);

    /// Specifies the color to use inside shapes.
    fn set_fill_style_brush(&mut self, color: &Brush);

    /// Specifies the current text style being used when drawing text. 
    fn set_font(&mut self, font: &str);

    /// Sets the thickness of lines.
    fn set_line_width(&mut self, width: f64);

    /// Specifies the amount of blur applied to shadows. The default is 0 (no blur).
    fn set_shadow_blur(&mut self, blur: f64);

    /// Specifies the color of shadows.
    fn set_shadow_color(&mut self, color: &str);

    /// Specifies the distance that shadows will be offset horizontally.
    fn set_shadow_offset_x(&mut self, x: f64);

    /// Specifies the distance that shadows will be offset vertically.
    fn set_shadow_offset_y(&mut self, y: f64);

    /// Specifies the color or style to use for the lines around shapes. The default is #000 (black).
    fn set_stroke_style_color(&mut self, color: &str);

    /// Saves the entire state of the canvas by pushing the current state onto a stack.
    fn save(&mut self);

    /// Adds a scaling transformation to the canvas units horizontally and/or vertically.
    fn scale(&mut self, x: f64, y: f64);

    /// Strokes (outlines) the current or given path with the current stroke style.
    fn stroke(&mut self);

    /// Multiplies the current transformation with the matrix described by the arguments of this method. You are able to scale, rotate, move and skew the context.
    fn transform(&mut self, a: f64, b: f64, c: f64, d: f64, e: f64, f: f64);

    /// Adds a translation transformation to the current matrix.
    fn translate(&mut self, x: f64, y: f64);

    /// Returns a `TextMetrics` object that contains information about the measured text (such as its width for example).
    fn measure_text(&self, text: &str) -> TextMetrics;

    /// Finish the drawing.
    fn finish(&self);

    /// Registers a new font from a path.
    fn register_font(&mut self, path: &str);

    /// Renders a 2D Shape.
    fn render_shape(&mut self, shape: &Shape2D) {
        self.render(shape.instructions());
    }

    /// Translates the render instructions to render methods of `RenderContext`.
    fn render(&mut self, instructions: &[Instruction]) {
        for instruction in instructions {
            match instruction {
                Instruction::Arc(x, y, radius, start_angle, end_engle, anti_clockwise) => {
                    self.arc(*x, *y, *radius, *start_angle, *end_engle, *anti_clockwise)
                }
                Instruction::ArcTo(x, y) => self.arc_to(*x, *y),
                Instruction::BeginPath() => self.begin_path(),
                Instruction::BezierCurveTo(cp1x, cp1y, cp2x, cp2y, x, y) => self.bezier_curve_to(*cp1x, *cp1y, *cp2x, *cp2y, *x, *y),
                Instruction::ClearRect(x, y, width, height) => self.clear_rect(*x, *y, *width, *height),
                Instruction::ClosePath() => self.close_path(),
                Instruction::DrawImage(image, x, y) => self.draw_image(image, *x, *y),
                Instruction::DrawImageD(image, x, y, width, height) => {
                    self.draw_image_d(image, *x, *y, *width, *height)
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
                    image, *s_x, *s_y, *s_width, *s_height, *x, *y, *width, *height,
                ),
                Instruction::Fill(file_rule) => self.fill(*file_rule),
                Instruction::FillRect(x, y, width, height) => {
                    self.fill_rect(*x, *y, *width, *height)
                }
                Instruction::FillText(text, x, y, max_width) => self.fill_text(text, *x, *y, *max_width),
                Instruction::LineTo(x, y) => self.line_to(*x, *y),
                Instruction::MoveTo(x, y) => self.move_to(*x, *y),
                Instruction::Restore() => self.restore(),
                Instruction::Rotate(angle) => self.rotate(*angle),
                Instruction::SetFillStyleBrush(brush) => self.set_fill_style_brush(&brush),
                Instruction::SetStrokeStyleColor(color) => self.set_stroke_style_color(&color),
                Instruction::SetFont(font) => self.set_font(font),
                Instruction::SetLineWidth(width) => self.set_line_width(*width),
                Instruction::SetShadowBlur(blur) => self.set_shadow_blur(*blur),
                Instruction::SetShadowColor(color) => self.set_shadow_color(color),
                Instruction::SetShadowOffsetX(x) => self.set_shadow_offset_x(*x),
                Instruction::SetShadowOffsetY(y) => self.set_shadow_offset_y(*y),
                Instruction::Save() => self.save(),
                Instruction::Scale(x, y) => self.scale(*x, *y),
                Instruction::Stroke() => self.stroke(),
                Instruction::Transform(a, b, c, d, e, f) => self.transform(*a, *b, *c, *d, *e, *f),
                Instruction::Translate(x, y) => self.translate(*x, *y),
            }
        }

        self.finish();
    }
}