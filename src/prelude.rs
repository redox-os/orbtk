pub use std::{
    any::{Any, TypeId},
    cell::RefCell,
    collections::HashMap,
    fmt::Debug,
    rc::Rc,
};

pub use dces::prelude::*;

pub use crate::{
    api::macros::*,
    api::prelude::*,
    localization::prelude::*,
    proc_macros::*,
    render::prelude::*,
    theme_default::{
        colors, fonts, register_default_fonts, theme_default, theme_default_dark,
        theme_default_light, vector_graphics::material_icons_font,
    },
    theming::prelude::*,
    tree::prelude::*,
    utils::prelude::*,
    widgets::prelude::*,
};

// optional uses

#[cfg(feature = "fluent")]
pub use crate::theme_fluent::prelude::*;

#[cfg(feature = "redox")]
pub use crate::theme_redox::prelude::*;
