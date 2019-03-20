use dces::prelude::Entity;

use crate::{
    layout::{Layout, StackLayout},
    properties::*,
    render_object::{RectangleRenderObject, RenderObject},
    theme::Selector,
    widget::Template,
};

widget!(
    /// The `Stack` defines a layout that is used to stack its children on the z-axis.
    /// 
    /// * CSS element: `stack`
    Stack {
        /// Sets or shares the orientation property.
        orientation: Orientation,

        /// Sets or shares the css selector property.
        selector: Selector
    }
);

impl Template for Stack {
    fn template(self, _: Entity, _: &mut BuildContext) -> Self {
        self.name("Stack").selector("stack")
    }

    fn layout(&self) -> Box<dyn Layout> {
        Box::new(StackLayout::new())
    }
}
