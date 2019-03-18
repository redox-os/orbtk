// use crate::{
//     layout::FixedSizeLayout,
//     properties::ImageProperty,
//     render_object::ImageRenderObject,
//     widget::{Template, Widget},
// };

// widget!(
//     /// The `ImageWidget` widget is used to draw an image. It is not interactive.
//     ImageWidget
//     (ImageProperty)
// );

// impl Widget for ImageWidget {
//     fn create() -> Self {
//         ImageWidget::new()
//             .selector("imagewidget")
//             .layout(FixedSizeLayout::new())
//             .render_object(ImageRenderObject)
//             .debug_name("ImageWidget")
//     }
// }
