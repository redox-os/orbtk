use crate::{api::prelude::*, proc_macros::*};

/// The `FocusBehaviorState` handles the `FocusBehavior` widget.
#[derive(Default, AsAny)]
pub struct FocusBehaviorState {
    request_focus: bool,
}

impl FocusBehaviorState {
    fn request_focus(&mut self) {
        self.request_focus = true;
    }
}

impl State for FocusBehaviorState {
    fn update(&mut self, _: &mut Registry, ctx: &mut Context) {
        if !self.request_focus || !focus_behavior(ctx.widget()).enabled() {
            return;
        }

        let target: Entity = (*focus_behavior(ctx.widget()).target()).into();

        ctx.push_event_by_window(FocusEvent::RequestFocus(target));

        self.request_focus = false;
    }
}

widget!(
    /// The `FocusBehavior` widget is used to handle internal the pressed behavior of a widget.
    ///
    /// **style:** `check-box`
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
                states.get_mut::<FocusBehaviorState>(id).request_focus();
                false
            })
    }
}
