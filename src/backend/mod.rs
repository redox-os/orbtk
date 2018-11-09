use std::cell::{Cell, RefCell};
use std::rc::Rc;

use dces::World;

use event::EventQueue;
use structs::{Point, Rect};
use theme::{Selector, Theme};
use tree::Tree;

pub struct RenderContext<'a> {
    pub renderer: &'a mut Renderer,
    pub theme: &'a Theme,
}

pub struct LayoutContext<'a> {
    pub window_size: (u32, u32),
    pub theme: &'a Theme,
}

pub struct EventContext<'a> {
    pub event_queue: &'a RefCell<EventQueue>,
}

pub trait Renderer {
    fn render(&mut self, theme: &Theme);
    fn render_rectangle(
        &mut self,
        theme: &Theme,
        bounds: &Rect,
        parent_bounds: &Rect,
        selector: &Selector,
        offset: &Point,
        global_position: &Point,
    );
    fn render_text(
        &mut self,
        theme: &Theme,
        text: &str,
        bounds: &Rect,
        parent_bounds: &Rect,
        selector: &Selector,
        offset: &Point,
        global_position: &Point,
    );
}

pub trait Backend {
    fn drain_events(&mut self);
    fn bounds(&mut self, bounds: &Rect);
    fn size(&self) -> (u32, u32);
    fn render_context(&mut self) -> RenderContext;
    fn layout_context(&mut self) -> LayoutContext;
    fn event_context(&mut self) -> EventContext;
}

pub trait BackendRunner {
    fn world(&mut self, world: World<Tree>);
    fn run(&mut self, update: Rc<Cell<bool>>);
}

pub use self::target::target_backend;

#[path = "orbital/mod.rs"]
mod target;
