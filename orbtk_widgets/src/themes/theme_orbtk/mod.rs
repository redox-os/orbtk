/*!

This crate provides the default theme resources of OrbTks default theme dark and light.
It provides fonts, icons and colors.

 */

use orbtk_theming::{config::ThemeConfig, prelude::*};

/// provides `constants` to reference colors.
pub mod colors;
/// provides `constants` associated to fonts.
pub mod orbtk_fonts;

/// provides information processed by the `graphic render` (e.g. glyphs, icons).
pub mod material_icons_font;

/// Resource file of default theme
pub const THEME_DEFAULT: &str = include_str!("../../../assets/themes/orbtk/theme_default.ron");

/// The default dark theme colors resource file.
pub const THEME_DEFAULT_COLORS_DARK: &str =
    include_str!("../../../assets/themes/orbtk/theme_default_colors_dark.ron");

/// The default light theme colors resource file.
pub const THEME_DEFAULT_COLORS_LIGHT: &str =
    include_str!("../../../assets/themes/orbtk/theme_default_colors_light.ron");

/// The font resources of the default theme
pub const THEME_DEFAULT_FONTS: &str =
    include_str!("../../../assets/themes/orbtk/theme_default_fonts.ron");

/// Segeo Icon Font map.
pub const MATERIAL_ICONS: &str =
    include_str!("../../../assets/fonts/material/material_icons_font.ron");

/// Returns the default OrbTk theme.
pub fn theme_default() -> Theme {
    theme_default_dark()
}

/// Creates OrbTks default dark theme.
pub fn theme_default_dark() -> Theme {
    register_default_fonts(Theme::from_config(
        ThemeConfig::from(THEME_DEFAULT)
            .extend(ThemeConfig::from(THEME_DEFAULT_COLORS_DARK))
            .extend(ThemeConfig::from(THEME_DEFAULT_FONTS))
            .extend(ThemeConfig::from(MATERIAL_ICONS)),
    ))
}

/// Creates OrbTks default light theme.
pub fn theme_default_light() -> Theme {
    register_default_fonts(Theme::from_config(
        ThemeConfig::from(THEME_DEFAULT)
            .extend(ThemeConfig::from(THEME_DEFAULT_COLORS_LIGHT))
            .extend(ThemeConfig::from(THEME_DEFAULT_FONTS))
            .extend(ThemeConfig::from(MATERIAL_ICONS)),
    ))
}

/// Register roboto and material icon fonts to the given theme.
#[cfg(not(target_arch = "wasm32"))]
pub fn register_default_fonts(theme: Theme) -> Theme {
    theme
        .register_font("Roboto-Regular", orbtk_fonts::ROBOTO_REGULAR_FONT)
        .register_font("Roboto-Medium", orbtk_fonts::ROBOTO_MEDIUM_FONT)
        .register_font("MaterialIcons-Regular", orbtk_fonts::MATERIAL_ICONS_FONT)
}

/// Dummy implementation for web to be compatible to other platforms.
#[cfg(target_arch = "wasm32")]
pub fn register_default_fonts(theme: Theme) -> Theme {
    theme
}
