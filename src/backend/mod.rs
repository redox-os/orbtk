//! This module contains all resources to build a backend for OrbTk.
//! A backend is used to open a window, draw on the screen and to call events.
//! This module contains also an `OrbClient` based backend.

use std::{
    cell::{Cell, RefCell},
    collections::BTreeMap,
    rc::Rc,
};

use dces::prelude::{Entity, World};
use orbclient::Color;
use orbfont::Font;
use orbgl_api::prelude::Canvas;

use crate::prelude::*;

/// [obsolete] This trait is used to define a backend renderer for OrbTk.
pub trait Renderer {
    fn render_text(
        &mut self,
        text: &str,
        bounds: &Bounds,
        parent_bounds: &Bounds,
        global_position: &Point,
        font_size: u32,
        color: Color,
        font: &Font,
    );
}

/// This trait is used to define a backend for OrbTk.
pub trait Backend {
    fn adapter(&mut self) -> Option<&mut WindowAdapter>;
    fn drain_events(&mut self);
}

/// Helper trait to measure the font size of the given `text`.
pub trait FontMeasure {
    fn measure(&self, text: &str, font: &Font, font_size: u32) -> (u32, u32);
}

pub trait WindowAdapter {
    fn update(&mut self);
    fn resize(&mut self, width: f64, height: f64);
    fn mouse(&mut self, x: f64, y: f64);
    fn mouse_event(&mut self, event: MouseEvent);
    fn key_event(&mut self, event: KeyEvent);
    fn quite_event(&mut self);
}

pub use self::platform::{WindowBuilder, ShellRunner, WindowShell, Updater, MouseButton, MouseEvent, ButtonState, KeyEvent };
pub use self::platform::FONT_MEASURE;

#[cfg(not(platform_arch = "wasm32"))]
#[path = "orbital/mod.rs"]
pub mod platform;
