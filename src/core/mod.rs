//! This module contains all resources to build a backend for OrbTk.
//! A backend is used to open a window, draw on the screen and to call events.
//! This module contains also an `OrbClient` based backend.

use std::{cell::Cell, rc::Rc};

use dces::World;

use crate::{application::Tree, properties::Bounds};

/// This trait is used to define a backend for OrbTk.
pub trait Backend {
    fn drain_events(&mut self);
    fn bounds(&mut self, bounds: &Bounds);
    fn size(&self) -> (u32, u32);
    fn render_context(&mut self) -> RenderContext<'_>;
    fn layout_context(&mut self) -> LayoutContext<'_>;
    fn event_context(&mut self) -> EventContext<'_>;
    fn state_context(&mut self) -> StateContext<'_>;
    fn flip(&mut self) -> bool;
}

/// This trait is used to create a backend runner.
pub trait BackendRunner {
    fn world(&mut self, world: World<Tree>);
    fn run(&mut self, update: Rc<Cell<bool>>);
}

/// Helper trait to meassure the font size of the given `text`.
pub trait FontMeasure {
    fn measure(&self, text: &str, font: &str, font_size: u32) -> (u32, u32);
}

pub use self::context::*;
pub use self::target::*;

mod context;
#[path = "orbital/mod.rs"]
mod target;
