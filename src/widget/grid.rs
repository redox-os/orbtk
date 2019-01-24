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
            .parent_type(ParentType::Multi)
            .layout(GridLayout::default())
            .render_object(RectangleRenderObject)
            .property(Background::default())
            .property(Columns::default())
            .property(Rows::default())
            .property(Selector::from("grid"))
            .debug_name("Grid")
    }
}
