use super::{Context, MessageBox, Registry};

/// Used to define a state of a widget.
///
/// A state is used to operate on the properties (components) of the widget, its parent or children.
pub trait State {
    /// Init is used for initial setup.
    fn init(&self, _: &mut Registry, _: &mut Context<'_>) {}

    /// Updates the state for the given `ctx`.
    ///
    /// This update method is called before layout is calculated.
    fn update(&self, _: &mut Registry, _: &mut Context<'_>) {}

    /// Updates the state for the given `ctx`.
    ///
    /// This update method is called after layout is calculated and before rendering.
    fn update_post_layout(&self, _: &mut Registry, _: &mut Context<'_>) {}

    /// Receives all messages from the message channel. This message is only called if the state has messages.
    fn receive_messages(&self, _: &mut Registry, _: &mut Context<'_>, _messages: &[MessageBox]) {}
}
