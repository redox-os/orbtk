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
        if !self.request_focus.get() || !focus_behavior(ctx.widget()).enabled() {
            return;
        }

        let target: Entity = (*focus_behavior(ctx.widget()).target()).into();

        ctx.push_event_by_window(FocusEvent::RequestFocus(target));

        self.request_focus.set(false);
    }
}

widget!(
    /// The `FocusBehavior` widget is used to handle internal the pressed behavior of a widget.
    ///
    /// **CSS element:** `check-box`
    FocusBehavior<FocusBehaviorState>: MouseHandler {
        /// Sets or shares the target of the behavior.
        target: u32,

        /// Sets or shares the focused property.
        focused: bool
    }
);

impl Template for FocusBehavior {
    fn template(self, id: Entity, _: &mut BuildContext) -> Self {
        self.name("FocusBehavior")
            .focused(true)
            .on_mouse_down(move |states, _| {
                states.get::<FocusBehaviorState>(id).request_focus();
                false
            })
    }
}
