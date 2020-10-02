use std::{
    any::{Any, TypeId},
    collections::BTreeMap,
    marker::PhantomData,
    sync::{Arc, RwLock},
};

use dces::entity::Entity;

#[derive(Debug)]
pub struct MessageBox {
    message: Box<dyn Any + Send>,
    message_type: TypeId,
    target: Entity,
}

impl MessageBox {
    /// Creates a new `MessageBox`.
    pub fn new<M: Any + Send>(message: M, target: Entity) -> Self {
        MessageBox {
            message: Box::new(message),
            target,
            message_type: TypeId::of::<M>(),
        }
    }

    /// Check if the given type is the type of the message.
    pub fn is_type<M: Any>(&self) -> bool {
        self.message_type == TypeId::of::<M>()
    }

    /// Returns the type of the event.
    pub fn message_type(&self) -> TypeId {
        self.message_type
    }

    /// Downcasts the box to an concrete message.
    pub fn downcast<M: Any>(self) -> Result<M, String> {
        if self.message_type == TypeId::of::<M>() {
            return Ok(*self.message.downcast::<M>().unwrap());
        }

        Err("Wrong message type".to_string())
    }

    /// Downcasts the box as reference of an concrete message.
    pub fn downcast_ref<M: Any>(&self) -> Result<&M, String> {
        if self.message_type == TypeId::of::<M>() {
            return Ok(&*self.message.downcast_ref::<M>().unwrap());
        }

        Err("Wrong message type".to_string())
    }
}

#[derive(Clone, Default, Debug)]
pub struct MessageAdapter {
    messages: Arc<RwLock<BTreeMap<Entity, Vec<MessageBox>>>>,
}

impl MessageAdapter {
    pub fn new() -> Self {
        MessageAdapter::default()
    }

    pub fn push_message<M: Any + Send>(&self, target: Entity, message: M) {
        if !self.messages.read().unwrap().contains_key(&target) {
            self.messages.write().unwrap().insert(target, vec![]);
        }

        self.messages
            .write()
            .unwrap()
            .get_mut(&target)
            .unwrap()
            .push(MessageBox::new(message, target));
    }

    pub(crate) fn entities(&self) -> Vec<Entity> {
        self.messages.read().unwrap().keys().cloned().collect()
    }

    /// Removes all messages for the given target entity. This is used to remove messages for
    /// entities that does not have a `State` to read the messages.
    pub(crate) fn remove_message_for_entity(&self, target: Entity) {
        self.messages.write().unwrap().remove(&target);
    }

    /// Returns the number of messages in the queue.
    pub fn len(&self) -> usize {
        self.messages.read().unwrap().len()
    }

    /// Returns `true` if the event message contains no events.
    pub fn is_empty(&self) -> bool {
        self.messages.read().unwrap().is_empty()
    }

    pub fn read<M: Any + Send>(&self, target: Entity) -> MessageReader<M> {
        if let Some(messages) = self.messages.write().unwrap().remove(&target) {
            return MessageReader::new(messages, target);
        }

        MessageReader::new(vec![], target)
    }
}

// todo split in MessageReader with all messages of an entity and generate a messageiteratior

#[derive(Debug)]
pub struct MessageReader<M>
where
    M: Any + Send,
{
    messages: Vec<MessageBox>,
    target: Entity,
    _phantom: PhantomData<M>,
}

impl<M> MessageReader<M>
where
    M: Any + Send,
{
    pub(crate) fn new(messages: Vec<MessageBox>, target: Entity) -> Self {
        MessageReader {
            messages,
            target,
            _phantom: PhantomData::default(),
        }
    }
}

impl<M> Iterator for MessageReader<M>
where
    M: Any + Send,
{
    type Item = M;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(index) = self
            .messages
            .iter()
            .position(|m| m.target == self.target && m.type_id() == TypeId::of::<M>())
        {
            return Some(self.messages.remove(index).downcast::<M>().unwrap());
        }

        None
    }
}
