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


#[cfg(not(target_arch = "wasm32"))]
#[path = "surfman/mod.rs"]
pub mod platform;
// #[path = "minifb/mod.rs"]
// pub mod platform;

#[cfg(not(target_arch = "wasm32"))]
pub mod native;

#[cfg(target_arch = "wasm32")]
#[path = "web/mod.rs"]
pub mod platform;

pub use orbtk_render::prelude as render;

#[derive(Copy, Clone, Debug)]
pub enum ShellRequest {
    Update,
}
