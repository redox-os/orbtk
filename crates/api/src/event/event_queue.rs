use std::any::{Any, TypeId};

use dces::prelude::Entity;

use super::{Event, EventStrategy};

#[derive(Debug)]
pub enum EventError {
    WrongType(TypeId),
}

#[derive(Debug)]
pub struct EventBox {
    event: Box<dyn Any>,
    event_type: TypeId,
    pub source: Entity,
    pub strategy: EventStrategy,
}

impl EventBox {
    pub fn new<E: Event>(event: E, strategy: EventStrategy, source: Entity) -> Self {
        EventBox {
            event: Box::new(event),
            source: source,
            event_type: TypeId::of::<E>(),
            strategy,
        }
    }

    pub fn is_type<E: Event>(&self) -> bool {
        self.event_type == TypeId::of::<E>()
    }

    pub fn event_type(&self) -> TypeId {
        self.event_type
    }

    pub fn downcast<E: Event>(self) -> Result<E, EventError> {
        if self.event_type == TypeId::of::<E>() {
            return Ok(*self.event.downcast::<E>().unwrap());
        }

        Err(EventError::WrongType(TypeId::of::<E>()))
    }

    pub fn downcast_ref<E: Any>(&self) -> Result<&E, EventError> {
        if self.event_type == TypeId::of::<E>() {
            return Ok(&*self.event.downcast_ref::<E>().unwrap());
        }

        Err(EventError::WrongType(TypeId::of::<E>()))
    }
}

#[derive(Default, Debug)]
pub struct EventQueue {
    event_queue: Vec<EventBox>,
}

impl EventQueue {
    pub fn append(&mut self, other: &mut Vec<EventBox>) {
        self.event_queue.append(other);
    }

    pub fn register_event_with_strategy<E: Event>(
        &mut self,
        event: E,
        strategy: EventStrategy,
        source: Entity,
    ) {
        self.event_queue
            .push(EventBox::new::<E>(event, strategy, source));
    }

    // todo rename to enqueue event
    pub fn register_event<E: Event>(&mut self, event: E, source: Entity) {
        self.event_queue
            .push(EventBox::new::<E>(event, EventStrategy::BottomUp, source));
    }

    pub fn dequeue(&mut self) -> Option<EventBox> {
        if !self.event_queue.is_empty() {
            return Some(self.event_queue.remove(0));
        }

        None
    }

    pub fn len(&self) -> usize {
        self.event_queue.len()
    }
}

impl<'a> IntoIterator for &'a mut EventQueue {
    type Item = EventBox;
    type IntoIter = EventQeueIterator<'a>;

    fn into_iter(self) -> Self::IntoIter {
        EventQeueIterator { event_queue: self }
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
