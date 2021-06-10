/*!
   API crate that provides base api and elements for OrbTk like widgets basis.
*/

#[macro_use]
extern crate derive_more;

pub(crate) use orbtk_orbclient::prelude as shell;
pub(crate) use orbtk_proc_macros as proc_macros;
pub(crate) use orbtk_tinyskia::prelude as render;
pub(crate) use orbtk_utils::prelude as utils;

pub mod application;
pub mod localization;
pub mod theming;
#[macro_use]
pub mod event;
pub mod layout;
pub mod prelude;
pub mod properties;
pub mod render_object;
pub mod services;
pub mod systems;
pub mod tree;
pub mod widget_base;

#[macro_use]
pub mod macros;
