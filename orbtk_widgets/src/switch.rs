use super::behaviors::MouseBehavior;

use crate::{api::prelude::*, prelude::*, proc_macros::*};

// --- KEYS --
static ID_SWITCH_TRACK: &str = "switch_track";
static ID_SWITCH_TOGGLE: &str = "switch_toggle";
// --- KEYS --

/// Used to trigger actions on the `Switch` widget.
pub enum SwitchAction {
    /// Toggles the selection state.
    ToggleSelection,

    /// Adjust visual state after selection changed.
    SelectionChanged,
}

/// State to handle the position of switch toggle.
#[derive(Default, AsAny)]
pub struct SwitchState {
    switch_toggle: Entity,
}

impl SwitchState {
    // toggles the selected property
    fn toggle_selection(&self, ctx: &mut Context) {
        let selected: bool = *Switch::selected_ref(&ctx.widget());
        Switch::selected_set(&mut ctx.widget(), !selected);
        toggle_flag("selected", &mut ctx.widget());
        ctx.widget().update(false);
    }

    // update the visual state to the selection state.
    fn update_visual(&self, ctx: &mut Context) {
        let selected: bool = *Switch::selected_ref(&ctx.widget());
        let mut switch_toggle = ctx.get_widget(self.switch_toggle);

        if selected {
            switch_toggle.set("h_align", Alignment::from("end"));
            switch_toggle
                .get_mut::<Selector>("selector")
                .push_state("selected");
        } else {
            switch_toggle.set("h_align", Alignment::from("start"));
            switch_toggle
                .get_mut::<Selector>("selector")
                .remove_state("selected");
        }

        switch_toggle.update(true);
    }
}

impl State for SwitchState {
    fn init(&mut self, _: &mut Registry, ctx: &mut Context) {
        self.switch_toggle = ctx.child(ID_SWITCH_TOGGLE).entity();
        self.update_visual(ctx);
    }

    fn messages(
        &mut self,
        mut messages: MessageReader,
        _registry: &mut Registry,
        ctx: &mut Context,
    ) {
        for message in messages.read::<SwitchAction>() {
            match message {
                SwitchAction::ToggleSelection => self.toggle_selection(ctx),
                SwitchAction::SelectionChanged => self.update_visual(ctx),
            }
        }
    }
}

widget!(
    /// The `Switch` widget can be switch between `selected` and not `selected`.
    ///
    /// **style:** `switch`
    ///
    /// # Example
    ///
    /// ```rust
    /// Switch::new().selected(true).build(ctx)
    /// ```
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

        /// Sets or shares the pressed property.
        pressed: bool,

        /// Sets or shares the selected property.
        selected: bool,

        /// Indicates if the widget is hovered by the mouse cursor.
        hover: bool,

        /// Defines the margin around the inner border.
        container_margin: Thickness
    }
);

impl Template for Switch {
    fn template(self, id: Entity, ctx: &mut BuildContext) -> Self {
        self.name("Switch")
            .style("switch")
            .pressed(false)
            .selected(false)
            .width(36.0)
            .height(30.0)
            .border_radius(8.0)
            .border_width(1.0)
            .padding(4.0)
            .container_margin((2, 8))
            .child(
                MouseBehavior::new()
                    .pressed(id)
                    .enabled(id)
                    .target(id.0)
                    .on_click(move |ctx, _| {
                        ctx.send_message(SwitchAction::ToggleSelection, id);
                        false
                    })
                    .child(
                        Container::new()
                            .style(ID_SWITCH_TRACK)
                            .opacity(id)
                            .background(id)
                            .border_brush(id)
                            .border_width(id)
                            .border_radius(id)
                            .margin(("container_margin", id))
                            .build(ctx),
                    )
                    .child(
                        Container::new()
                            .id(ID_SWITCH_TOGGLE)
                            .opacity(id)
                            .style("switch_toggle")
                            .v_align("center")
                            .h_align("start")
                            .width(20.0)
                            .height(20.0)
                            .border_radius(10.0)
                            .build(ctx),
                    )
                    .build(ctx),
            )
            .on_changed("selected", move |ctx, _| {
                ctx.send_message(SwitchAction::SelectionChanged, id);
            })
    }
}
