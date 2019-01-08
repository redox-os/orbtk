//! This module contains all resources to build a backend for OrbTk.
//! A backend is used to open a window, draw on the screen and to call events.
//! This module contains also an `OrbClient` based backend.

use std::cell::{Cell, RefCell};
use std::rc::Rc;

use dces::World;
use orbclient::Color;

use crate::application::Tree;
use crate::event::EventQueue;
use crate::properties::{Bounds, Point};
use crate::theme::Theme;

/// Is used to provides data from the `Backend` to the `RenderSystem`.
pub struct RenderContext<'a> {
    pub renderer: &'a mut dyn Renderer,
    pub theme: &'a Theme,
}

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
    fn render(&mut self, background: Color);
    fn render_rectangle(
        &mut self,
        bounds: &Bounds,
        parent_bounds: &Bounds,
        global_position: &Point,
        border_radius: u32,
        background: Color,
        border_width: u32,
        border_color: Color,
        opacity: f32,
    );
    fn render_text(
        &mut self,
        text: &str,
        bounds: &Bounds,
        parent_bounds: &Bounds,
        global_position: &Point,
        font_size: u32,
        color: Color,
        font: &str,
    );
    fn render_image(
        &mut self,
        image: &[Color],
        bounds: &Bounds,
        parent_bounds: &Bounds,
        global_position: &Point,
    );
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

pub use self::target::target_backend;
pub use self::target::FONT_MEASURE;

#[path = "orbital/mod.rs"]
mod target;
