#![crate_name = "orbtk"]
#![crate_type = "lib"]

pub use orbtk_shell::prelude::initialize;

pub mod core {
    pub use orbtk_core::application;
    pub use orbtk_core::macros;
    pub use orbtk_core::prelude::*;
}

pub mod localization {
    pub use orbtk_localization::prelude::*;
}

pub mod proc_macros {
    pub use orbtk_proc_macros::*;
}

pub mod render {
    pub use orbtk_render::*;
}

pub mod shell {
    pub use orbtk_shell::prelude::*;
}

pub mod theming {
    pub use orbtk_theming::*;
}

pub mod tree {
    pub use orbtk_tree::*;
}

pub mod utils {
    pub use orbtk_utils::*;
}

pub mod widgets {
    pub use orbtk_widgets::*;
}

pub mod prelude;

mod application;

pub use self::application::*;
