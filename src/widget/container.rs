use dces::prelude::Entity;

use crate::{properties::*, theme::Selector, widget::Template, render_object::{RenderObject, RectangleRenderObject}, layout::{Layout, PaddingLayout}};

widget!(
    /// The `Container` layout widget surrounds its child with a padding. Draws a box around the child.
    Container {
        /// Sets or shares the background property.
        background: Background,

        /// Sets or shares the border radius property.
        border_radius: BorderRadius,

        /// Sets or shares the border thickness property.
        border_thickness: BorderThickness,

        /// Sets or shares the border brush property.
        border_brush: BorderBrush,

        /// Sets or shares the padding property.
        padding: Padding,

        /// Sets or shares the css selector property. 
        selector: Selector
    }
);

impl Template for Container {
    fn template(self, _: Entity, _: &mut BuildContext) -> Self {
        self.name("Container").padding(0.0)
            .background("transparent")
            .border_radius(0.0)
            .border_thickness(0.0)
            .border_brush("transparent")
    }

    fn render_object(&self) -> Option<Box<dyn RenderObject>> {
        Some(Box::new(RectangleRenderObject))
    }

    fn layout(&self) -> Box<dyn Layout> {
        Box::new(PaddingLayout::new())
    }
}
