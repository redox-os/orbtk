use crate::prelude::*;

/// The `SelectionBehaviorState` handles the `SelectionBehavior` widget.
#[derive(Default, AsAny)]
pub struct SelectionBehaviorState {
    toggle_selection: bool,
    selected: bool,
}

impl SelectionBehaviorState {
    fn toggle_selection(&mut self) {
        self.toggle_selection = true;
    }
}

impl State for SelectionBehaviorState {
    fn init(&mut self, _: &mut Registry, ctx: &mut Context) {
        self.selected = *selection_behavior(ctx.widget()).selected();
        let target = *ctx.widget().get::<u32>("target");
        toggle_flag("selected", &mut ctx.get_widget(Entity(target)));
        ctx.get_widget(Entity(target)).update(false);
    }

    fn update(&mut self, _: &mut Registry, ctx: &mut Context) {
        let selected = *selection_behavior(ctx.widget()).selected();

        if self.selected == selected && !self.toggle_selection {
            return;
        }

        if *selection_behavior(ctx.widget()).enabled() && self.toggle_selection {
            selection_behavior(ctx.widget()).set_selected(!selected);
        }

        self.toggle_selection = false;
        self.selected = *selection_behavior(ctx.widget()).selected();

        let target: Entity = (*selection_behavior(ctx.widget()).target()).into();

        toggle_flag("selected", &mut ctx.get_widget(target));

        ctx.get_widget(target).update(false);
    }
}

widget!(
    /// The `SelectionBehavior` widget is used to handle internal the pressed behavior of a widget.
    ///
    /// **style:** `check-box`
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
