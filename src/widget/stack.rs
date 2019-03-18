// use crate::{
//     enums::ParentType,
//     layout::StackLayout,
//     properties::OrientationProperty,
//     widget::{Template, Widget},
// };

// widget!(
//     /// The `Stack` represents a layout widget that is used to stack its children on the z-axis.
//     Stack
//     ( OrientationProperty )
// );

// impl Widget for Stack {
//     fn create() -> Self {
//         Stack::new()
//             .orientation("Vertical")
//             .parent_type(ParentType::Multi)
//             .layout(StackLayout::new())
//             .debug_name("Stack")
//     }
// }