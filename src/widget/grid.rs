use crate::{
    enums::ParentType,
    layout::GridLayout,
    properties::{Background, Columns},
    render_object::RectangleRenderObject,
    theme::Selector,
    widget::{Template, Widget},
};

// todo: documentation, add all default properties (bounds, margin, ...) to the widgets, also for a better documentation
/// Grid ...
///
/// # Properties
///
/// * Background
///
/// # CSS properties
///
/// * background
pub struct Grid;

impl Widget for Grid {
    fn create() -> Template {
        Template::default()
            .as_parent_type(ParentType::Multi)
            .with_layout(GridLayout::default())
            .with_render_object(RectangleRenderObject)
            .with_property(Background::default())
            .with_property(Columns::default())
            .with_property(Selector::from("grid"))
            .with_debug_name("Grid")
    }
}
