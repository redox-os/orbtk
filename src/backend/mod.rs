//! This module contains all resources to build a backend for OrbTk.
//! A backend is used to open a window, draw on the screen and to call events.
//! This module contains also an `OrbClient` based backend.

use std::{
    cell::{Cell, RefCell},
    collections::BTreeMap,
    rc::Rc,
};

use dces::prelude::{Entity, World};
use orbclient::Color;
use orbfont::Font;
use orbgl_api::prelude::Canvas;

use crate::prelude::*;

/// Provides the context for the `InitSystem`.
pub struct InitContext<'a> {
    pub theme: &'a Theme,
}

/// Is used to provides data from the `Backend` to the `RenderSystem`.
pub struct RenderContext<'a> {
    pub canvas: &'a mut Canvas,
    pub renderer: &'a mut dyn Renderer,
    pub theme: &'a Theme,
    pub event_queue: &'a RefCell<EventQueue>,
}

/// Is used to provides data from the `Backend` to the `LayoutSystem`.
pub struct LayoutContext<'a> {
    pub window_size: (u32, u32),
    pub theme: &'a Theme,
}

/// Is used to provides data from the `Backend` to the `StateSystem` and `PostLayoutStateSystem`.
pub struct StateContext<'a> {
    pub theme: &'a Theme,
    pub event_queue: &'a RefCell<EventQueue>,
    pub messages: &'a RefCell<BTreeMap<Entity, Vec<MessageBox>>>,
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
        font: &Font,
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
    fn init_context(&mut self) -> InitContext<'_>;
    fn render_context(&mut self) -> RenderContext<'_>;
    fn layout_context(&mut self) -> LayoutContext<'_>;
    fn event_context(&mut self) -> EventContext<'_>;
    fn state_context(&mut self) -> StateContext<'_>;
}

/// This trait is used to create a backend runner.
pub trait BackendRunner {
    fn world(&mut self, world: World<Tree>);
    fn run(&mut self, update: Rc<Cell<bool>>, running: Rc<Cell<bool>>);
}

/// Helper trait to measure the font size of the given `text`.
pub trait FontMeasure {
    fn measure(&self, text: &str, font: &Font, font_size: u32) -> (u32, u32);
}

pub use self::target::target_backend;
pub use self::target::FONT_MEASURE;

#[cfg(target_arch = "wasm32")]
#[path = "web/mod.rs"]
mod target;

#[cfg(not(target_arch = "wasm32"))]
#[path = "orbital/mod.rs"]
mod target;
