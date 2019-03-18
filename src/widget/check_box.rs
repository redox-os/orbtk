// use crate::{
//     material_font_icons,
//     properties::{
//         FontIcon, FontIconProperty, OrientationProperty, PressedProperty, SelectedProperty, Text,
//         TextProperty,
//     },
//     theme::Selector,
//     widget::{Container, FontIconBlock, Property, Stack, Template, TextBlock, Widget},
// };

// widget!(
//     /// The `Checkbox` widget can be switch its selected state. It contains a selection box and a text.
//     CheckBox
//     (
//         TextProperty,
//         FontIconProperty,
//         PressedProperty,
//         SelectedProperty
//     )
// );


// impl Widget for CheckBox {
//     fn create() -> Self {
//         let text = Property::new(Text::default());
//         let icon = Property::new(FontIcon::from(material_font_icons::CHECK_FONT_ICON));
//         let selector = Property::new(Selector::from("checkbox"));

//         CheckBox::new()
//             .height(24.0)
//             .selected(false)
//             .debug_name("CheckBox")
//             .child(
//                 Stack::create()
//                     .orientation("Horizontal")
//                     .child(
//                         Container::create()
//                             .size(24.0, 24.0)
//                             .shared_selector(selector.share())
//                             .child(
//                                 FontIconBlock::create()
//                                     .vertical_alignment("Center")
//                                     .horizontal_alignment("Center")
//                                     .font_icon_prop(icon.share())
//                                     .shared_selector(selector.share()),
//                             ),
//                     )
//                     .child(
//                         TextBlock::create()
//                             .vertical_alignment("Center")
//                             .margin((8.0, 0.0, 0.0, 0.0))
//                             .text_prop(text.share())
//                             .shared_selector(selector.share()),
//                     ),
//             )
//             .font_icon_prop(icon)
//             .text_prop(text)
//             .shared_selector(selector)
//     }
// }