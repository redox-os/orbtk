use super::{Context, MessageBox};
use crate::prelude::*;

/// Used to define a state of a widget.
///
/// A state is used to operate on the properties (components) of the widget, its parent or children.
pub trait State {
    /// Init is used for initial setup.
    fn init(&self, _context: &mut Context<'_>) {}

    /// Updates the state for the given `context`.
    ///
    /// This update method is called before layout is calculated.
    fn update(&self, _context: &mut Context<'_>) {}

    /// Updates the state for the given `context`.
    ///
    /// This update method is called after layout is calculated and before rendering.
    fn update_post_layout(&self, _context: &mut Context<'_>) {}

    /// Receives all messages from the message channel. This message is only called if the state has messages.
    fn receive_messages(&self, _context: &mut Context<'_>, _messages: &Vec<MessageBox>) {}
}

/// Used to update the pressed state.
pub trait PressedState {
    /// Updates the pressed state.
    fn update_pressed(&self, widget: &mut WidgetContainer) {
        if widget.get::<Pressed>().0 {
            add_selector_to_widget("active", widget);
        } else {
            remove_selector_from_widget("active", widget);
        }
    }
}

/// Used to update the selected state.
pub trait SelectedState {
    /// Updates the selected state.
    fn update_selected(&self, widget: &mut WidgetContainer) {
        if widget.get::<Selected>().0 {
            add_selector_to_widget("selected", widget);
        } else {
            remove_selector_from_widget("selected", widget);
        }
    }
}

/// Used to update the focused state.
pub trait FocusedState {
    /// Updates the focused state.
    fn update_focused(&self, widget: &mut WidgetContainer) {
        if widget.get::<Focused>().0 {
            add_selector_to_widget("focus", widget);
        } else {
            remove_selector_from_widget("focus", widget);
        }
    }
}
