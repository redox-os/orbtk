#![crate_name = "orbtk"]
#![crate_type = "lib"]

pub use dces::prelude::*;

pub use orbtk_api::prelude as api;
pub use orbtk_css_engine::prelude as css_engine;
pub use orbtk_proc_macros as proc_macros;
pub use orbtk_render::prelude as render;
pub use orbtk_shell::prelude as shell;
pub use orbtk_theme::prelude as theme;
pub use orbtk_tree::prelude as tree;
pub use orbtk_utils::prelude as utils;
pub use orbtk_widgets::prelude as widgets;
pub use shell::initialize;

pub use crate::widgets::*;

pub mod prelude;