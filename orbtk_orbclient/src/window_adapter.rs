//! This module contains traits to inject custom logic into the window shell.

use crate::render::RenderContext2D;
use crate::{event::*, utils::Point};

/// The `WindowAdapter` represents the bridge to the `Shell` backend.
/// It receives events from the `Window` and runs it's own logic.  
pub trait WindowAdapter {
    /// Is called if active state of the window is changed.
    fn active(&mut self, active: bool);

    /// Used to update the clipboard, could be used to read and set the current clipboard value.
    fn clipboard_update(&mut self, value: &mut Option<String>);

    /// This method is called when a file is dropped on the window.
    fn file_drop_event(&mut self, file_name: String);

    /// Is called after the state of a keyboard key is changed.
    fn key_event(&mut self, _event: KeyEvent) {}

    /// Is called after the mouse was moved.
    fn mouse(&mut self, _x: f64, _y: f64) {}

    /// Is called after the state of a mouse button is changed.
    fn mouse_event(&mut self, _event: MouseEvent) {}

    /// Gets the current mouse position.
    fn mouse_position(&self) -> Point;

    /// Is called after the quit event of the window is called.
    fn quit_event(&mut self) {}

    /// Is called after the window is resized.
    fn resize(&mut self, _width: f64, _height: f64) {}

    /// Runs the inner logic of the shell adapter.
    fn run(&mut self, render_context: &mut RenderContext2D);

    /// Is called if mouse wheel or trackpad detect scroll event.
    fn scroll(&mut self, _delta_x: f64, _delta_y: f64) {}

    /// Sets raw window handle.
    fn set_raw_window_handle(&mut self, raw_window_handle: raw_window_handle::RawWindowHandle);

    /// Is called when the keyboard emits an text input.
    fn text_input(&mut self, _text: String) {}

    /// This method is called when a text string is dropped on the window.
    fn text_drop_event(&mut self, text: String);
}
