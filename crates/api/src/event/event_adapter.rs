use std::sync::{Arc, Mutex};

use dces::entity::Entity;

use crate::event::*;

#[derive(Clone, Default, Debug)]
pub struct EventAdapter {
    event_queue: Arc<Mutex<EventQueue>>,
}

impl EventAdapter {
    pub fn new() -> Self {
        EventAdapter::default()
    }

    pub fn push_event<E: Event>(&self, entity: Entity, event: E) {
        self.event_queue
            .lock()
            .expect("EventAdapter::push_event: Cannot lock event queue.")
            .register_event(event, entity);
    }

    pub fn push_event_direct<E: Event>(&self, entity: Entity, event: E) {
        self.event_queue
            .lock()
            .expect("EventAdapter::push_event_direct: Cannot lock event queue")
            .register_event_with_strategy(event, EventStrategy::Direct, entity);
    }

    pub fn len(&self) -> usize {
        self.event_queue
            .lock()
            .expect("EventAdapter::len: Cannot lock event queue.")
            .len()
    }

    pub fn is_empty(&self) -> bool {
        self.event_queue
            .lock()
            .expect("EventAdapter::is_empty: Cannot lock event queue.")
            .is_empty()
    }

    pub(crate) fn dequeue(&self) -> DequeueIterator {
        DequeueIterator {
            event_adapter: self.clone(),
        }
    }
}

pub struct DequeueIterator {
    event_adapter: EventAdapter,
}

impl Iterator for DequeueIterator {
    type Item = EventBox;

    fn next(&mut self) -> Option<EventBox> {
        self.event_adapter
            .event_queue
            .lock()
            .expect("DequeueIterator::next: Cannot lock event queue.")
            .dequeue()
    }
}
