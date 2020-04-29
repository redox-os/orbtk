#![recursion_limit = "256"]

use std::{any::Any, fmt};

pub mod prelude;

pub use orbtk_utils::prelude as utils;

#[cfg(not(target_arch = "wasm32"))]
// #[path = "pathfinder/mod.rs"]
// pub mod platform;
pub mod concurrent;

#[cfg(not(target_arch = "wasm32"))]
// pub use self::platform::*;
pub use self::concurrent::*;

#[cfg(target_arch = "wasm32")]
#[path = "web/mod.rs"]
pub mod platform;

#[cfg(target_arch = "wasm32")]
pub use platform::RenderContext2D;

#[cfg(not(target_arch = "wasm32"))]
#[path = "raqote/mod.rs"]
pub mod platform;

pub use self::render_target::*;

mod render_target;

/// Defines the current configuration of the render ctx.
#[derive(Debug, Clone)]
pub struct RenderConfig {
    pub fill_style: utils::Brush,
    pub stroke_style: utils::Brush,
    pub line_width: f64,
    pub font_config: FontConfig,
    pub alpha: f32,
}

impl Default for RenderConfig {
    fn default() -> Self {
        RenderConfig {
            fill_style: utils::Brush::default(),
            stroke_style: utils::Brush::default(),
            line_width: 1.,
            font_config: FontConfig::default(),
            alpha: 1.,
        }
    }
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

pub trait RenderPipeline {
    /// Draws the ctx of the pipeline.
    fn draw(&self, image: &mut RenderTarget);
}

/// Used to implement a custom render pipeline.
pub trait Pipeline: RenderPipeline + Any + Send {
    /// Equality for two Pipeline objects.
    fn box_eq(&self, other: &dyn Any) -> bool;

    /// Converts self to an any reference.
    fn as_any(&self) -> &dyn Any;

    /// Clones self as box.
    fn clone_box(&self) -> Box<dyn Pipeline>;

    /// Draws the ctx of the pipeline.
    fn draw_pipeline(&self, image: &mut RenderTarget) {
        self.draw(image);
    }
}

impl PartialEq for Box<dyn Pipeline> {
    fn eq(&self, other: &Box<dyn Pipeline>) -> bool {
        self.box_eq(other.as_any())
    }
}

impl Clone for Box<dyn Pipeline> {
    fn clone(&self) -> Self {
        self.clone_box()
    }
}

impl fmt::Debug for Box<dyn Pipeline> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Box<dyn Pipeline>")
    }
}
