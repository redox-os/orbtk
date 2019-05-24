/*!

Window shell abstraction layer used by OrbTk.

 */

use orbgl_api::Canvas;

#[macro_use]
extern crate lazy_static;

pub mod event;
pub mod obsolete;
pub mod prelude;
pub mod window;

#[cfg(not(target_arch = "wasm32"))]
#[path = "orbclient/mod.rs"]
mod platform;

