#![recursion_limit = "128"]

pub mod prelude;

pub use orbtk_utils::prelude as utils;

use utils::*;

#[cfg(not(target_arch = "wasm32"))]
#[path = "orbclient/mod.rs"]
pub mod platform;

#[cfg(target_arch = "wasm32")]
#[path = "web/mod.rs"]
pub mod platform;

/// The TextMetrics struct represents the dimension of a text.
#[derive(Clone, Copy, Default, Debug)]
pub struct TextMetrics {
    pub width: f64,
    pub height: f64,
}

// Internal font helper.
#[derive(Default, Clone, PartialEq, Debug)]
pub struct FontConfig {
    pub family: String,
    pub font_size: f64,
}

impl ToString for FontConfig {
    fn to_string(&self) -> String {
        format!("{}px {}", self.font_size, self.family)
    }
}

/// The RenderContext2D trait, provides the 2D rendering context. It is used for drawing shapes, text, images, and other objects.
pub trait RenderContext2D {
    // Rectangles

    /// Draws a filled rectangle whose starting point is at the coordinates {x, y} with the specified width and height and whose style is determined by the fillStyle attribute.
    fn fill_rect(&mut self, x: f64, y: f64, width: f64, height: f64);

    /// Draws a rectangle that is stroked (outlined) according to the current strokeStyle and other context settings.
    fn stroke_rect(&mut self, x: f64, y: f64, width: f64, height: f64);

    // Text

    /// Draws (fills) a given text at the given (x, y) position.
    fn fill_text(&mut self, text: &str, x: f64, y: f64, max_width: Option<f64>);

    /// Draws (strokes) a given text at the given (x, y) position.
    fn stroke_text(&mut self, text: &str, x: f64, y: f64, max_width: Option<f64>);

    /// Returns a TextMetrics object.
    fn measure_text(&mut self, text: &str) -> TextMetrics;

    // Draw Path

    /// Fills the current or given path with the current file style.
    fn fill(&mut self);

    /// Strokes {outlines} the current or given path with the current stroke style.
    fn stroke(&mut self);

    /// Starts a new path by emptying the list of sub-paths. Call this when you want to create a new path.
    fn begin_path(&mut self);

    /// Attempts to add a straight line from the current point to the start of the current sub-path. If the shape has already been closed or has only one point, this function does nothing.
    fn close_path(&mut self);

    /// Adds a rectangle to the current path.
    fn rect(&mut self, x: f64, y: f64, width: f64, height: f64);

    /// Creates a circular arc centered at (x, y) with a radius of radius. The path starts at startAngle and ends at endAngle.
    fn arc(
        &mut self,
        x: f64,
        y: f64,
        radius: f64,
        start_angle: f64,
        end_angle: f64,
        anticlockwise: bool,
    );

    /// Begins a new sub-path at the point specified by the given {x, y} coordinates.
    fn move_to(&mut self, x: f64, y: f64);

    /// Adds a straight line to the current sub-path by connecting the sub-path's last point to the specified {x, y} coordinates.
    fn line_to(&mut self, x: f64, y: f64);

    /// Adds a quadratic Bézier curve to the current sub-path.
    fn quadratic_curve_to(&mut self, cpx: f64, cpy: f64, x: f64, y: f64);

    /// Adds a cubic Bézier curve to the current sub-path. It requires three points: the first two are control points and the third one is the end point. The starting point is the latest point in the current path, which can be changed using MoveTo{} before creating the Bézier curve.
    fn bezier_curve_to(&mut self, cp1x: f64, cp1y: f64, cp2x: f64, cp2y: f64, x: f64, y: f64);

    // Draw image

    /// Draws the image.
    fn draw_image(&mut self, path: &str, dx: f64, dy: f64);

    /// Draws the image with the given size.
    fn draw_image_width_size(&mut self, path: &str);

    /// Draws the given part of the image.
    fn draw_image_with_clip_and_size(
        &mut self,
        path: &str,
        x: f64,
        y: f64,
        width: f64,
        height: f64,
    );

    // Line styles

    /// Sets the thickness of lines.
    fn set_line_width(
        &mut self,
        clip_x: f64,
        clip_y: f64,
        clip_width: f64,
        clip_height: f64,
        x: f64,
        y: f64,
        width: f64,
        height: f64,
    );

    // fn set_line_cap(&mut self);

    // fn set_line_join(&mut self);

    // fn set_miter_limit(&mut self);

    // fn set_line_dash(&mut self);

    // fn set_line_dash_offset(&mut self);

    // Text styles

    /// Specific the font family.
    fn set_font_family(&mut self, family: &str);

    /// Specifies the font size.
    fn set_font_size(&mut self, size: f64);

    /// Specifies the text alignment.
    fn set_text_align(&mut self, alignment: TextAlignment);

    // Fill and stroke style

    /// Specifies the fill color to use inside shapes.
    fn set_fill_style(&mut self, brush: Brush);

    /// Specifies the fill stroke to use inside shapes.
    fn set_stroke_style(&mut self, brush: Brush);

    // Shadows

    fn set_shadow_color(&mut self, color: Color);

    fn set_shadow_offset(&mut self, x: f64, y: f64);

    // Transformations

    /// Multiplies the current transformation with the matrix described by the arguments of this method. You are able to scale, rotate, move and skew the context.
    fn transform(&mut self, a: f64, b: f64, c: f64, d: f64, e: f64, f: f64);

    /// Sets the tranformation.
    fn set_transform(&mut self, a: f64, b: f64, c: f64, d: f64, e: f64, f: f64);

    // Canvas states

    /// Saves the entire state of the canvas by pushing the current state onto a stack.
    fn save(&mut self);

    /// Restores the most recently saved canvas state by popping the top entry in the drawing state stack. If there is no saved state, this method does nothing.
    fn restore(&mut self);
}
