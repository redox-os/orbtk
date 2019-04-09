//! This module contains all resources to create and use a window.

use super::events::*;

#[cfg(not(target_arch = "wasm32"))]
#[path = "impl_orbclient.rs"]
mod implementation;

/// Used to define a window.
pub trait Window {
    /// Creates a new `WindowBuilder` with default values.
    fn create() -> WindowBuilder {
        WindowBuilder::new()
    }

    /// Sets the title.
    fn set_title(&mut self, title: impl Into<String>);

    /// Gets the title.
    fn title(&self) -> String;

    /// Sets the position.
    fn set_position(&mut self, x: f64, y: f64);

    /// Gets the position.
    fn position(&self) -> (f64, f64);

    /// Sets the size.
    fn set_size(&mut self, width: f64, height: f64);

    /// Gets the size.
    fn size(&self) -> (f64, f64);

    /// Swaps the buffers.
    fn sync(&mut self);

    /// Request current window events.
    fn events(&mut self) -> Vec<Event>;
}

/// Used to build a window, specifying additional details.
#[derive(Default, Debug, PartialEq)]
pub struct WindowBuilder  {
    title: String,
    size: (f64, f64),
    position: (f64, f64),
    resizable: bool,
}

impl WindowBuilder {
    /// Creates a new border builder with default values.
    pub fn new() -> WindowBuilder {
        WindowBuilder::default()
    }

    pub fn title(mut self, title: impl Into<String>) -> Self {
        self.title = title.into();
        self
    }

    /// Inserts the window bounds.
    pub fn bounds(mut self, bounds: impl Into<(f64, f64, f64, f64)>) -> Self {
        let bounds = bounds.into();
        self.position = (bounds.0, bounds.1);
        self.size = (bounds.2, bounds.3);
        self
    }

    /// Inserts a position.
    pub fn position(mut self, position: impl Into<(f64, f64)>) -> Self {
        self.position =  position.into();
        self
    }

    /// Inserts a size.
    pub fn size(mut self, size: impl Into<(f64, f64)>) -> Self {
        self.size = size.into();
        self
    }

    /// Inserts a value that indicates wether the window is resizable.
    pub fn resizable(mut self, resizable: impl Into<bool>) -> Self {
        self.resizable = resizable.into();
        self
    } 
}