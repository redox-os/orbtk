use crate::{
    enums::ParentType,
    layout::GridLayout,
    properties::{Columns, ColumnsProperty, Rows, RowsProperty, BackgroundProperty},
    shapes::Rectangle,
    widget::{Template, Widget},
};

/// Defines a flexible grid area that consists of columns and rows.
///
/// # Properties
///
/// * `columns` - used to define the columns of the grid.
/// * `background` - background brush of the grid.
/// * `rows` - used to define the rows of the grid.
/// * `selector` - css selector with element `grid`.
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
            .shape(Rectangle::default())
            .selector("grid")
            .debug_name("Grid")
    }
}

template!(GridTemplate, [BackgroundProperty, RowsProperty, ColumnsProperty]);
