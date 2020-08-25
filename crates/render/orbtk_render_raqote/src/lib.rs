pub use self::font::*;

#[cfg(feature = "synchronized")]
pub use render_context_2d::*;

#[cfg(not(feature = "synchronized"))]
pub use concurrent::*;

#[cfg(not(feature = "synchronized"))]
mod concurrent;
mod font;
pub mod prelude;
mod render_context_2d;
