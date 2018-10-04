use std::sync::{Arc, mpsc::Sender, mpsc::Receiver};

use {Rect, Selector, Theme, EventManager, RenderContainer};

#[cfg(target_arch = "wasm32")]
pub use self::wasm::*;
#[cfg(target_arch = "wasm32")]
mod wasm;

#[cfg(not(target_arch = "wasm32"))]
pub use self::orbital::*;
#[cfg(not(target_arch = "wasm32"))]
mod orbital;

pub trait Renderer {
    fn render(&mut self, theme: &Arc<Theme>);
    fn render_rectangle(
        &mut self,
        bounds: &Rect,
        selector: &Selector,
        offset: (i32, i32),
        theme: &Arc<Theme>,
    );
    fn render_text(
        &mut self,
        text: &str,
        bounds: &Rect,
        selector: &Selector,
        offset: (i32, i32),
        theme: &Arc<Theme>,
    );
}

pub trait Backend {
    fn drain_events(&mut self);
    fn bounds(&mut self, bounds: &Rect);
    fn size(&self) -> (u32, u32);
    fn run(&mut self);
    fn event_sender(&mut self, event_sender: Sender<EventManager>);
    fn render_receiver(&mut self, render_receiver: Receiver<Vec<RenderContainer>>);
}
