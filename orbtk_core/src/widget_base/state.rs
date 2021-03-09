use crate::widget_base::{Context, MessageReader, Registry};
use std::any::Any;

pub trait AsAny: Any {
    fn as_any(&self) -> &dyn Any;

    fn as_any_mut(&mut self) -> &mut dyn Any;
}

/// A state trait of a widget (the `state`) comes in handy to provide
/// interactivity.  You are not requested to define a `state` for your
/// widget. If you don't you can't change the properties. The `view`
/// of widget will stay static.
///
/// Each defined `state` of a widget inherits the values of its
/// associated properties (`current values`).  To gain the access each
/// state has to derive or implement the
/// [`Default`](https://doc.rust-lang.org/std/default/trait.Default.html)
/// and the [`AsAny`] traits.  The defined methods of the `state` will
/// operate on the properties of the widget (the DECS components),
/// which are organized in the tree (parent, children or level
/// entities).
///
/// # Example
///
/// The following code will define a widget called `MyWidget` (the
/// `view`) with an associcated state called `MyState`. The `MyState`
/// structure defines a propery `count` (a level entity), that will
/// store values of type usize.  Inside the `state` trait, the method
/// `init` will manipulate the value of the count property (42). The
/// `update` method will increment the `count` property each time the
/// `view` got dirty and initiates a new render cycle.
///
/// ```rust
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
    /// Init is used to set up the initial state of a widget. The
    /// `state` fields values (level entities) are preset and
    ///  service(s) are registered. This method is called after the
    ///  widget is created.
    ///
    /// # Arguments
    /// * `_registry`: Provides access to the global Service Registry.
    /// * `_ctx`: Represents the context of the current widget.
    /// This structure allows the manipulation of the widget tree.
    fn init(&mut self, _registry: &mut Registry, _ctx: &mut Context) {}

    /// Used to cleanup the state and is called after window close is requested.
    /// # Arguments
    /// * `_registry`: Provides access to the global Service Registry.
    /// * `_ctx`: Represents the context of the current widget.
    /// This structure allows the manipulation of the widget tree.
    fn cleanup(&mut self, _registry: &mut Registry, _ctx: &mut Context) {}

    /// Updates the state of a widget **before layout is calculated**.
    /// When the widget gets `dirty`, that means a property of a
    /// widget is changed or an [`event`] is fired up, the update
    /// cycle is initiated.
    ///
    /// # Arguments * `_registry`: Provides access to the global
    /// Service Registry.
    /// * `_ctx`: Represents the context of the current widget.
    /// This struct allows the manipulation of the widget tree.
    ///
    /// [`event`]: ../trait.Event.html
    fn update(&mut self, _registry: &mut Registry, _ctx: &mut Context) {}

    /// This method is called, when a widget sends a message via the `MessageHandler`.
    /// `messages` will read from the message queue. This will
    /// be called after `update` and before `update_post_layout`.
    ///
    /// # Arguments
    /// * `_messages`: Provides access to messages of the widget.
    /// * `_registry`: Provides access to the global Service Registry.
    /// * `_ctx`: Represents the context of the current widget.Allows manipulation of the widget tree.
    fn messages(&mut self, _messages: MessageReader, _registry: &mut Registry, _ctx: &mut Context) {
    }

    /// Updates the state **after layout is calculated and before rendering**
    /// for the given context when the widget becomes "dirty",
    /// (e.g.: a property of a widget is changed, or an [`event`] is fired)
    ///
    /// # Arguments
    /// * `_registry`: Provides access to the global Service Registry.
    /// * `_ctx`: Represents the context of the current widget.Allows manipulation of the widget tree.
    ///
    /// [`event`]: ../trait.Event.html
    fn update_post_layout(&mut self, _registry: &mut Registry, _ctx: &mut Context) {}
}
