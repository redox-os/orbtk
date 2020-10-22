/*!

This crate provides the redox theme resources of OrbTks redox theme dark and light.
It provides fonts, icons and colors.

 */

use orbtk_theming::{config::ThemeConfig, prelude::*};

/// provides `constants` associated to fonts.
pub mod fonts;
pub mod prelude;
/// provides information processed by the `graphic render` (e.g. glyphs, icons).
pub mod vector_graphics;

/// Resource file of redox theme
pub const THEME_REDOX: &str = include_str!("../theme/theme_redox.ron");

/// The redox dark theme colors resource file.
pub const THEME_REDOX_COLORS_DARK: &str = include_str!("../theme/theme_redox_colors.ron");

/// The font resources of the redox theme
pub const THEME_REDOX_FONTS: &str = include_str!("../theme/theme_redox_fonts.ron");

/// Returns the redox OrbTk theme.
pub fn theme_redox() -> Theme {
    register_fonts(Theme::from_config(
        ThemeConfig::from(THEME_REDOX)
            .extend(ThemeConfig::from(THEME_REDOX_COLORS_DARK))
            .extend(ThemeConfig::from(THEME_REDOX_FONTS)),
    ))
}

#[cfg(not(target_arch = "wasm32"))]
fn register_fonts(theme: Theme) -> Theme {
    theme
        .register_font("Roboto-Regular", crate::fonts::ROBOTO_REGULAR_FONT)
        .register_font("Roboto-Medium", crate::fonts::ROBOTO_MEDIUM_FONT)
        .register_font("MaterialIcons-Regular", crate::fonts::MATERIAL_ICONS_FONT)
}

#[cfg(target_arch = "wasm32")]
fn register_fonts(theme: Theme) -> Theme {
    theme
}
