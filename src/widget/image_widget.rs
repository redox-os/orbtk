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


use dces::prelude::Entity;

use crate::{
    layout::{FixedSizeLayout, Layout},
    properties::*,
    render_object::{RenderObject, ImageRenderObject},
    styling::{colors, fonts},
    theme::Selector,
    widget::Template,
};

widget!(
    /// The `ImageWidget` widget is used to draw an image. It is not interactive.
    /// 
    /// * CSS element: `image-widget`
    ImageWidget {
        /// Sets or shares the image property.
        image: Image,

        /// Sets or shares the css selector property.
        selector: Selector
    }
);

impl Template for ImageWidget {
    fn template(self, _: Entity, _: &mut BuildContext) -> Self {
        self.name("ImageWidget")
            .selector("image-widget")
            .image("")
    }

    fn render_object(&self) -> Option<Box<dyn RenderObject>> {
        Some(Box::new(ImageRenderObject))
    }

    fn layout(&self) -> Box<dyn Layout> {
        Box::new(FixedSizeLayout::new())
    }
}
