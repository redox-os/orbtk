use std::any::{Any, TypeId};

#[derive(Debug)]
pub enum MessageError {
    WrongType(TypeId),
}

/// A message box is a container for wrapping a message.
#[derive(Debug)]
pub struct MessageBox {
    message: Box<dyn Any>,
    message_type: TypeId,
}

impl MessageBox {
    pub fn new<M: Any>(message: M) -> Self {
        MessageBox {
            message: Box::new(message),
            message_type: TypeId::of::<M>(),
        }
    }

    pub fn is_type<M: Any>(&self) -> bool {
        self.message_type == TypeId::of::<M>()
    }

    pub fn message_type(&self) -> TypeId {
        self.message_type
    }

    pub fn downcast<M: Any>(self) -> Result<M, MessageError> {
        if self.message_type == TypeId::of::<M>() {
            return Ok(*self.message.downcast::<M>().unwrap());
        }

        Err(MessageError::WrongType(TypeId::of::<M>()))
    }

    pub fn downcast_ref<M: Any>(&self) -> Result<&M, MessageError> {
        if self.message_type == TypeId::of::<M>() {
            return Ok(&*self.message.downcast_ref::<M>().unwrap());
        }

        Err(MessageError::WrongType(TypeId::of::<M>()))
    }
}

/// Used to sent a simple string message over the message channel.
#[derive(Default, Clone)]
pub struct StringMessage(pub String);

impl From<&str> for StringMessage {
    fn from(s: &str) -> Self {
        StringMessage(s.to_string())
    }
}

impl From<String> for StringMessage {
    fn from(s: String) -> Self {
        StringMessage(s)
    }
}

impl Into<MessageBox> for StringMessage {
    fn into(self) -> MessageBox {
        MessageBox::new(self)
    }
}
