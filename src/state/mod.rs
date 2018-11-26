//! This module contains all widget state related resources.

use widget::WidgetContainer;

/// Used to define a state of a widget. A state is used to customize properties of a widget.
pub trait State {
    /// Updates the state for the given `widget`.
    fn update(&self, widget: &mut WidgetContainer);
}
