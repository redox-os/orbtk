#![recursion_limit = "128"]

pub mod prelude;

pub use orbtk_utils::prelude as utils;

#[cfg(target_arch = "wasm32")]
#[path = "web/mod.rs"]
pub mod platform;

#[cfg(not(target_arch = "wasm32"))]
#[path = "raqote/mod.rs"]
pub mod platform;

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
