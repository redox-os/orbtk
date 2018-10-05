use std::any::{Any, TypeId};

#[derive(Debug)]
pub enum EventError {
    WrongType(TypeId)
}

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

    pub fn is_type<E: Any>(&self) -> bool {
        self.event_type == TypeId::of::<E>()
    }

    pub fn downcast<E: Any>(self) -> Result<E, EventError> {
        if self.event_type == TypeId::of::<E>() {
            return Ok(*self.event.downcast::<E>().unwrap())
        }
        
        Err(EventError::WrongType(TypeId::of::<E>()))
    }
}

#[derive(Default)]
pub struct EventQueue {
    event_queue: Vec<EventBox>,
}

impl EventQueue {
    pub fn register_event<E: Any>(&mut self, event: E) {
        self.event_queue.push(EventBox::new::<E>(event));
    }

    pub fn dequeue(&mut self) -> Option<EventBox> {
        if self.event_queue.len() > 0 {
            return Some(self.event_queue.remove(0));
        }

        None
    }
}

// todo state as component include other components and change it.
