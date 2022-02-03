#![crate_name = "orbtk"]
#![crate_type = "lib"]

//! # OrbTk - The Orbital Toolkit
//!
//! ![Welcome to the OrbTk planet.][orbtk_planet]
//!
//! The `OrbTk` crate implements a cross-platform (G)UI toolkit for
//! building scalable user interfaces. The codebase is natively build
//! with the `Rust` programming language.
//!
//! `OrbTk` provides a [functional reactive-like][functional_reative] API. It depends on
//! the rust [`DCES`][dces] crate, that provides an Entity Component
//! System. Interaction with `DCES` is managed via the `Entity
//! Component Manager`(ECM), a wrapper API, that transparently mapps `OrbTk` widgets
//! to `ECM`  entities and `OrbTk` properties to `ECM` components.
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

/// Tries to make your OrbTk experience more convenient.
/// It will automatically import traits and types into scope, that you likely need in your app.
pub use orbtk_orbclient::prelude::initialize;

/// Handles core implenentations (OrbTk building blocks).
pub mod core {
    pub use orbtk_core::application;
    pub use orbtk_core::localization;
    pub use orbtk_core::macros;
    pub use orbtk_core::prelude::*;
    pub use orbtk_core::theming;
    pub use orbtk_core::tree;
}

/// Handles procedural macros.
pub mod proc_macros {
    pub use orbtk_proc_macros::*;
}

/// Handles renderer implementations.
pub mod render {
    pub use orbtk_tinyskia::*;
}

/// Handles shell interaction implementations.
pub mod shell {
    pub use orbtk_orbclient::prelude::*;
}

/// Handles helper utilities and global methods.
pub mod utils {
    pub use orbtk_utils::*;
}

/// Handle widget implementations.
pub mod widgets {
    pub use orbtk_widgets::*;
}

/// Pre-selects commonly used OrbTk crates and put them into scope.
pub mod prelude;

mod application;

pub use self::application::*;
