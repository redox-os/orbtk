//! App and window implementation for morph_ui based on OrbClient.

mod app;
pub mod error;
mod window;

pub use self::app::*;
pub use self::error::*;
pub use self::window::*;

/// Gets the screen size of the current display.
pub fn screen_size() -> Result<(u32, u32), Error> {
    orbclient::get_display_size().map_err(|_| Error::CannotReadScreenSize)
}
