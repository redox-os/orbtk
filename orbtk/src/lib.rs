//! The Orbital Widget Toolkit is a cross-platform (G)UI toolkit.

pub mod base {
    //! Base crate of OrbTk with all mandatory stuff.
    pub use orbtk_base::*;
}

// im features excludes the orbclient app and window implementation
#[cfg(not(feature = "im"))]
pub mod orbclient {
    //! App and window implementation for OrbTk based on OrbClient.
    pub use orbtk_orbclient::*;
}

pub mod shell {
    //! Immediate mode user interface (ui) shell for OrbTk.
    pub use orbtk_shell::*;
}

pub mod widgets {
    //! Integrated default OrbTk widget library with different themes.
    pub use orbtk_widgets::*;
}
