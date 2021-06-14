#![crate_name = "orbtk"]
#![crate_type = "lib"]

//! # OrbTK - The Orbital Toolkit
//!
//! ![Welcome to the OrbTk planet.][orbtk_planet]
//!
//! The `OrbTK` crate implements a cross-platform (G)UI toolkit for
//! building scalable user interfaces. The codebase is natively build
//! with the `Rust` programming language.
//!
//! `OrbTK` provides a [functional reactive-like][functional_reative] API. It depends on
//! the rust [`DCES`][dces] crate, that provides an Entity Component
//! System. Interaction with `DCES` is managed via the `Entity
//! Component Manager`(ECM), a wrapper API, that transparently mapps `OrbTK` widgets
//! to `ECM`  entities and `OrbTK` properties to `ECM` componets.
//!
//! The main goals of `OrbTk` are speed, ease of use, and cross-platform compatibility.
//!
//! Happy 🦀 hacking! ✨
//!
//! [dces]: https://docs.rs/dces
//! [functional_reative]: https://en.wikipedia.org/wiki/Functional_reactive_programming
//! [orbtk_planet]: https://raw.githubusercontent.com/rzerres/orbtk/wip_documentation/orbtk/images/orbtk_planet.svg
// //! [orbtk_planet]: https://raw.githubusercontent.com/redox-os/orbtk/develop/orbtk/images/orbtk_planet.svg

//#![feature(extern_doc)]
//#[doc(include="../README.md")]

pub use orbtk_orbclient::prelude::initialize;

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
    pub use orbtk_tinyskia::*;
}

pub mod shell {
    pub use orbtk_orbclient::prelude::*;
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
