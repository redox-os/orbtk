//! This module pre-selects commonly used OrbTk crates and put them into scope.

// std
pub use std::rc::Rc;

// crates modules
pub use crate::application::*;
pub use crate::event::*;
pub use crate::layout::*;
pub use crate::localization::*;
pub use crate::macros::*;
pub use crate::properties::*;
pub use crate::render_object::*;
pub use crate::services::*;
pub use crate::systems::*;
pub use crate::theming::*;
pub use crate::tree::*;
pub use crate::widget_base::*;

pub use crate::{into_property_source, trigger_event, widget};
