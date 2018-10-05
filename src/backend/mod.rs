
use {Rect, Selector, Theme, EventQueue, World, Tree};

#[cfg(target_arch = "wasm32")]
pub use self::wasm::*;
#[cfg(target_arch = "wasm32")]
mod wasm;

#[cfg(not(target_arch = "wasm32"))]
pub use self::orbital::*;
#[cfg(not(target_arch = "wasm32"))]
mod orbital;

pub struct RenderContext<'a> {
    pub renderer: &'a mut Renderer,
    pub theme: &'a Theme,
}

pub struct LayoutContext<'a> {
    pub window_size: (u32, u32),
    pub theme: &'a Theme,
}

pub struct EventContext<'a> {
    pub event_queue: &'a EventQueue,
}

pub trait Renderer {
    fn render(&mut self, theme: &Theme);
    fn render_rectangle(&mut self, theme: &Theme, bounds: &Rect, selector: &Selector, offset: (i32, i32));
    fn render_text(&mut self, theme: &Theme, text: &str, bounds: &Rect, selector: &Selector, offset: (i32, i32));
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
    fn run(&mut self);
}

pub use self::target::target_backend;

#[cfg(not(target_arch = "wasm32"))]
#[path="target/orbital.rs"]
mod target;

#[cfg(target_arch = "wasm32")]
#[path="target/wasm.rs"]
mod target;