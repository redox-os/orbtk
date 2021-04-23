pub mod core {
    //! Core components of OrbTk. Provides base elements to create an user interface.
    pub use orbtk_core::*;
}

pub mod orbclient {
    //! Ready to run (desktop and web) wrapper for OrbTk based on OrbClient.
    pub use orbtk_orbclient::*;
}

pub mod widgets {
    //! Default widget library of OrbTk.
    pub use orbtk_widgets::*;
}

pub mod tiny_skia {
    //! 2D drawing.
    pub use orbtk_tiny_skia::*;
}

pub use self::core::*;
pub use self::orbclient::*;
pub use self::tiny_skia::*;
pub use self::widgets::*;
