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

use platform::{WindowBuilder, Window};
use window_adapter::WindowAdapter;

pub mod event;
pub mod prelude;
pub mod window_adapter;

pub use orbtk_utils::prelude as utils;

#[cfg(all(not(target_arch = "wasm32"), feature = "pfinder"))]
#[path = "glutin/mod.rs"]
pub mod platform;

#[cfg(all(
    not(target_arch = "wasm32"),
    feature = "default",
    not(feature = "pfinder")
))]
#[path = "minifb/mod.rs"]
pub mod platform;

#[cfg(not(target_arch = "wasm32"))]
pub mod native;

#[cfg(target_arch = "wasm32")]
#[path = "web/mod.rs"]
pub mod platform;

pub use orbtk_render::prelude as render;

use std::{collections::HashMap, sync::mpsc};

/// Used to send a request to the window.
#[derive(Clone, Debug)]
pub enum WindowRequest {
    /// Request redraw of the `Windows`s content.
    Redraw,

    /// Request to close the `Windows`.
    Close,

    /// Request to change the title of the `Windows`.
    ChangeTitle(String),
}

/// Used to send a request to the application shell.
pub enum ShellRequest<W> where W: window_adapter::WindowAdapter {
    /// Request redraw of the `Windows`s content.
    CreateWindow(W, WindowSettings, mpsc::Receiver<WindowRequest>),
}

/// Contains settings of a window.
#[derive(Clone, Debug, Default)]
pub struct WindowSettings {
    /// Title of the window.
    pub title: String,

    /// Is the window borderless / without decorations?
    pub borderless: bool,

    /// Is the window resizable?
    pub resizeable: bool,

    /// Will the window always shown on top of other windows.
    pub always_on_top: bool,

    /// The initial position of the window.
    pub position: (f64, f64),

    /// The initial size of the window.
    pub size: (f64, f64),

    /// List of fonts to register.
    pub fonts: HashMap<String, &'static [u8]>,
}

/// Represents an application shell that could handle multiple windows.
pub struct Shell<A: 'static>
where
    A: WindowAdapter,
{
    window_shells: Vec<Window<A>>,
    requests: mpsc::Receiver<ShellRequest<A>>,
}

impl<A> Shell<A>
where
    A: WindowAdapter,
{
    /// Creates a new application shell.
    pub fn new(requests: mpsc::Receiver<ShellRequest<A>>) -> Self {
        Shell {
            window_shells: vec![],
            requests,
        }
    }

    /// Creates a window builder, that could be used to create a window and add it to the application shell.
    pub fn create_window(&mut self, adapter: A) -> WindowBuilder<A> {
        WindowBuilder::new(self, adapter)
    }

    /// Creates a window builder from a settings object.
    pub fn create_window_from_settings(
        &mut self,
        settings: WindowSettings,
        adapter: A,
    ) -> WindowBuilder<A> {
        WindowBuilder::from_settings(settings, self, adapter)
    }

    /// Receives window request from the application and handles them.
    pub fn receive_requests(&mut self) {
        let mut requests = vec![];
        for request in self.requests.try_iter() {
            requests.push(request);
        }

        for request in requests {
            match request {
                ShellRequest::CreateWindow(adapter, settings, window_requests) => {
                    self.create_window_from_settings(settings, adapter)
                        .request_receiver(window_requests)
                        .build();
                }
            }
        }
    }
}