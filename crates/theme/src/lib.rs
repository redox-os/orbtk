/*!

This crate provides the default theme resources of OrbTks default theme dark and light.
It provides fonts, icons and colors.

 */

#[macro_use]
extern crate lazy_static;

use std::sync::Arc;

use orbtk_css_engine::prelude::*;

/// provides `constants` to reference colors.
pub mod colors;
/// provides `constants` assiciated to fonts.
pub mod fonts;
pub mod prelude;
/// provides information processed by the `graphic render` (e.g. glyphs, icons).
pub mod vector_graphics;

/// orbtk's `default` rendering theme referencing the `dark` stylesheet.
pub const DEFAULT_THEME_CSS: &str = include_str!("dark.css");

/// the `light` variant of orbtk's rendering theme. References the `light` stylesheet.
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
