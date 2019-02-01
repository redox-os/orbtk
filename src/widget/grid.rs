use crate::{
    enums::ParentType,
    layout::GridLayout,
    properties::{Columns, ColumnsProperty, Rows, RowsProperty},
    render_object::RectangleRenderObject,
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
    type Template = GridTemplate;

    fn create() -> Self::Template {
        GridTemplate::new()
            .columns(Columns::default())
            .rows(Rows::default())
            .parent_type(ParentType::Multi)
            .layout(GridLayout::default())
            .render_object(RectangleRenderObject)
            .selector("grid")
            .debug_name("Grid")
    }
}

template!(GridTemplate, [RowsProperty, ColumnsProperty]);
