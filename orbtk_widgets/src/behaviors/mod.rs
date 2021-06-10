//! Behavior widgets are provides a default set of event actions like mouse event handling.
//! Use them as child to expand the event behavior of your widget.

pub use self::mouse_behavior::*;
pub use self::selection_behavior::*;
pub use self::text_behavior::*;

mod mouse_behavior;
mod selection_behavior;
mod text_behavior;
