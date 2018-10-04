use std::any::{Any, TypeId};
use std::sync::Arc;

#[derive(Debug)]
pub enum EventError {
    WrongType(TypeId)
}

pub struct EventBox {
    pub event: Arc<dyn Any + Send + Sync>,
    pub event_type: TypeId,
}

impl EventBox {
    pub fn new<E: Any + Send + Sync>(event: E) -> Self {
        EventBox {
            event: Arc::new(event),
            event_type: TypeId::of::<E>(),
        }
    }

    pub fn is_type<E: Any + Send + Sync>(&self) -> bool {
        self.event_type == TypeId::of::<E>()
    }

    pub fn downcast<E: Any + Send + Sync>(self) -> Result<Arc<E>, EventError> {
        if self.event_type == TypeId::of::<E>() {
            if let Ok(event) = self.event.downcast::<E>() {
                return Ok(event)
            }   
        }
        
        Err(EventError::WrongType(TypeId::of::<E>()))
    }
}

#[derive(Default)]
pub struct EventManager {
    event_queue: Vec<EventBox>,
}

impl EventManager {
    pub fn register_event<E: Any + Send + Sync>(&mut self, event: E) {
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
