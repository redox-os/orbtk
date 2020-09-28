use std::any::{Any, TypeId};

use dces::prelude::Entity;

use super::{Event, EventStrategy};

#[derive(Debug)]
pub enum EventError {
    WrongType(TypeId),
}

/// Internal wrapper for an event, including the strategy and source entity.
#[derive(Debug)]
pub struct EventBox {
    event: Box<dyn Any + Send>,
    event_type: TypeId,
    pub source: Entity,
    pub strategy: EventStrategy,
}

impl EventBox {
    /// Creates a new `EventBox`.
    pub fn new<E: Event + Send>(event: E, strategy: EventStrategy, source: Entity) -> Self {
        EventBox {
            event: Box::new(event),
            source,
            event_type: TypeId::of::<E>(),
            strategy,
        }
    }

    /// Check if the given type is the type of the event.
    pub fn is_type<E: Event>(&self) -> bool {
        self.event_type == TypeId::of::<E>()
    }

    /// Returns the type of the event.
    pub fn event_type(&self) -> TypeId {
        self.event_type
    }

    /// Downcasts the box to an concrete event.
    pub fn downcast<E: Event>(self) -> Result<E, EventError> {
        if self.event_type == TypeId::of::<E>() {
            return Ok(*self.event.downcast::<E>().unwrap());
        }

        Err(EventError::WrongType(TypeId::of::<E>()))
    }

    /// Downcasts the box as reference of an concrete event.
    pub fn downcast_ref<E: Any>(&self) -> Result<&E, EventError> {
        if self.event_type == TypeId::of::<E>() {
            return Ok(&*self.event.downcast_ref::<E>().unwrap());
        }

        Err(EventError::WrongType(TypeId::of::<E>()))
    }
}

/// The  `EventQueue` is used to register and read new events.
#[derive(Default, Debug)]
pub struct EventQueue {
    event_queue: Vec<EventBox>,
}

impl EventQueue {
    /// Creates a new event queue.
    pub fn new() -> Self {
        Self::default()
    }

    /// Appends a new event box.
    pub fn append(&mut self, other: &mut Vec<EventBox>) {
        self.event_queue.append(other);
    }

    /// Registers an event with a given event strategy and a source (Entity of a widget) where the event should start.
    pub fn register_event_with_strategy<E: Event + Send>(
        &mut self,
        event: E,
        strategy: EventStrategy,
        source: Entity,
    ) {
        self.event_queue
            .push(EventBox::new::<E>(event, strategy, source));
    }

    // todo rename to enqueue event
    pub fn register_event<E: Event + Send>(&mut self, event: E, source: Entity) {
        self.event_queue
            .push(EventBox::new::<E>(event, EventStrategy::BottomUp, source));
    }

    /// Dequeue an event.
    pub fn dequeue(&mut self) -> Option<EventBox> {
        if !self.event_queue.is_empty() {
            return Some(self.event_queue.remove(0));
        }

        None
    }

    /// Returns the number of events in the `EventQueue`.
    pub fn len(&self) -> usize {
        self.event_queue.len()
    }

    /// If the `EventQueue` has more then zero events it will return `true` otherwise `false`.
    pub fn is_empty(&self) -> bool {
        self.event_queue.is_empty()
    }
}

impl<'a> IntoIterator for &'a mut EventQueue {
    type Item = EventBox;
    type IntoIter = EventQueueIterator<'a>;

    fn into_iter(self) -> Self::IntoIter {
        EventQueueIterator { event_queue: self }
    }
}

pub struct EventQueueIterator<'a> {
    event_queue: &'a mut EventQueue,
}

impl<'a> EventQueueIterator<'a> {
    pub fn new(event_queue: &'a mut EventQueue) -> Self {
        EventQueueIterator { event_queue }
    }
}

impl<'a> Iterator for EventQueueIterator<'a> {
    type Item = EventBox;

    fn next(&mut self) -> Option<EventBox> {
        self.event_queue.dequeue()
    }
}
