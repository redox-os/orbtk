use crate::prelude::*;

/// The `SelectionBehaviorState` handles the `SelectionBehavior` widget.
#[derive(Default, AsAny)]
pub struct SelectionBehaviorState {
    toggle_selection: bool,
}

impl SelectionBehaviorState {
    fn toggle_selection(&mut self) {
        self.toggle_selection = true;
    }
}

impl State for SelectionBehaviorState {
    fn update(&mut self, _: &mut Registry, ctx: &mut Context<'_>) {
        if !ctx.widget().get::<bool>("enabled") || !self.toggle_selection {
            return;
        }

        self.toggle_selection = false;

        let selected = *ctx.widget().get::<bool>("selected");
        ctx.widget().set("selected", !selected);

        let target: Entity = (*ctx.widget().get::<u32>("target")).into();
        ctx.push_event_strategy_by_entity(ChangedEvent(target), target, EventStrategy::Direct);
        ctx.get_widget(target).update_theme_by_state(false);
    }
}

widget!(
    /// The `SelectionBehavior` widget is used to handle internal the pressed behavior of a widget.
    ///
    /// **CSS element:** `check-box`
    SelectionBehavior<SelectionBehaviorState>: MouseHandler {
        /// Sets or shares the target of the behavior.
        target: u32,

        /// Sets or shares the selected property.
        selected: bool,

        /// Sets the parent id.
        parent: u32
    }
);

impl Template for SelectionBehavior {
    fn template(self, id: Entity, _: &mut BuildContext) -> Self {
        self.name("SelectionBehavior")
            .selected(true)
            .on_click(move |states, _| {
                states
                    .get_mut::<SelectionBehaviorState>(id)
                    .toggle_selection();
                false
            })
    }
}
