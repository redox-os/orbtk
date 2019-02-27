use std::rc::Rc;

use crate::{
    enums::Alignment,
    properties::{
        HorizontalAlignment, PaddingProperty, PressedProperty, Selected, SelectedProperty,
    },
    theme::Selector,
    widget::{Container, Context, Grid, Property, State, Template, Widget},
};

// State to handle the position of switch toggle.
struct SwitchState;

impl State for SwitchState {
    fn update(&self, context: &mut Context<'_>) {
        let mut selected = false;
        if let Ok(sel) = context.widget().borrow_property::<Selected>() {
            selected = sel.0;
        }

        let mut switch_toggle = context.child_by_id("SwitchSwitchToggle").unwrap();

        if let Ok(horizontal_alignment) = switch_toggle.borrow_mut_property::<HorizontalAlignment>()
        {
            if selected {
                *horizontal_alignment = HorizontalAlignment(Alignment::End);
            } else {
                *horizontal_alignment = HorizontalAlignment(Alignment::Start);
            }
        }
    }
}

/// The `Switch` widget can be switch between `on` and `off`.
///
/// # Properties
///
/// * `selector` - CSS selector with  element name `checkbox`, used to request the theme of the widget.
/// * `selected` - Bool value represents the selected state of the widget.
///
/// # Others
///
/// * `ParentType`- Single.
pub struct Switch;

impl Widget for Switch {
    type Template = SwitchTemplate;

    fn create() -> Self::Template {
        let selector = Property::new(Selector::from("switch"));
        let selected = Property::new(Selected::from(false));

        SwitchTemplate::new()
            .width(56.0)
            .height(32.0)
            .state(Rc::new(SwitchState))
            .debug_name("Switch")
            .child(
                Container::create()
                    .padding(4.0)
                    .shared_selector(selector.share())
                    .child(
                        Grid::create().child(
                            Container::create()
                                .size(24.0, 24.0)
                                .vertical_alignment("Center")
                                .horizontal_alignment("Start")
                                .attach_shared_property(selected.share())
                                .selector(Selector::from("switchtoggle").id("SwitchSwitchToggle")),
                        ),
                    ),
            )
            .shared_selector(selector)
            .shared_selected(selected)
    }
}

template!(SwitchTemplate, [PressedProperty, SelectedProperty]);
