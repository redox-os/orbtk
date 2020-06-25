/*!

This crate provides the default theme resources of OrbTks default theme dark and light.
It provides fonts, icons and colors.

 */

use orbtk_theming::{config::ThemeConfig, prelude::*};

pub mod colors;
pub mod fonts;
pub mod prelude;
pub mod vector_graphics;

pub const DEFAULT_THEME_RON: &str = include_str!("../assets/dark/dark.ron");
pub const DEFAULT_COLORS_RON: &str = include_str!("../assets/dark/colors.ron");
pub const DEFAULT_FONTS_RON: &str = include_str!("../assets/dark/fonts.ron");

pub fn dark_theme() -> Theme {
    Theme::from_config(
        ThemeConfig::from(DEFAULT_THEME_RON)
            .extend(ThemeConfig::from(DEFAULT_COLORS_RON))
            .extend(ThemeConfig::from(DEFAULT_FONTS_RON)),
    )
}
