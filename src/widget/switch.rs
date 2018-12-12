use std::rc::Rc;

use enums::{ParentType, Placement};
use properties::{Label, PrimaryFontIcon, SecondaryFontIcon, Selected};
use theme::{material_font_icons, Selector};
use widget::{Container, Context, SharedProperty, Stack, State, Template, Widget, WidgetKey};

// State to handle the position of switch toggle.
struct SwitchState;

impl State for SwitchState {
    fn update(&self, context: &mut Context) {
        let mut switch_toggle = context.widget_from_key("SwitchToggle").unwrap();
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
        let label = SharedProperty::new(Label::default());
        let primary_icon =
            SharedProperty::new(PrimaryFontIcon::from(material_font_icons::CHECK_FONT_ICON));
        let secondary_icon = SharedProperty::new(SecondaryFontIcon::from(
            material_font_icons::CHECK_FONT_ICON,
        ));
        let selector = SharedProperty::new(Selector::from("switch"));
        let selected = SharedProperty::new(Selected(false));
        let switch_toggle_key = WidgetKey::from("SwitchToggle");

        Template::default()
            .as_parent_type(ParentType::Single)
            .with_child(
                Container::create()
                    .with_child(
                        Stack::create().with_child(
                            Container::create()
                                .with_shared_property(selected.clone())
                                .with_property(Placement::default())
                                .with_key(switch_toggle_key.clone())
                                .with_property(Selector::from("switchtoggle")),
                        ),
                    )
                    .with_shared_property(selector.clone()),
            )
            .with_shared_property(primary_icon)
            .with_shared_property(secondary_icon)
            .with_shared_property(label)
            .with_shared_property(selector)
            .with_shared_property(selected)
            .with_child_key(switch_toggle_key)
            .with_state(Rc::new(SwitchState))
            .with_debug_name("Switch")
    }
}
