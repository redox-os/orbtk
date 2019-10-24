use std::cell::Cell;

use super::behaviors::MouseBehavior;
use crate::prelude::*;

/// State to handle the position of switch toggle.
#[derive(Default)]
pub struct SwitchState {
    selected: Cell<bool>,
}

impl SwitchState {
    fn toggle_selection(&self) {
        self.selected.set(!self.selected.get());
    }
}

impl State for SwitchState {
    fn update(&self, context: &mut Context<'_>) {
        if context.widget().get::<Selected>("selected").0 == self.selected.get() {
            return;
        }

        context.widget().set("selected", Selected(self.selected.get()));

        let element = context.widget().clone::<Selector>("selector").0.element.unwrap();

        if let Some(parent) = context.parent_entity_by_element(element) {
            context.get_widget(parent).update_theme_by_state(false);
        }

        {
            let mut switch_toggle = context.child_by_id("SwitchSwitchToggle").unwrap();

            switch_toggle.set("selected", Selected(self.selected.get()));

            if self.selected.get() {
                switch_toggle.set("horizontal_alignment", HorizontalAlignment::from("End"));
            } else {
                switch_toggle.set("horizontal_alignment", HorizontalAlignment::from("Start"));
            }

            switch_toggle.update_theme_by_state(true);
        }

        let entity = context.entity_of_child("SwitchSwitchToggle").unwrap();

        context.get_widget(entity).update_theme_by_state(false);
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
        border_radius: BorderRadius,

        /// Sets or shares the border thickness property.
        border_width: BorderThickness,

        /// Sets or shares the border brush property.
        border_brush: Brush,

        /// Sets or shares the padding property.
        padding: Padding,

        /// Sets or shares the css selector property.
        selector: Selector,

        /// Sets or shares the pressed property.
        pressed: Pressed,

        /// Sets or shares the selected property.
        selected: Selected
    }
);

impl Template for Switch {
    fn template(self, id: Entity, context: &mut BuildContext) -> Self {
        let state = self.clone_state();
        self.name("Switch")
            .selector("switch")
            .pressed(false)
            .selected(false)
            .width(56.0)
            .height(32.0)
            .border_brush(colors::BOMBAY_COLOR)
            .background(colors::SLATE_GRAY_COLOR)
            .border_radius(2.0)
            .border_width(1.0)
            .padding(4.0)
            .child(
                MouseBehavior::create()
                    .pressed(id)
                    .enabled(id)
                    .selector(id)
                    .on_click(move |_| {
                        state.toggle_selection();
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
                                    .child(Container::create().size(24.0, 24.0).build(context))
                                    .border_radius(1.0)
                                    .attach_by_source::<Selected>("selected", id)
                                    .selector(
                                        SelectorValue::from("switch-toggle")
                                            .id("SwitchSwitchToggle"),
                                    )
                                    .vertical_alignment("center")
                                    .horizontal_alignment("Start")
                                    .attach_by_source::<Selected>("selected", id)
                                    .build(context),
                            )
                            .build(context),
                    )
                    .build(context),
            )
    }
}
