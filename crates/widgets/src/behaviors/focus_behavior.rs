use std::cell::Cell;

use crate::prelude::*;

/// The `FocusBehaviorState` handles the `FocusBehavior` widget.
#[derive(Default, AsAny)]
pub struct FocusBehaviorState {
    request_focus: Cell<bool>,
}

impl FocusBehaviorState {
    fn request_focus(&self) {
        self.request_focus.set(!self.request_focus.get());
    }
}

impl State for FocusBehaviorState {
    fn update(&mut self, _: &mut Registry, ctx: &mut Context<'_>) {
        if !self.request_focus.get() || !ctx.widget().get::<bool>("enabled") {
            return;
        }

        if let Some(old_focused_element) = ctx.window().get::<Global>("global").focused_widget {
            let mut old_focused_element = ctx.get_widget(old_focused_element);
            old_focused_element.set("focused", false);
            old_focused_element.update_theme_by_state(false);
        }

        ctx.widget().set("focused", true);

        let element = ctx.widget().clone::<Selector>("selector").element.unwrap();

        if let Some(parent) = ctx.parent_entity_by_element(&*element) {
            ctx.get_widget(parent).update_theme_by_state(false);
            ctx.window().get_mut::<Global>("global").focused_widget = Some(parent);
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
        focused: bool
    }
);

impl Template for FocusBehavior {
    fn template(self, id: Entity, _: &mut BuildContext) -> Self {
        self.name("FocusBehavior")
            .selector("")
            .focused(true)
            .on_mouse_down(move |states, _| {
                states.get::<FocusBehaviorState>(id).request_focus();
                false
            })
    }
}
