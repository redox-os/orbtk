use std::{fmt::Debug, rc::Rc};

use dces::prelude::{Component, Entity};

use crate::{event::EventHandler, properties::AttachedProperty, theming::Selector};

pub use self::build_context::*;
pub use self::context::*;
pub use self::message_adapter::*;
pub use self::registry::*;
pub use self::state::*;
pub use self::states_context::*;
pub use self::template::*;
pub use self::widget_container::*;

mod build_context;
mod context;
mod message_adapter;
mod registry;
mod state;
mod states_context;
mod template;
mod widget_container;

/// Toggles the selector state`.
pub fn toggle_flag(flag: &str, widget: &mut WidgetContainer) {
    if !widget.has::<bool>(flag) {
        return;
    }

    let value = *widget.get::<bool>(flag);

    if let Some(selector) = widget.try_get_mut::<Selector>("selector") {
        if value {
            selector.push_state(flag);
        } else {
            selector.remove_state(flag);
        }
    }
}

/// Sets the given property flag to  `true` and set the flags name as state of the widgets selector.
pub fn set_flag(flag: &str, widget: &mut WidgetContainer) {
    if !widget.has::<bool>(flag) {
        return;
    }

    widget.set(flag, true);

    if let Some(selector) = widget.try_get_mut::<Selector>("selector") {
        selector.push_state(flag);
    }

    widget.update(false);
}

/// Sets the given property flag to  `false` and clears the state of the widgets selector.
pub fn remove_flag(flag: &str, widget: &mut WidgetContainer) {
    if !widget.has::<bool>(flag) {
        return;
    }

    widget.set(flag, false);

    if let Some(selector) = widget.try_get_mut::<Selector>("selector") {
        selector.remove_state(flag);
    }

    widget.update(false);
}

/// Used to define the `parent_type`of a widget.
pub enum ParentType {
    /// No children could be added to the widget.
    None,

    /// Only one child could be added to the widget.
    Single,

    /// Multiple children could be added to the widget.
    Multi,
}

/// The `Widget` trait is used to define a new widget.
pub trait Widget: Template {
    /// Creates a new widget.
    fn new() -> Self;

    /// Creates a new widget.
    #[inline(always)]
    #[deprecated = "Use new instead"]
    fn create() -> Self {
        Self::new()
    }

    // This method will always be overwritten by the `widget!` macros.
    fn attach<P: Component + Debug>(self, _: AttachedProperty<P>) -> Self {
        self
    }

    /// Builds the widget and returns the template of the widget.
    fn build(self, ctx: &mut BuildContext) -> Entity;

    /// Inerts a new event handler.
    fn insert_handler(self, handler: impl Into<Rc<dyn EventHandler>>) -> Self;

    // Inserts a new changed handler.
    fn insert_changed_handler<H: Fn(&mut StatesContext, Entity) + 'static>(
        self,
        key: impl Into<String>,
        handler: Rc<H>,
    ) -> Self;

    /// Appends a child to the widget.
    fn child(self, child: Entity) -> Self;
}
