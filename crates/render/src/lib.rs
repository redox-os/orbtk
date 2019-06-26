#![recursion_limit = "128"]

pub mod prelude;

pub use orbtk_utils::prelude as utils;

#[cfg(not(target_arch = "wasm32"))]
#[cfg(feature = "experimental")]
#[path = "pathfinder/mod.rs"]
pub mod platform;

#[cfg(not(target_arch = "wasm32"))]
#[cfg(not(feature = "experimental"))]
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
