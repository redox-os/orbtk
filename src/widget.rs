use super::{Color, Event, Rect, Renderer, Window};

use std::any::Any;
use std::sync::Arc;
use std::cell::Cell;

pub struct WidgetCore {
    pub rect: Cell<Rect>,
    pub bg: Color,
    pub fg: Color,
}

impl WidgetCore {
    pub fn new(bg: Color, fg: Color) -> Self {
        WidgetCore {
            rect: Cell::new(Rect::default()),
            bg: bg,
            fg: fg,
        }
    }
}

pub trait Widget : Any {
    fn draw(&self, renderer: &mut Renderer, focused: bool);
    fn event(&self, event: Event, focused: bool, redraw: &mut bool) -> bool;
}

pub trait WidgetPlace
    where Self: Widget + Sized
{
    fn place(self, window: &Window) -> Arc<Self> {
        let arc = Arc::new(self);
        let mut widgets = window.widgets.borrow_mut();

        widgets.push(arc.clone());

        arc
    }
}
