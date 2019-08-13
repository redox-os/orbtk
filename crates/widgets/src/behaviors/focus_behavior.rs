use std::cell::Cell;

use crate::prelude::*;

/// The `FocusBehaviorState` handles the `FocusBehavior` widget.
#[derive(Default)]
pub struct FocusBehaviorState {
    request_focus: Cell<bool>,
}

impl FocusBehaviorState {
    fn request_focus(&self) {
        self.request_focus.set(!self.request_focus.get());
    }
}

impl State for FocusBehaviorState {
    fn update(&self, context: &mut Context<'_>) {
        if !self.request_focus.get() || !context.widget().get::<Enabled>().0 {
            return;
        }

        if let Some(old_focused_element) = context.window().get::<Global>().focused_widget {
            let current_widget = context.entity;

            context.entity = old_focused_element;
            context.widget().set(Focused(false));
            remove_selector_from_widget("focus", &mut context.widget());
            context.update_theme_properties(old_focused_element);

            context.entity = current_widget;
        }
        context.widget().set(Focused(true));
        add_selector_to_widget("focus", &mut context.widget());

        let element = context.widget().clone::<Selector>().0.element.unwrap();

        if let Some(parent) = context.parent_entity_by_element(element) {
            context.update_theme_properties(parent);
            context.window().get_mut::<Global>().focused_widget = Some(parent);
        }

        self.request_focus.set(false);
    }
}

widget!(
    /// The `FocusBehavior` widget is used to handle internal the pressed behavior of a widget.
    /// 
    /// **CSS element:** `check-box`
    FocusBehavior<FocusBehaviorState>: MouseHandler {
        /// Sets or shares the css selector property. 
        selector: Selector,

        /// Sets or shares the focused property. 
        focused: Focused
    }
);

impl Template for FocusBehavior {
    fn template(self, _: Entity, _: &mut BuildContext) -> Self {
        let state = self.clone_state();

        self.name("FocusBehavior")
            .selector("")
            .focused(true)
            .on_mouse_down(move |_| {
                state.request_focus();
                false
            })
    }
}
