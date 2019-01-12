//! This module contains all resources to build a backend for OrbTk.
//! A backend is used to open a window, draw on the screen and to call events.
//! This module contains also an `OrbClient` based backend.

use std::{
    cell::{Cell, RefCell},
    rc::Rc,
};

use dces::World;
use orbclient::Color;

use crate::{
    application::Tree,
    event::EventQueue,
    properties::{Bounds, Point},
    theme::Theme,
};

/// Is used to provides data from the `Backend` to the `LayoutSystem`.
pub struct LayoutContext<'a> {
    pub window_size: (u32, u32),
    pub theme: &'a Theme,
}

/// Is used to provides data from the `Backend` to the `StateSystem` and `PostLayoutStateSystem`.
pub struct StateContext<'a> {
    pub theme: &'a Theme,
}

/// Is used to provides data from the `Backend` to the `EventSystem`.
pub struct EventContext<'a> {
    pub event_queue: &'a RefCell<EventQueue>,
}

/// This trait is used to define a backend renderer for OrbTk.
pub trait Renderer {
    fn render_shape(&mut self, shape: &Shape2D);
}

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

pub use self::target::*;
pub use self::render_context::*;

#[path = "orbital/mod.rs"]
mod target;
mod render_context;
