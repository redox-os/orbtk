/*!

Window shell abstraction layer used by OrbTk.

 */

use orbgl_api::Canvas;


pub use self::platform::*;
pub use self::update::*;

pub mod prelude;
mod update;

#[cfg(not(target_arch = "wasm32"))]
#[path = "orbclient/mod.rs"]
mod platform;

pub trait WindowShell {
    fn adapter(&mut self, adapter: impl Into<Box<WindowAdapter>>);
    fn canvas(&mut self) -> &mut Canvas;
    fn run(&mut self);
}

pub trait WindowAdapter {
    fn resize(&mut self);
    fn update(&mut self);
}

