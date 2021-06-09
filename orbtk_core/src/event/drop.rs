use crate::{
    prelude::*,
    proc_macros::{Event, IntoHandler},
};

/// This event occurs on a drop file event on the window.
#[derive(Event, Clone)]
pub struct DropFileEvent {
    pub file_name: String,
    pub position: Point,
}

/// This event occurs on a drop file event on the window.
#[derive(Event, Clone)]
pub struct DropTextEvent {
    pub text: String,
    pub position: Point,
}

pub type DropFn = dyn Fn(&mut StatesContext, String, Point) -> bool + 'static;

#[derive(IntoHandler)]
pub struct DropFileHandler {
    pub handler: Rc<DropFn>,
}

impl EventHandler for DropFileHandler {
    fn handle_event(&self, states: &mut StatesContext, event: &EventBox) -> bool {
        if let Ok(event) = event.downcast_ref::<DropFileEvent>() {
            return (self.handler)(states, event.file_name.clone(), event.position);
        }

        false
    }

    fn handles_event(&self, event: &EventBox) -> bool {
        event.is_type::<DropFileEvent>()
    }
}

#[derive(IntoHandler)]
pub struct DropTextHandler {
    pub handler: Rc<DropFn>,
}

impl EventHandler for DropTextHandler {
    fn handle_event(&self, states: &mut StatesContext, event: &EventBox) -> bool {
        if let Ok(event) = event.downcast_ref::<DropTextEvent>() {
            return (self.handler)(states, event.text.clone(), event.position);
        }

        false
    }

    fn handles_event(&self, event: &EventBox) -> bool {
        event.is_type::<DropTextEvent>()
    }
}

/// Implement this trait if you want that your widget can handle drop (file | text) events
pub trait DropHandler: Sized + Widget {
    /// Inserts a handler for drop file events.
    fn on_drop_file<H: Fn(&mut StatesContext, String, Point) -> bool + 'static>(
        self,
        handler: H,
    ) -> Self {
        self.insert_handler(DropFileHandler {
            handler: Rc::new(handler),
        })
    }

    /// Inserts a handler for drop text events.
    fn on_drop_text<H: Fn(&mut StatesContext, String, Point) -> bool + 'static>(
        self,
        handler: H,
    ) -> Self {
        self.insert_handler(DropTextHandler {
            handler: Rc::new(handler),
        })
    }
}
