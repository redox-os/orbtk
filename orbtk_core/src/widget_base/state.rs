use crate::widget_base::{Context, MessageReader, Registry};
use std::any::Any;

pub trait AsAny: Any {
    fn as_any(&self) -> &dyn Any;

    fn as_any_mut(&mut self) -> &mut dyn Any;
}

/// A state trait of a widget (the `state`) comes in handy to provide
/// interactivity.  It is not required to define a `state` for the
/// widget. But if you don't, you cut of the possibility to adapt
/// properties during runtime. The `view` of the widget will stay
/// static.
///
/// When defining a `state` of a widget, it inherits the values of its
/// associated properties (`current values`), as well as the implemented system  To gain access, each
/// state has to derive or implement the [`Default`] and the [`AsAny`]
/// traits. You are free to implement associated functions to the
/// `state`, that react on triggered events or adapt current
/// values. The `properties` are stored via ECM. They are organized in
/// a tree (parent, children or level entities).
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
/// [`Default`]: (https://doc.rust-lang.org/std/default/trait.Default.html)
/// [`AsAny`]: ./trait.AsAny.html

pub trait State: AsAny {
    /// Init is used for setting up the initial state of a widget.
    ///
    /// Within the init process, you preset properties with intended
    /// initial values. You may register service(s).
    /// It is called **after creation** of a widget.
    ///
    /// # Arguments
    /// * `_registry`: Provides access to the global Service Registry.
    /// * `_ctx`: Represents the context of the current widget. Allows manipulation of the widget tree.
    fn init(&mut self, _registry: &mut Registry, _ctx: &mut Context) {}

    /// Cleanup is used to sanitize the sthe state of a widget.
    ///
    /// It is called **after window close** is requested.
    ///
    /// # Arguments
    /// * `_registry`: Provides access to the global Service Registry.
    /// * `_ctx`: Represents the context of the current widget. Allows manipulation of the widget tree.
    fn cleanup(&mut self, _registry: &mut Registry, _ctx: &mut Context) {}

    /// Update is used to update the state of a widget.
    ///
    /// Within the update process, you react on triggered
    /// `events`. You need to adapt, if entities of the given context
    /// are marked **dirty**. Property changes and handler messages
    /// will fire an [`event`].
    /// It is called **before layout calculation** is triggered.
    ///
    /// # Arguments
    /// * `_registry`: Provides access to the global Service Registry.
    /// * `_ctx`: Represents the context of the current widget. Allows manipulation of the widget tree.
    ///
    /// [`event`]: ./trait.Event.html
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
    /// [`event`]: ./trait.Event.html
    fn update_post_layout(&mut self, _registry: &mut Registry, _ctx: &mut Context) {}
}
