use std::any::{Any, TypeId};

pub struct EventBox {
    pub event: Box<Any>,
    pub event_type: TypeId,
}

impl EventBox {
    pub fn new<E: Any>(event: E) -> Self {
        EventBox {
            event: Box::new(event),
            event_type: TypeId::of::<E>(),
        }
    }

    pub fn consume(self) -> Box<Any> {
        self.event
    }
}

#[derive(Default)]
pub struct EventManager {
    event_queue: Vec<EventBox>,
}

impl EventManager {
    pub fn enqueue(&mut self, event_box: EventBox) {
        self.event_queue.push(event_box);
    }

    pub fn dequeue<E: Any>(&mut self) -> Option<E> {
        if self.event_queue.len() > 0 {
            return Some(
                *self
                    .event_queue
                    .remove(0)
                    .consume()
                    .downcast::<E>()
                    .unwrap(),
            );
        }

        None
    }
}

// todo state as component include other components and change it.
