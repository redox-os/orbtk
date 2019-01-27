use std::rc::Rc;

use crate::{
    enums::{ParentType, Placement},
    properties::{Text, PrimaryFontIcon, SecondaryFontIcon, Selected},
    styling::vector_graphics::material_font_icons,
    theme::Selector,
    widget::{Container, Context, SharedProperty, Stack, State, Template, Widget},
};

// State to handle the position of switch toggle.
struct SwitchState;

impl State for SwitchState {
    fn update(&self, context: &mut Context<'_>) {
        let mut switch_toggle = context.widget_from_id("SwitchSwitchToggle").unwrap();
        let mut selected = false;

        if let Ok(sel) = switch_toggle.borrow_property::<Selected>() {
            selected = sel.0;
        }

        if let Ok(placement) = switch_toggle.borrow_mut_property::<Placement>() {
            if selected {
                *placement = Placement::Right;
            } else {
                *placement = Placement::Left;
            }
        }
    }
}

/// The `Switch` widget can be switch between `on` and `off`.
///
/// # Shared Properties
///
/// * `PrimaryFontIcon` - String used to display the font icon of the on state.
/// * `SecondaryFontIcon` - String used to display the font icon of the off state.
/// * `Selector` - CSS selector with  element name `checkbox`, used to request the theme of the widget.
///
/// # Properties
///
/// * `Selected` - Bool value represents the selected state of the widget.
///
/// # Others
///
/// * `ParentType`- Single.
pub struct Switch;

impl Widget for Switch {
    fn create() -> Template {
        let text = SharedProperty::new(Text::default());
        let primary_icon =
            SharedProperty::new(PrimaryFontIcon::from(material_font_icons::CHECK_FONT_ICON));
        let secondary_icon = SharedProperty::new(SecondaryFontIcon::from(
            material_font_icons::CHECK_FONT_ICON,
        ));
        let selector = Selector::from("switch");
        let selected = SharedProperty::new(Selected(false));

        Template::default()
           .parent_type(ParentType::Single)
            .child(
                Container::create()
                    .child(
                        Stack::create().child(
                            Container::create()
                                .shared_property(selected.clone())
                                .property(Placement::default())
                                .property(
                                    Selector::from("switchtoggle").id("SwitchSwitchToggle"),
                                ),
                        ),
                    )
                    .property(selector.clone()),
            )
            .shared_property(primary_icon)
            .shared_property(secondary_icon)
            .shared_property(text)
            .property(selector)
            .shared_property(selected)
            .state(Rc::new(SwitchState))
            .debug_name("Switch")
    }
}
