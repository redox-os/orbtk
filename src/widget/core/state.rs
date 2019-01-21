use super::Context;

use super::MessageBox;

/// Used to define a state of a widget. A state is used to customize properties of a widget.
pub trait State {
    /// Updates the state for the given `context`.
    ///
    /// This update method is called before layout is calculated.
    fn update(&self, _context: &mut Context<'_>) {}

    /// Updates the state for the given `context`.
    ///
    /// This update method is called after layout is calculated and before rendering.
    fn update_post_layout(&self, _context: &mut Context<'_>) {}

    /// Recives all messages from the message channel. This message is only called if the state has messages.
    fn receive_messages(&self, _context: &mut Context<'_>, _messages: &Vec<MessageBox>) {}
}
