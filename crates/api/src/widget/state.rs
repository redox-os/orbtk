use super::{Context, Registry};
use std::any::Any;

pub trait AsAny: Any {
    fn as_any(&self) -> &dyn Any;

    fn as_any_mut(&mut self) -> &mut dyn Any;
}

/// Used to define a state of a [`widget`].
/// 
/// The state holds the logic of a widget which makes it interactive.
/// The state of a widget made up of a struct which implements this trait with its fields and its methods.
/// A state of a widget is represented by the current values of its properties.
/// Each state has to implement this trait.
/// Each state has to derive or implement the [Default](https://doc.rust-lang.org/std/default/trait.Default.html) and the [`AsAny`] traits.
/// A state is operating on the properties (components) of the widget, its parent or children, or the state's fields.
/// It is not mandatory to have a state for a widget (in this case it will be static).
/// 
/// # Example
/// ```
/// use orbtk::prelude::*;
/// 
/// #[derive(Default, AsAny)]
/// struct MyState {
///     count: usize
/// }
/// 
/// impl State for MyState {
///     fn init(&mut self, _registry: &mut Registry, _ctx: &mut Context) {
///         self.count = 42;
///         println!("MyState initialized.");
///     }
/// 
///     fn update(&mut self, _registry: &mut Registry, _ctx: &mut Context) {
///         self.count += 1;
///         println("MyState updated.");
///     }
///
///     fn update_post_layout(&mut self, _registry: &mut Registry, _ctx: &mut Context) {
///         println("MyState updated after layout is calculated.");
///     }
/// }
/// 
/// widget!(MyWidget<MyState>)
/// ```
/// 
/// [`widget`]: ./trait.Widget.html
/// [`AsAny`]: ./trait.AsAny.html

pub trait State: AsAny {
    /// Init is used for setup the initial state of a widget, setting fields to starting values, registering service(s).
    /// It is called after the widget is created.
    /// 
    /// # Arguments
    /// * `_registry`: Gives you access to the global Service Registry.
    /// * `_ctx`: Represents the context of the current widget.Lets you manipulate the widget tree.
    fn init(&mut self, _registry: &mut Registry, _ctx: &mut Context) {}

    /// Used to cleanup the state and is called after window close is requested.
    /// # Arguments
    /// * `_registry`: Gives you access to the global Service Registry.
    /// * `_ctx`: Represents the context of the current widget.Lets you manipulate the widget tree.
    fn cleanup(&mut self, _registry: &mut Registry, _ctx: &mut Context) {}

    /// Updates the state of a widget **before layout is calculated** for the given context when the widget becomes "dirty",
    /// (e.g.: a property of a widget is changed or an [`event`] is fired)
    /// 
    /// # Arguments
    /// * `_registry`: Gives you access to the global Service Registry.
    /// * `_ctx`: Represents the context of the current widget.Lets you manipulate the widget tree.
    /// 
    /// [`event`]: ../trait.Event.html
    fn update(&mut self, _registry: &mut Registry, _ctx: &mut Context) {}

    /// Updates the state **after layout is calculated and before rendering**
    /// for the given context when the widget becomes "dirty",
    /// (e.g.: a property of a widget is changed, or an [`event`] is fired)
    ///
    /// # Arguments
    /// * `_registry`: Gives you access to the global Service Registry.
    /// * `_ctx`: Represents the context of the current widget.Lets you manipulate the widget tree.
    ///
    /// [`event`]: ../trait.Event.html
    fn update_post_layout(&mut self, _registry: &mut Registry, _ctx: &mut Context) {}
}
