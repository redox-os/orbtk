use std::cell::RefCell;

use crate::event::EventQueue;


/// Is used to provides data from the `Backend` to the `EventSystem`.
pub struct EventContext<'a> {
    pub event_queue: &'a RefCell<EventQueue>,
}