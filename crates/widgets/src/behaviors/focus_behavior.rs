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
        if !self.request_focus.get() || !context.widget().get::<Enabled>("enabled").0 {
            return;
        }

        if let Some(old_focused_element) = context.window().get::<Global>("global").focused_widget {
            let mut old_focused_element = context.get_widget(old_focused_element);
            old_focused_element.set("focused", Focused(false));
            old_focused_element.update_theme_by_state(false);
        }

        context.widget().set("focused", Focused(true));

        let element = context.widget().clone::<Selector>("selector").0.element.unwrap();

        if let Some(parent) = context.parent_entity_by_element(element) {
            context.get_widget(parent).update_theme_by_state(false);
            context.window().get_mut::<Global>("global").focused_widget = Some(parent);
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
