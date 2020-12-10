use crate::{api::prelude::*, proc_macros::*};

#[derive(Debug, Copy, Clone)]
enum Action {
    Press(Mouse),
    Release(Mouse),
    Scroll(Point),
}

/// The `MouseBehaviorState` handles the `MouseBehavior` widget.
#[derive(Default, AsAny)]
pub struct MouseBehaviorState {
    has_delta: bool,
    target: Entity,
}

impl State for MouseBehaviorState {
    fn init(&mut self, _: &mut Registry, ctx: &mut Context) {
        self.target = (*MouseBehavior::target_ref(&ctx.widget())).into();
    }

    fn messages(
        &mut self,
        mut messages: MessageReader,
        _registry: &mut Registry,
        ctx: &mut Context,
    ) {
        for message in messages.read::<Action>() {
            match message {
                Action::Press(_) => {
                    ctx.get_widget(self.target).set("pressed", true);
                    toggle_flag("pressed", &mut ctx.get_widget(self.target));
                }
                Action::Release(p) => {
                    if !*MouseBehavior::pressed_ref(&ctx.widget()) {
                        return;
                    }

                    ctx.get_widget(self.target).set("pressed", false);
                    toggle_flag("pressed", &mut ctx.get_widget(self.target));

                    if check_mouse_condition(p.position, &ctx.widget()) {
                        ctx.event_adapter().push_event(
                            self.target,
                            ClickEvent {
                                position: p.position,
                            },
                        );
                    }
                }
                Action::Scroll(p) => {
                    MouseBehavior::position_set(&mut ctx.widget(), p);
                    self.has_delta = true;
                }
            }

            ctx.get_widget(self.target).update(false);
        }
    }

    fn update_post_layout(&mut self, _: &mut Registry, ctx: &mut Context) {
        if self.has_delta {
            MouseBehavior::delta_set(&mut ctx.widget(), Point::default());
            self.has_delta = false;
        }
    }
}

widget!(
    /// The `MouseBehavior` widget is used to handle internal the pressed behavior of a widget.
    ///
    /// **style:** `check-box`
    MouseBehavior<MouseBehaviorState>: MouseHandler {
        /// Sets or shares the target of the behavior.
        target: u32,

        /// Sets or shares the pressed property.
        pressed: bool,

        /// Sets or shares the (wheel, scroll) delta property.
        delta: Point
    }
);

impl Template for MouseBehavior {
    fn template(self, id: Entity, _: &mut BuildContext) -> Self {
        self.name("MouseBehavior")
            .delta(0.0)
            .pressed(false)
            .on_mouse_down(move |ctx, m| {
                ctx.send_message(Action::Press(m), id);
                false
            })
            .on_mouse_up(move |ctx, m| {
                ctx.send_message(Action::Release(m), id);
            })
            .on_scroll(move |ctx, p| {
                ctx.send_message(Action::Scroll(p), id);
                false
            })
    }
}
