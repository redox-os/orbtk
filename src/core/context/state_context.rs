use std::cell::RefCell;

use crate::{event::EventQueue, theme::Theme};

/// Is used to provides data from the `Backend` to the `StateSystem` and `PostLayoutStateSystem`.
pub struct StateContext<'a> {
    pub theme: &'a Theme,
    pub event_queue: &'a RefCell<EventQueue>,
}
