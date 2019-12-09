use super::behaviors::MouseBehavior;
use crate::prelude::*;

/// State to handle the position of switch toggle.
#[derive(Default, AsAny)]
pub struct SwitchState {
    selected: bool,
}

impl SwitchState {
    fn toggle_selection(&mut self) {
        self.selected = !self.selected;
    }
}

impl State for SwitchState {
    fn update(&mut self, _: &mut Registry, ctx: &mut Context<'_>) {
        if *ctx.widget().get::<bool>("selected") == self.selected {
            return;
        }

        ctx.widget().set("selected", self.selected);

        let element = ctx.widget().clone::<Selector>("selector").element.unwrap();

        if let Some(parent) = ctx.parent_entity_by_element(&*element) {
            ctx.get_widget(parent).update_theme_by_state(false);
        }

        {
            let mut switch_toggle = ctx.child("switch_toggle");

            if self.selected {
                switch_toggle.set("horizontal_alignment", Alignment::from("end"));
                add_selector_to_widget("selected", &mut switch_toggle);
            } else {
                switch_toggle.set("horizontal_alignment", Alignment::from("start"));
                remove_selector_from_widget("selected", &mut switch_toggle);
            }

            switch_toggle.update_theme_by_state(true);
        }

        ctx.push_event_strategy_by_entity(
            ChangedEvent(ctx.entity),
            ctx.entity,
            EventStrategy::Direct,
        );
        let entity = ctx.entity_of_child("switch_toggle").unwrap();

        ctx.get_widget(entity).update_theme_by_state(false);
    }
}

widget!(
    /// The `Switch` widget can be switch between `on` and `off`.
    ///
    /// **CSS element:** `switch`
    Switch<SwitchState>: MouseHandler {
        /// Sets or shares the background property.
        background: Brush,

        /// Sets or shares the border radius property.
        border_radius: f64,

        /// Sets or shares the border thickness property.
        border_width: Thickness,

        /// Sets or shares the border brush property.
        border_brush: Brush,

        /// Sets or shares the padding property.
        padding: Thickness,

        /// Sets or shares the css selector property.
        selector: Selector,

        /// Sets or shares the pressed property.
        pressed: bool,

        /// Sets or shares the selected property.
        selected: bool
    }
);

impl Template for Switch {
    fn template(self, id: Entity, ctx: &mut BuildContext) -> Self {
        self.name("Switch")
            .selector("switch")
            .pressed(false)
            .selected(false)
            .width(48.0)
            .height(32.0)
            .border_brush(colors::BOMBAY_COLOR)
            .background(colors::SLATE_GRAY_COLOR)
            .border_radius(8.0)
            .border_width(1.0)
            .padding(4.0)
            .child(
                MouseBehavior::create()
                    .pressed(id)
                    .enabled(id)
                    .selector(id)
                    .on_click(move |states, _| {
                        states.get_mut::<SwitchState>(id).toggle_selection();
                        false
                    })
                    .child(
                        Container::create()
                            .background(id)
                            .border_radius(id)
                            .border_width(id)
                            .border_brush(id)
                            .padding(id)
                            .child(
                                Grid::create()
                                    .child(Container::create().size(24.0, 24.0).build(ctx))
                                    .border_radius(8.0)
                                    .selector(Selector::from("switch-toggle").id("switch_toggle"))
                                    .vertical_alignment("center")
                                    .horizontal_alignment("start")
                                    .build(ctx),
                            )
                            .build(ctx),
                    )
                    .build(ctx),
            )
    }
}
