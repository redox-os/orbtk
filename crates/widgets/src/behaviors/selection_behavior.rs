use crate::{api::prelude::*, proc_macros::*};

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
        self.selected = *SelectionBehavior::selected_ref(&ctx.widget());
        let target = *ctx.widget().get::<u32>("target");
        toggle_flag("selected", &mut ctx.get_widget(Entity(target)));
        ctx.get_widget(Entity(target)).update(false);
    }

    fn update(&mut self, _: &mut Registry, ctx: &mut Context) {
        let selected = *SelectionBehavior::selected_ref(&ctx.widget());
        let target: Entity = (*SelectionBehavior::target_ref(&ctx.widget())).into();

        if self.selected == selected && !self.toggle_selection {
            return;
        }

        if *SelectionBehavior::enabled_ref(&ctx.widget()) && self.toggle_selection {
            ctx.get_widget(target).set("selected", !selected);
        }

        self.toggle_selection = false;
        self.selected = *SelectionBehavior::selected_ref(&ctx.widget());

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
