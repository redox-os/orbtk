use crate::{
    enums::ParentType,
    properties::{FontIcon, Label, Selected},
    theme::{material_font_icons, Selector},
    widget::{
        Center, Container, FontIconBlock, Row, SharedProperty, Spacer, Template, TextBlock, Widget,
    },
};

/// The `Checkbox` widget can be switch its selected state. It contains a selection box and a label.
///
/// # Shared Properties
///
/// * `Label` - String used to display the text of the check box.
/// * `FontIcon` - String used to display the font icon of the check box.
/// * `Selector` - CSS selector with  element name `checkbox`, used to request the theme of the widget.
///
/// # Properties
///
/// * `Selected` - Bool value represents the selected state of the widget.
///
/// # Others
///
/// * `ParentType`- Single.
pub struct CheckBox;

impl Widget for CheckBox {
    fn create() -> Template {
        let label = SharedProperty::new(Label::default());
        let icon = SharedProperty::new(FontIcon::from(material_font_icons::CHECK_FONT_ICON));
        let selector = SharedProperty::new(Selector::from("checkbox"));

        Template::default()
            .as_parent_type(ParentType::Single)
            .with_child(
                Row::create()
                    .with_child(
                        Container::create()
                            .with_child(
                                Center::create().with_child(
                                    FontIconBlock::create()
                                        .with_shared_property(icon.clone())
                                        .with_shared_property(selector.clone()),
                                ),
                            )
                            .with_shared_property(selector.clone()),
                    )
                    .with_child(Spacer::create())
                    .with_child(
                        Center::create().with_child(
                            TextBlock::create()
                                .with_shared_property(label.clone())
                                .with_shared_property(selector.clone()),
                        ),
                    ),
            )
            .with_shared_property(icon)
            .with_shared_property(label)
            .with_shared_property(selector)
            .with_property(Selected(false))
            .with_debug_name("CheckBox")
    }
}
