/*!

This submodule provides the default theme resources of OrbTks redox theme dark and light.
It provides fonts, icons and colors.

 */
use orbtk_core::theming::*;

use super::theme_orbtk::{register_default_fonts, MATERIAL_ICONS};

/// Resource file of redox theme.
pub const THEME_REDOX: &str = include_str!("../../assets/themes/redox/theme_redox.ron");

/// The redox dark theme colors resource file.
pub const THEME_REDOX_COLORS_DARK: &str =
    include_str!("../../assets/themes/redox/theme_redox_colors.ron");

/// The font resources of the redox theme.
pub const THEME_REDOX_FONTS: &str = include_str!("../../assets/themes/redox/theme_redox_fonts.ron");

/// Returns the redox OrbTk theme.
pub fn theme_redox() -> Theme {
    register_default_fonts(Theme::from_config(
        ThemeConfig::from(THEME_REDOX)
            .extend(ThemeConfig::from(THEME_REDOX_COLORS_DARK))
            .extend(ThemeConfig::from(THEME_REDOX_FONTS))
            .extend(ThemeConfig::from(MATERIAL_ICONS)),
    ))
}
