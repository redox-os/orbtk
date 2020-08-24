/*!

Window shell abstraction layer used by OrbTk.

 */
#[macro_use]
extern crate lazy_static;

pub mod event;
pub mod prelude;
pub mod window_adapter;

#[cfg(not(target_arch = "wasm32"))]
pub mod native;

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
pub enum ShellRequest<W>
where
    W: window_adapter::WindowAdapter,
{
    /// Request redraw of the `Windows`s content.
    CreateWindow(W, WindowSettings, mpsc::Receiver<WindowRequest>),

    None,
}

impl<W> Default for ShellRequest<W>
where
    W: window_adapter::WindowAdapter,
{
    fn default() -> Self {
        ShellRequest::None
    }
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
