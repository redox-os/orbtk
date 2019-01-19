use std::cell::RefCell;
use orbgl::prelude::Canvas;
use crate::theme::Theme;

use crate::event::EventQueue;

pub struct RenderContext<'a> {
    pub renderer: &'a mut Canvas,
    pub theme: &'a Theme,
    pub event_queue: &'a RefCell<EventQueue>,
}