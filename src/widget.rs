use super::{Event, Renderer};

use std::any::Any;

pub trait Widget : Any {
    fn draw(&self, renderer: &mut Renderer);
    fn event(&self, event: Event);
    fn position(&mut self, x: isize, y: isize) -> &mut Self;
}
