use dces::prelude::Entity;

use crate::{layout::*, render_object::*};

use super::BuildContext;

/// The `Template` trait defines the template of a particular type of a widget.
///
/// A widget's `Template` consists three type of objects:
/// * default values of its properties, children, handlers
/// * a render object
/// * a layout object
pub trait Template: Sized {
    /// Builds the template of the widget and returns it.
    ///
    /// # Arguments
    /// * `_id`: The id (Entity) of the instantiated widget in the Entity Store
    /// * `_context`: The BuildContext used to build and instantiate new widgets
    ///
    /// # Example
    /// Define a widget called MyWidget with min, max and val properties with type of usize,
    /// and then set default values and add a TextBlock child.
    ///
    /// ```
    /// widget!(MyWidget {
    ///     min: usize,
    ///     max: usize,
    ///     val: usize
    /// });
    ///
    /// impl Template for MyWidget {
    ///     fn template(self, _id: Entity, context: &mut BuildContext) -> Self {
    ///         self.name("MyWidget")
    ///             .min(100)
    ///             .max(1000)
    ///             .val(500)
    ///             .child(TextBlock::new().text("Set a value!").build(context))
    ///     }
    ///
    ///     fn render_object(&self) -> Box<dyn RenderObject> {
    ///         Box::new(RectangleRenderObject)
    ///     }
    ///
    ///     fn layout(&self) -> Box<dyn Layout> {
    ///        Box::new(AbsoluteLayout)
    ///     }
    /// }
    /// ```
    fn template(self, _id: Entity, _ctx: &mut BuildContext) -> Self {
        self
    }

    /// Returns a pointer to a heap allocated object
    /// which specifies how the widget should be drawn on the canvas.
    /// For the list of available render objects, see the [`render_object`] module.
    fn render_object(&self) -> Box<dyn RenderObject> {
        DefaultRenderObject.into()
    }

    /// Returns a pointer to a heap allocated object
    /// which specifies the way in which the widget are arranged or laid out on the canvas.
    /// For the list of available layout objects, see the [`layout`] module.
    fn layout(&self) -> Box<dyn Layout> {
        GridLayout::new().into()
    }
}
