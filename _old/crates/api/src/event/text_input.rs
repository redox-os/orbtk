use std::rc::Rc;

use crate::{prelude::*, proc_macros::*};

/// The text input occurs if the keyboard registers a text input.
#[derive(Clone, Default, Debug, Event)]
pub struct TextInputEvent {
    pub text: String,
}

/// Callback closure to handle text input events.
pub type TextHandler = dyn Fn(&mut StatesContext, &str) -> bool + 'static;

/// Internal struct to manage text input event handlers.
#[derive(IntoHandler)]
pub struct TextInputEventHandler {
    handler: Rc<TextHandler>,
}

impl EventHandler for TextInputEventHandler {
    fn handle_event(&self, state_context: &mut StatesContext, event: &EventBox) -> bool {
        event
            .downcast_ref::<TextInputEvent>()
            .ok()
            .map_or(false, |event| {
                (self.handler)(state_context, event.text.as_str())
            })
    }

    fn handles_event(&self, event: &EventBox) -> bool {
        event.is_type::<TextInputEvent>()
    }
}

/// Implement this trait for widgets that should handle text input events.
///
/// # Examples
///
/// ```rust
/// widget!(MyWidget: TextInputHandler {});
///
/// MyWidget::new()
///     .on_text_input(|_ctx, text| {
///         println!("{}", text);
///         true
///     }).build(ctx)
/// ```
pub trait TextInputHandler: Sized + Widget {
    /// Callback that is called when a text input event reaches the widget.
    ///
    /// If the callback returns `true` the event is marked as handled and will not available to
    /// to other widgets.
    fn on_text_input<H: Fn(&mut StatesContext, &str) -> bool + 'static>(self, handler: H) -> Self {
        self.insert_handler(TextInputEventHandler {
            handler: Rc::new(handler),
        })
    }
}
