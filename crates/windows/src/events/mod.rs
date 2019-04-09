//! This module contains all resources to handles events.
//! 
pub use self::key::*;
pub use self::mouse::*;
pub use self::window::*;

mod key;
mod mouse;
mod window;

/// Describes a event form a window.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Event {
    KeyEvent,
    MouseEvent,
    WindowEvent
}