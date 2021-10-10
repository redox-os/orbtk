//! Provides dedicated `system` pipelines inside OrbTk.
//!
//! System pipelines are modules, that handle specific tasks when
//! iteratively walking the widget tree. Because each widget
//! implements the `state` trait, all system modules are accessible.
//! Pipelines are connected in a logical order. E.g. the `InitSystem`
//! is triggered **before** the `LayoutSystem`. The `LayoutSystem` is
//! triggerd **before** the `RenderSystem`. Handling of widget objects
//! inside the pipelines rely on the Entity Component System
//! ([`DCES`]).
//!
//! [`DCES`]: https://gitlab.redox-os.org/redox-os/dces-rust

pub use self::cleanup_system::*;
pub use self::event_state_system::*;
pub use self::init_system::*;
pub use self::layout_system::*;
pub use self::post_layout_state_system::*;
pub use self::render_system::*;

mod cleanup_system;
mod event_state_system;
mod init_system;
mod layout_system;
mod post_layout_state_system;
mod render_system;
