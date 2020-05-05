//! This module contains traits to inject custom logic into the window shell.

use crate::{context::ShellContext, event::*, utils::Point};

/// The window adapter is used to work with the window shell.
/// It handles updates from the shell and provides method to update and render its content.
pub trait ShellAdapter<'a> {
    // /// Renders the content.
    // fn render(&mut self, _canvas: &mut Canvas) {}

    // /// Updates the content.
    // fn update(&mut self) {}

    /// Is called after the window is resized.
    fn resize(&mut self, _width: f64, _height: f64) {}

    /// Is called after the mouse was moved.
    fn mouse(&mut self, _x: f64, _y: f64) {}

    /// Is called after the state of a mouse button is changed.
    fn mouse_event(&mut self, _event: MouseEvent) {}

    /// Is called if mouse wheel or trackpad detect scroll event.
    fn scroll(&mut self, _delta_x: f64, _delta_y: f64) {}

    /// Is called after the state of a keyboard key is changed.
    fn key_event(&mut self, _event: KeyEvent) {}

    /// Is called after the quit event of the window is called.
    fn quit_event(&mut self) {}

    /// Gets the current mouse position.
    fn mouse_position(&self) -> Point;

    /// Is called if active state of the window is changed.
    fn active(&mut self, active: bool);

    /// Runs the inner logic of the shell adapter.
    fn run(&mut self, shell_context: &mut ShellContext<'a>);
}