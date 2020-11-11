/*!

This crate provides the default theme resources of OrbTks fluent theme dark and light.
It provides fonts, icons and colors.

 */
use orbtk_theme_default::{register_default_fonts, MATERIAL_ICONS, THEME_DEFAULT_FONTS};
use orbtk_theming::{config::ThemeConfig, prelude::*};

pub mod prelude;

/// Resource file of default theme
pub const THEME_FLUENT: &str = include_str!("../theme/theme_fluent.ron");

/// The default dark theme colors resource file.
pub const THEME_FLUENT_COLORS_DARK: &str = include_str!("../theme/theme_fluent_colors_dark.ron");

/// The default light theme colors resource file.
pub const THEME_FLUENT_COLORS_LIGHT: &str = include_str!("../theme/theme_fluent_colors_light.ron");

/// Returns the fluent OrbTk theme.
pub fn theme_fluent() -> Theme {
    theme_fluent_dark()
}

/// Creates OrbTks fluent dark theme.
pub fn theme_fluent_dark() -> Theme {
    register_default_fonts(Theme::from_config(
        ThemeConfig::from(THEME_FLUENT)
            .extend(ThemeConfig::from(THEME_FLUENT_COLORS_DARK))
            .extend(ThemeConfig::from(THEME_DEFAULT_FONTS))
            .extend(ThemeConfig::from(MATERIAL_ICONS)),
    ))
}

/// Creates OrbTks fluent light theme.
pub fn theme_fluent_light() -> Theme {
    register_default_fonts(Theme::from_config(
        ThemeConfig::from(THEME_FLUENT)
            .extend(ThemeConfig::from(THEME_FLUENT_COLORS_LIGHT))
            .extend(ThemeConfig::from(THEME_DEFAULT_FONTS))
            .extend(ThemeConfig::from(MATERIAL_ICONS)),
    ))
}
