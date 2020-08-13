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
    action: Option<Action>,
    has_delta: bool,
    target: Entity,
}

impl MouseBehaviorState {
    fn action(&mut self, action: Action) {
        self.action = Some(action);
    }
}

impl State for MouseBehaviorState {
    fn init(&mut self, _: &mut Registry, ctx: &mut Context) {
        self.target = (*mouse_behavior(ctx.widget()).target()).into();
    }
    fn update(&mut self, _: &mut Registry, ctx: &mut Context) {
        if !mouse_behavior(ctx.widget()).enabled() {
            return;
        }

        if let Some(action) = self.action {
            match action {
                Action::Press(_) => {
                    ctx.get_widget(self.target).set("pressed", true);
                    toggle_flag("pressed", &mut ctx.get_widget(self.target));
                }
                Action::Release(p) => {
                    if !*mouse_behavior(ctx.widget()).pressed() {
                        self.action = None;
                        return;
                    }

                    ctx.get_widget(self.target).set("pressed", false);
                    toggle_flag("pressed", &mut ctx.get_widget(self.target));

                    if check_mouse_condition(p.position, &ctx.widget()) {
                        let parent = ctx.entity_of_parent().unwrap();
                        ctx.push_event_by_entity(
                            ClickEvent {
                                position: p.position,
                            },
                            parent,
                        )
                    }
                }
                Action::Scroll(p) => {
                    mouse_behavior(ctx.widget()).set_position(p);
                    self.has_delta = true;
                }
            };

            ctx.get_widget(self.target).update(false);

            self.action = None;
        }
    }

    fn update_post_layout(&mut self, _: &mut Registry, ctx: &mut Context) {
        if self.has_delta {
            mouse_behavior(ctx.widget()).set_delta(Point::default());
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
            .on_mouse_down(move |states, m| {
                states
                    .get_mut::<MouseBehaviorState>(id)
                    .action(Action::Press(m));
                false
            })
            .on_mouse_up(move |states, m| {
                states
                    .get_mut::<MouseBehaviorState>(id)
                    .action(Action::Release(m));
                false
            })
            .on_scroll(move |states, p| {
                states
                    .get_mut::<MouseBehaviorState>(id)
                    .action(Action::Scroll(p));
                false
            })
    }
}
