#![recursion_limit = "128"]

use std::{any::Any, fmt};

pub mod prelude;

pub use orbtk_utils::prelude as utils;

#[cfg(not(target_arch = "wasm32"))]
pub mod concurrent;

#[cfg(not(target_arch = "wasm32"))]
pub use self::concurrent::*;

#[cfg(target_arch = "wasm32")]
#[path = "web/mod.rs"]
pub mod platform;

#[cfg(target_arch = "wasm32")]
pub use platform::RenderContext;

#[cfg(not(target_arch = "wasm32"))]
#[path = "raqote/mod.rs"]
pub mod platform;

pub use self::render_target::*;

mod render_target;

/// Defines the current configuration of the render context.
#[derive(Default, Debug, Clone)]
pub struct RenderConfig {
    pub fill_style: utils::Brush,
    pub stroke_style: utils::Brush,
    pub line_width: f64,
    pub font_config: FontConfig,
}

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

/// Use this to implement a 2D render context.
pub trait RenderContext2D {
    // Rectangles

    /// Draws a filled rectangle whose starting point is at the coordinates {x, y} with the specified width and height and whose style is determined by the fillStyle attribute.
    fn fill_rect(&mut self, x: f64, y: f64, width: f64, height: f64);

    /// Draws a rectangle that is stroked (outlined) according to the current strokeStyle and other context settings.
    fn stroke_rect(&mut self, x: f64, y: f64, width: f64, height: f64);

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
    fn arc(&mut self, x: f64, y: f64, radius: f64, start_angle: f64, end_angle: f64);

    /// Begins a new sub-path at the point specified by the given {x, y} coordinates.

    fn move_to(&mut self, x: f64, y: f64);

    /// Adds a straight line to the current sub-path by connecting the sub-path's last point to the specified {x, y} coordinates.
    fn line_to(&mut self, x: f64, y: f64);

    /// Adds a quadratic Bézier curve to the current sub-path.
    fn quadratic_curve_to(&mut self, cpx: f64, cpy: f64, x: f64, y: f64);

    /// Adds a cubic Bézier curve to the current sub-path. It requires three points: the first two are control points and the third one is the end point. The starting point is the latest point in the current path, which can be changed using MoveTo{} before creating the Bézier curve.
    fn bezier_curve_to(&mut self, cp1x: f64, cp1y: f64, cp2x: f64, cp2y: f64, x: f64, y: f64);

    fn clip(&mut self);

    // Line styles

    /// Sets the thickness of lines.
    fn set_line_width(&mut self, line_width: f64);
}

/// Used to implement a custom render pipeline.
pub trait RenderPipeline: Any + Send {
    /// Equality for two RenderPipeline objects.
    fn box_eq(&self, other: &dyn Any) -> bool;

    /// Converts self to an any reference.
    fn as_any(&self) -> &dyn Any;

    /// Clones self as box.
    fn clone_box(&self) -> Box<dyn RenderPipeline>;

    /// Draws the context of the pipeline.
    fn draw_pipeline(&self, image: &mut RenderTarget);
}

impl PartialEq for Box<dyn RenderPipeline> {
    fn eq(&self, other: &Box<dyn RenderPipeline>) -> bool {
        self.box_eq(other.as_any())
    }
}

impl Clone for Box<dyn RenderPipeline> {
    fn clone(&self) -> Self {
        self.clone_box()
    }
}

impl fmt::Debug for Box<dyn RenderPipeline> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Box<dyn RenderPipeline>")
    }
}
