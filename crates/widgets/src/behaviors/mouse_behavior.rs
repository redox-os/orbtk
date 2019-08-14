use std::cell::Cell;

use crate::prelude::*;

#[derive(Debug, Copy, Clone)]
enum Action {
    Press(Point),
    Release(Point),
}

/// The `MouseBehaviorState` handles the `MouseBehavior` widget.
#[derive(Default)]
pub struct MouseBehaviorState {
    action: Cell<Option<Action>>,
}

impl MouseBehaviorState {
    fn action(&self, action: Action) {
        self.action.set(Some(action));
    }
}

impl State for MouseBehaviorState {
    fn update(&self, context: &mut Context<'_>) {
        if !context.widget().get::<Enabled>().0 {
            return;
        }

        if let Some(action) = self.action.get() {
            match action {
                Action::Press(_) => {
                    context.widget().set(Pressed(true));
                }
                Action::Release(p) => {
                    context.widget().set(Pressed(false));

                    if check_mouse_condition(p, &context.widget()) {
                        let parent = context.entity_of_parent().unwrap();
                        context.push_event_by_entity(ClickEvent { position: p }, parent)
                    }
                }
            };

            let element = context.widget().clone::<Selector>().0.element.unwrap();

            if let Some(parent) = context.parent_entity_by_element(element) {
                context.get_widget(parent).update_theme_by_state(false);
            }

            self.action.set(None);
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
        pressed: Pressed
    }
);

impl Template for MouseBehavior {
    fn template(self, _: Entity, _: &mut BuildContext) -> Self {
        let md_state = self.clone_state();
        let mu_state = self.clone_state();

        self.name("MouseBehavior")
            .selector("")
            .pressed(false)
            .on_mouse_down(move |p| {
                md_state.action(Action::Press(p));
                false
            })
            .on_mouse_up(move |p| {
                mu_state.action(Action::Release(p));
                false
            })
    }
}
