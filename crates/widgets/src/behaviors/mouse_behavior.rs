use std::cell::Cell;

use crate::prelude::*;

#[derive(Debug, Copy, Clone)]
enum Action {
    Press(Point),
    Release(Point),
    Scroll(Point),
}

/// The `MouseBehaviorState` handles the `MouseBehavior` widget.
#[derive(Default)]
pub struct MouseBehaviorState {
    action: Cell<Option<Action>>,
    has_delta: Cell<bool>,
}

impl MouseBehaviorState {
    fn action(&self, action: Action) {
        self.action.set(Some(action));
    }
}

impl State for MouseBehaviorState {
    fn update(&self, context: &mut Context<'_>) {
        if !context.widget().get::<Enabled>("enabled").0 {
            return;
        }

        if let Some(action) = self.action.get() {
            match action {
                Action::Press(_) => {
                    context.widget().set("pressed", Pressed(true));
                }
                Action::Release(p) => {
                    context.widget().set("pressed", Pressed(false));

                    if check_mouse_condition(p, &context.widget()) {
                        let parent = context.entity_of_parent().unwrap();
                        context.push_event_by_entity(ClickEvent { position: p }, parent)
                    }
                }
                Action::Scroll(p) => {
                    context.widget().set("position", Pos::from(p));
                    self.has_delta.set(true);
                }
            };

            let element = context.widget().clone::<Selector>("selector").0.element.unwrap();

            if let Some(parent) = context.parent_entity_by_element(element) {
                context.get_widget(parent).update_theme_by_state(false);
            }

            self.action.set(None);
        }
    }

    fn update_post_layout(&self, context: &mut Context<'_>) {
        if self.has_delta.get() {
            context.widget().set("delta", Delta(Point::new(0.0, 0.0)));
            self.has_delta.set(false);
        }
    }
}

widget!(
    /// The `MouseBehavior` widget is used to handle internal the pressed behavior of a widget.
    /// 
    /// **CSS element:** `check-box`
    MouseBehavior<MouseBehaviorState>: MouseHandler {
        /// Sets or shares the css selector property. 
        selector: Selector,

        /// Sets or shares the pressed property. 
        pressed: Pressed,

        /// Sets or shares the (wheel, scroll) delta property. 
        delta: Delta
    }
);

impl Template for MouseBehavior {
    fn template(self, _: Entity, _: &mut BuildContext) -> Self {
        let md_state = self.clone_state();
        let mu_state = self.clone_state();
        let wh_state = self.clone_state();

        self.name("MouseBehavior")
            .selector("")
            .delta(0.0)
            .pressed(false)
            .on_mouse_down(move |p| {
                md_state.action(Action::Press(p));
                false
            })
            .on_mouse_up(move |p| {
                mu_state.action(Action::Release(p));
                false
            })
            .on_scroll(move |p| {
                wh_state.action(Action::Scroll(p));
                false
            })
    }
}
