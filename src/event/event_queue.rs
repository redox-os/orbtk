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

    pub fn event_type(&self) -> TypeId {
        self.event_type
    }

    pub fn downcast<E: Any>(self) -> Result<E, EventError> {
        if self.event_type == TypeId::of::<E>() {
            return Ok(*self.event.downcast::<E>().unwrap())
        }
        
        Err(EventError::WrongType(TypeId::of::<E>()))
    }

    pub fn downcast_ref<E: Any>(&self) -> Result<&E, EventError> {
        if self.event_type == TypeId::of::<E>() {
            return Ok(&*self.event.downcast_ref::<E>().unwrap())
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

impl<'a> IntoIterator for &'a mut EventQueue {
    type Item = EventBox;
    type IntoIter = EventQeueIterator<'a>;

    fn into_iter(self) -> Self::IntoIter {
        EventQeueIterator {
            event_queue: self,
        }
    }
}

pub struct EventQeueIterator<'a> {
    event_queue: &'a mut EventQueue,
}

impl<'a> Iterator for EventQeueIterator<'a> {
    type Item = EventBox;

    fn next(&mut self) -> Option<EventBox> {
        self.event_queue.dequeue()
    }
}

// todo state as component include other components and change it.
