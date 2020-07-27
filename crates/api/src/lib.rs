/*!
   API crate that provides base api and elements for OrbTk like widgets basis.
*/

#[macro_use]
extern crate derive_more;

pub use dces::prelude::*;

pub use orbtk_proc_macros as proc_macros;
pub use orbtk_render::prelude as render;
pub use orbtk_shell::prelude as shell;
pub use orbtk_theme::prelude as theme;
pub use orbtk_theming as theming;
pub use orbtk_tree::prelude as tree;
pub use orbtk_utils::prelude as utils;

pub mod application;
#[macro_use]
pub mod event;
pub mod layout;
pub mod prelude;
pub mod properties;
pub mod render_object;
pub mod services;
pub mod systems;
pub mod widget;

#[macro_use]
pub mod macros;
