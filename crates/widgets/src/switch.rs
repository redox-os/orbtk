use std::cell::Cell;

use crate::prelude::*;

/// State to handle the position of switch toggle.
#[derive(Default)]
pub struct SwitchState {
    selected: Cell<bool>,
}

impl SwitchState {
    fn select(&self) {
        self.selected.set(!self.selected.get());
    }
}

// todo: selection helper

impl PressedState for SwitchState {}
impl SelectedState for SwitchState {}

impl State for SwitchState {
    fn update(&self, context: &mut Context<'_>) {
        context.widget().set(Selected(self.selected.get()));
        self.update_pressed(&mut context.widget());
        self.update_selected(&mut context.widget());

        let selected = context.widget().get::<Selected>().0;

        let mut switch_toggle = context.child_by_id("SwitchSwitchToggle").unwrap();
        self.update_selected(&mut switch_toggle);

        if selected {
            switch_toggle.set(HorizontalAlignment::from("End"));
        } else {
            switch_toggle.set(HorizontalAlignment::from("Start"));
        }
    }
}

widget!(
    /// The `Switch` widget can be switch between `on` and `off`.
    ///
    /// **CSS element:** `switch`
    Switch<SwitchState>: ClickHandler {
        /// Sets or shares the background property.
        background: Background,

        /// Sets or shares the border radius property.
        border_radius: BorderRadius,

        /// Sets or shares the border thickness property.
        border_thickness: BorderThickness,

        /// Sets or shares the border brush property.
        border_brush: BorderBrush,

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
            .on_click(move |_| {
                state.select();
                true
            })
            .selector("switch")
            .pressed(false)
            .selected(false)
            .width(56.0)
            .height(32.0)
            .border_brush(colors::BOMBAY_COLOR)
            .background(colors::SLATE_GRAY_COLOR)
            .border_radius(2.0)
            .border_thickness(1.0)
            .padding(4.0)
            .child(
                Container::create()
                    .background(id)
                    .border_radius(id)
                    .border_thickness(id)
                    .border_brush(id)
                    .padding(id)
                    .child(
                        Grid::create()
                            .child(Container::create().size(24.0, 24.0).build(context))
                            .border_radius(1.0)
                            .attach_by_source::<Selected>(id)
                            .selector(SelectorValue::from("switch-toggle").id("SwitchSwitchToggle"))
                            .vertical_alignment("Center")
                            .horizontal_alignment("Start")
                            .attach_by_source::<Selected>(id)
                            .build(context),
                    )
                    .build(context),
            )
    }
}
