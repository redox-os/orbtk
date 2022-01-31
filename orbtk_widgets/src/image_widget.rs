use crate::{api::prelude::*, proc_macros::*, render::prelude::*};

widget!(
    /// The `ImageWidget` widget is used to draw an image. It is not interactive.
    ///
    /// **style:** `image-widget`
    ImageWidget {
        /// Sets or shares the image property.
        ///
        /// Set image property:
        /// * &str: `ImageWidget::new().image("path/to/image.png").build(xt)`
        /// * String: `ImageWidget::new().image(String::from()).build(xt)`
        /// * (width: u32, height: u32, data: Vec<u32>): `ImageWidget::new().image((width, height, vec![0; width * height]));`
        image: Image
    }
);

impl Template for ImageWidget {
    fn template(self, _: Entity, _: &mut BuildContext) -> Self {
        self.name("ImageWidget").style("image-widget").image("")
    }

    fn render_object(&self) -> Box<dyn RenderObject> {
        ImageRenderObject.into()
    }

    fn layout(&self) -> Box<dyn Layout> {
        FixedSizeLayout::new().into()
    }
}
