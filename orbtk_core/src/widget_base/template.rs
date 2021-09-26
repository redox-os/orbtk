use dces::prelude::Entity;

use crate::{layout::*, render_object::*};

use super::BuildContext;

/// The `Template` trait is used to implement a particular widget type.
///
/// When a widget's `Template` is implemented, it provides three object types:
/// * a template objet (with default values for its properties, children, handlers)
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
    ///
    /// Define a widget called MyWidget with min, max and val
    /// properties. The properties have a type of `usize` and we do
    /// preset suitable default values. The definition is completed
    /// with a TextBlock widget, that is the only child.
    ///
    /// ```rust
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

    /// Returns a pointer to a heap allocated object.
    ///
    /// Widgets will be rendered as pixmaps inside the render buffer.
    /// For the list of available render objects, see the
    /// [`render_object`] module.
    ///
    /// [`render_object`]: ../../orbtk_core/render_object/index.html
    fn render_object(&self) -> Box<dyn RenderObject> {
        DefaultRenderObject.into()
    }

    /// Returns a pointer to a heap allocated object.
    ///
    /// The `layout` process will arrange all `box` entities of a widget
    /// in a tree, were size properties will meet the individual constraints.
    /// For the list of available layout objects, see
    /// the [`layout`] module.
    ///
    /// [`layout`]: ../../orbtk_core/layout/index.html
    fn layout(&self) -> Box<dyn Layout> {
        GridLayout::new().into()
    }
}
