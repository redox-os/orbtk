/*!
   API crate that provides base api and elements for OrbTk like widgets basis.
*/

#[macro_use]
extern crate derive_more;

pub(crate) use orbtk_proc_macros as proc_macros;
pub(crate) use orbtk_render::prelude as render;
pub(crate) use orbtk_theme::prelude as theme;
pub(crate) use orbtk_theming as theming;
pub(crate) use orbtk_tree::prelude as tree;
pub(crate) use orbtk_utils::prelude as utils;

#[cfg(all(
    not(target_arch = "wasm32"),
    any(feature = "default", feature = "orbraq", not(feature = "miniraq"))
))]
pub(crate) use orbtk_shell_orbclient as shell;

#[cfg(all(
    not(target_arch = "wasm32"),
    feature = "miniraq",
    not(feature = "default"),
    not(feature = "orbraq")
))]
pub(crate) use orbtk_shell_minifb as shell;

#[cfg(target_arch = "wasm32")]
pub(crate) use orbtk_shell as shell;

pub mod application;
#[macro_use]
pub mod event;
pub mod layout;
pub mod prelude;
pub mod properties;
pub mod render_object;
pub mod services;
pub mod systems;
pub mod widget_base;

#[macro_use]
pub mod macros;
