use crate::prelude::*;

/// The `SelectionBehaviorState` handles the `SelectionBehavior` widget.
#[derive(Default, AsAny)]
pub struct SelectionBehaviorState {
    selected: bool,
}

impl SelectionBehaviorState {
    fn toggle_selection(&mut self) {
        self.selected = !self.selected;
    }
}

impl State for SelectionBehaviorState {
    fn init(&mut self, _: &mut Registry, ctx: &mut Context<'_>) {
        self.selected = *ctx.widget().get("selected");
    }
    
    fn update(&mut self, _: &mut Registry, ctx: &mut Context<'_>) {
        if !ctx.widget().get::<bool>("enabled")
            || *ctx.widget().get::<bool>("selected") == self.selected
        {
            return;
        }

        ctx.widget().set("selected", self.selected);

        let parent: Entity = ctx.widget().clone::<u32>("parent").into();
        ctx.push_event_strategy_by_entity(ChangedEvent(parent), parent, EventStrategy::Direct);

        let element = ctx.widget().clone::<Selector>("selector").element.unwrap();

        if let Some(parent) = ctx.parent_entity_by_element(&*element) {
            ctx.get_widget(parent).update_theme_by_state(false);
        }
    }
}

widget!(
    /// The `SelectionBehavior` widget is used to handle internal the pressed behavior of a widget.
    /// 
    /// **CSS element:** `check-box`
    SelectionBehavior<SelectionBehaviorState>: MouseHandler {
        /// Sets or shares the css selector property. 
        selector: Selector,

        /// Sets or shares the selected property. 
        selected: bool,

        /// Sets the parent id.
        parent: u32
    }
);

impl Template for SelectionBehavior {
    fn template(self, id: Entity, _: &mut BuildContext) -> Self {
        self.name("SelectionBehavior")
            .parent(id.0)
            .selector("")
            .selected(true)
            .on_click(move |states, _| {
                states
                    .get_mut::<SelectionBehaviorState>(id)
                    .toggle_selection();
                false
            })
    }
}
