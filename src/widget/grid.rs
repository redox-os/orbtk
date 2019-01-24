use crate::{
    enums::ParentType,
    layout::GridLayout,
    properties::{Background, Columns, Rows},
    render_object::RectangleRenderObject,
    theme::Selector,
    widget::{Template, Widget},
};

/// Defines a flexible grid area that consists of columns and rows.
///
/// # Properties
///
/// * `Background` - stores the css background.
/// * `Columns` - used to define the columns of the grid.
/// * `Rows` - used to define the rows of the grid.
/// * `Selector` - css selector with element `grid`.
///
/// # CSS properties
///
/// * `background` - defines the background of the widget.
///
/// # Others
///
/// * `ParentType`- Multi.
/// * `GridLayout` - used to layout the widget.
pub struct Grid;

impl Widget for Grid {
    fn create() -> Template {
        Template::default()
            .as_parent_type(ParentType::Multi)
            .with_layout(GridLayout::default())
            .with_render_object(RectangleRenderObject)
            .with_property(Background::default())
            .with_property(Columns::default())
            .with_property(Rows::default())
            .with_property(Selector::from("grid"))
            .with_debug_name("Grid")
    }
}
