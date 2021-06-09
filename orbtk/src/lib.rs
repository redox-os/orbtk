#![crate_name = "orbtk"]
#![crate_type = "lib"]

pub use orbtk_shell::prelude::initialize;

pub mod core {
    pub use orbtk_core::application;
    pub use orbtk_core::localization;
    pub use orbtk_core::macros;
    pub use orbtk_core::prelude::*;
    pub use orbtk_core::theming;
    pub use orbtk_core::tree;
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

pub mod utils {
    pub use orbtk_utils::*;
}

pub mod widgets {
    pub use orbtk_widgets::*;
}

pub mod prelude;

mod application;

pub use self::application::*;
