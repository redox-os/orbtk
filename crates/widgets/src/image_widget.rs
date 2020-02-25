use crate::prelude::*;

widget!(
    /// The `ImageWidget` widget is used to draw an image. It is not interactive.
    ///
    /// **CSS element:** `image-widget`
    ImageWidget {
        /// Sets or shares the image property.
        ///
        /// Set image property:
        /// * &str: `Image::create().image("path/to/image.png").build(xt)`
        /// * String: `Image::create().image(String::from()).build(xt)`
        /// * (width: u32, height: u32, data: Vec<u32>): `Image::create().image((width, height, vec![0; width * height]));`
        image: Image
    }
);

impl Template for ImageWidget {
    fn template(self, _: Entity, _: &mut BuildContext) -> Self {
        self.name("ImageWidget").element("image-widget").image("")
    }

    fn render_object(&self) -> Box<dyn RenderObject> {
        Box::new(ImageRenderObject)
    }

    fn layout(&self) -> Box<dyn Layout> {
        Box::new(FixedSizeLayout::new())
    }
}
