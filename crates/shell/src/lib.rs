/*!

Window shell abstraction layer used by OrbTk. Provides support for desktop and web.

# Example

Basic usage of the shell:

```rust,no_run

use orbtk_shell::prelude::*;

let shell = WindowBuilder::new(MyCustomWindowAdapter::new())
                        .title("Window")
                        .bounds((0.0, 0.0, 100.0, 100.0))
                        .build();

let runner = ShellRunner {
    shell,
    updater: Box::new(MyCustomUpdater::new())
};

runner.run()
```

 */
#[macro_use]
extern crate lazy_static;

pub mod event;
pub mod prelude;
pub mod window;

pub use orbtk_utils::prelude as utils;

#[cfg(all(not(target_arch = "wasm32"), feature = "pfinder"))]
#[path = "glutin/mod.rs"]
pub mod platform;

#[cfg(all(not(target_arch = "wasm32"), feature = "default", not(feature = "pfinder")))]
#[path = "minifb/mod.rs"]
pub mod platform;

#[cfg(not(target_arch = "wasm32"))]
pub mod native;

#[cfg(target_arch = "wasm32")]
#[path = "web/mod.rs"]
pub mod platform;

pub use orbtk_render::prelude as render;

/// Used to sent requests to the window `Shell`.
#[derive(Clone, Debug)]
pub enum ShellRequest {
    /// Request redraw of the `Shell`s content.
    Update,

    /// Request redraw of the `Shell`s content.
    Redraw,

    /// Request to close the `Shell`.
    Close,

    /// Request to resize the `Shell` to the specified size.
    Resize(f64, f64),

    /// Request to change the title of the `Shell`.
    ChangeTitle(String),
}

#[derive(Clone, Debug)]
pub enum WindowRequest {
    /// Request redraw of the `Windows`s content.
    Redraw,

    /// Request to close the `Windows`.
    Close,

    /// Request to change the title of the `Windows`.
    ChangeTitle(String),
}
