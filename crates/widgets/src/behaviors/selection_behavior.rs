use crate::{api::prelude::*, proc_macros::*};

/// Used for selection.
pub enum SelectionAction {
    ToggleSelection,
}

/// The `SelectionBehaviorState` handles the `SelectionBehavior` widget.
#[derive(Default, AsAny)]
pub struct SelectionBehaviorState {
    target: Entity,
}

impl State for SelectionBehaviorState {
    fn init(&mut self, _: &mut Registry, ctx: &mut Context) {
        self.target = (*ctx.widget().get::<u32>("target")).into();
        toggle_flag("selected", &mut ctx.get_widget(self.target));
        ctx.get_widget(self.target).update(false);
    }

    fn messages(
        &mut self,
        mut messages: MessageReader,
        _registry: &mut Registry,
        ctx: &mut Context,
    ) {
        for message in messages.read::<SelectionAction>() {
            match message {
                SelectionAction::ToggleSelection => {
                    let selected = *ctx.get_widget(self.target).get::<bool>("selected");
                    ctx.get_widget(self.target).set("selected", !selected);
                    toggle_flag("selected", &mut ctx.get_widget(self.target));
                    ctx.get_widget(self.target).update(false);
                }
            };
        }
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
            .on_click(move |ctx, _| {
                ctx.send_message(SelectionAction::ToggleSelection, id);
                false
            })
    }
}
