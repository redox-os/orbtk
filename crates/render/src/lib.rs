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
pub use platform::RenderContext2D;

#[cfg(not(target_arch = "wasm32"))]
#[path = "raqote/mod.rs"]
pub mod platform;

pub use euc as three;

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

/// Used to implement a 3D render pipeline to render 3D objects.
pub trait ThreePipeline: Any {
    /// Equality for two ThreePipeline objects.
    fn box_eq(&self, other: &dyn Any) -> bool;

    /// Converts self to an any reference.
    fn as_any(&self) -> &dyn Any;

    /// Clones self as box.
    fn clone_box(&self) -> Box<ThreePipeline>;

    /// Draws the context of the pipeline.
    fn draw(
        &self,
        buffer: &mut three::buffer::Buffer2d<f64>,
        depth: &mut three::buffer::Buffer2d<f64>,
    );
}

impl PartialEq for Box<dyn ThreePipeline> {
    fn eq(&self, other: &Box<dyn ThreePipeline>) -> bool {
        self.box_eq(other.as_any())
    }
}

impl Clone for Box<dyn ThreePipeline> {
    fn clone(&self) -> Self {
        self.clone_box()
    }
}

impl fmt::Debug for Box<dyn ThreePipeline> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Box<ThreePipeline>")
    }
}