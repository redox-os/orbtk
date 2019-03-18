// use crate::{
//     enums::ParentType,
//     layout::GridLayout,
//     properties::{BackgroundProperty, Columns, ColumnsProperty, Rows, RowsProperty},
//     render_object::RectangleRenderObject,
//     widget::{Template, Widget},
// };

// widget!(
//     /// Defines a flexible grid area that consists of columns and rows.
//     Grid
//     (BackgroundProperty, RowsProperty, ColumnsProperty)
// );

// impl Widget for Grid {
//     fn create() -> Self {
//         Grid::new()
//             .background("transparent")
//             .columns(Columns::default())
//             .rows(Rows::default())
//             .parent_type(ParentType::Multi)
//             .layout(GridLayout::default())
//             .render_object(RectangleRenderObject)
//             .selector("grid")
//             .debug_name("Grid")
//     }
// }
