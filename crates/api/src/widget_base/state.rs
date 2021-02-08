use crate::widget_base::{Context, MessageReader, Registry};
use std::any::Any;

pub trait AsAny: Any {
    fn as_any(&self) -> &dyn Any;

    fn as_any_mut(&mut self) -> &mut dyn Any;
}

/// Used to define a state of a [`widget`].
///
/// The state holds the logic of a widget which makes it interactive.
/// The state of a widget is made of a struct which implements this trait with its fields and its methods.
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
///     fn init(&mut self, _ctx: &mut Context, _res: &mut Resources) {
///         self.count = 42;
///         println!("MyState initialized.");
///     }
///
///     fn update(&mut self, _ctx: &mut Context, _res: &mut Resources) {
///         self.count += 1;
///         println("MyState updated.");
///     }
///
///     fn update_post_layout(&mut self, _ctx: &mut Context, _res: &mut Resources) {
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
    /// Init is used for setting up the initial state of a widget, setting up fields to starting values and registering service(s).
    /// It is called after the widget is created.
    ///
    /// # Arguments
    /// * `_ctx`: Represents the context of the current widget.Lets you manipulate the widget tree.
    /// * `_res`: Provides access to the global resources Storage.
    fn init(&mut self, _ctx: &mut Context, _res: &mut Resources) {}

    /// Used to cleanup the state and is called after window close is requested.
    ///
    /// # Arguments
    /// * `_ctx`: Represents the context of the current widget.Allows manipulation of the widget tree.
    /// * `_res`: Provides access to the global resources Storage.
    fn cleanup(&mut self, _ctx: &mut Context, _res: &mut Resources) {}

    /// Updates the state of a widget **before layout is calculated** for the given context when the widget becomes "dirty",
    /// (e.g.: a property of a widget is changed or an [`event`] is fired)
    ///
    /// # Arguments
    /// * `_ctx`: Represents the context of the current widget.Allows manipulation of the widget tree.
    /// * `_res`: Provides access to the global resources Storage.
    ///
    /// [`event`]: ../trait.Event.html
    fn update(&mut self, _ctx: &mut Context, _res: &mut Resources) {}

    /// Used to read messages that are sent to the widget. This will be called after `update` and before `update_post_layout`.
    ///
    /// # Arguments
    /// * `_messages`: Provides access to messages of the widget.
    /// * `_ctx`: Represents the context of the current widget.Allows manipulation of the widget tree.
    /// * `_res`: Provides access to the global resources Storage.
    fn messages(&mut self, _messages: MessageReader, _ctx: &mut Context, _res: &mut Resources) {}

    /// Updates the state **after layout is calculated and before rendering**
    /// for the given context when the widget becomes "dirty",
    /// (e.g.: a property of a widget is changed, or an [`event`] is fired)
    ///
    /// # Arguments
    /// * `_ctx`: Represents the context of the current widget.Allows manipulation of the widget tree.
    /// * `_res`: Provides access to the global resources Storage.
    ///
    /// [`event`]: ../trait.Event.html
    fn update_post_layout(&mut self, _ctx: &mut Context, _res: &mut Resources) {}
}
