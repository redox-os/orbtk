use dces::prelude::Entity;

use crate::{GridLayout, Layout, RenderObject};

use super::BuildContext;

/// The `Template` trait provides the method for the widget template creation.
pub trait Template: Sized {
    /// Creates the template of the widget and returns it.
    fn template(self, _id: Entity, _context: &mut BuildContext) -> Self {
        self
    }

    fn render_object(&self) -> Option<Box<dyn RenderObject>> {
        None
    }

    fn layout(&self) -> Box<dyn Layout> {
        Box::new(GridLayout::new())
    }
}