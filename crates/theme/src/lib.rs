/*!

This crate provides the default theme resources of OrbTks default theme dark and light.
It provides fonts, icons and colors.

 */

#[macro_use]
extern crate lazy_static;

use std::sync::Arc;

use orbtk_css_engine::prelude::*;

pub mod colors;
pub mod fonts;
pub mod prelude;
pub mod vector_graphics;

pub const DEFAULT_THEME_CSS: &str = include_str!("dark.css");
pub const LIGHT_THEME_EXTENSION_CSS: &str = include_str!("light.css");

lazy_static! {
    pub static ref DEFAULT_THEME: Arc<Theme> =
         Arc::new(Theme::create_from_css(DEFAULT_THEME_CSS).build());
}

lazy_static! {
    pub static ref LIGHT_THEME_CSS: String =
        format!("{}{}", LIGHT_THEME_EXTENSION_CSS, DEFAULT_THEME_CSS);
}

pub fn default_theme() -> Theme {
    Theme::create_from_css(DEFAULT_THEME_CSS).build()
}

pub fn light_theme() -> Theme {
    Theme::create_from_css(&LIGHT_THEME_CSS[..]).build()
}
