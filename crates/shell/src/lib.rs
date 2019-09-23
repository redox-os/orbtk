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
    update: Rc::new(Cell::new(true)),
    running: Rc::new(Cell:new(true)),
    updater: Box::new(MyCustomUpdater::new())
};

runner.run()
```

 */

extern crate lazy_static;

pub mod event;
pub mod prelude;
pub mod window;

pub use orbtk_utils::prelude as utils;

#[cfg(any(target_os = "linux", target_os = "macos", target_os = "windows"))]
#[path = "minifb/mod.rs"]
pub mod platform;

#[cfg(target_os = "redox")]
#[path = "orbclient/mod.rs"]
pub mod platform;

#[cfg(target_arch = "wasm32")]
#[path = "web/mod.rs"]
pub mod platform;

pub use orbtk_render::prelude as render;
