use super::Context;

/// Used to define a state of a widget. A state is used to customize properties of a widget.
pub trait State {
    /// Updates the state for the given `context`.
    fn update(&self, context: &mut Context<'_>);

    /// This update method is called after layout is calculated and before rendering.
    fn update_post_layout(&self, _context: &mut Context<'_>) {}
}
