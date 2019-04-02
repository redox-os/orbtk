use crate::prelude::*;

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
