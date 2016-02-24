use super::{Event, Renderer};

use std::any::Any;

pub trait Widget : Any {
    fn draw(&self, renderer: &mut Renderer, focused: bool);
    fn event(&self, event: Event, focused: bool, redraw: &mut bool) -> bool;
}
