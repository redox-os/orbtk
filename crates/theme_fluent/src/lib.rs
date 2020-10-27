/*!

This crate provides the default theme resources of OrbTks fluent theme dark and light.
It provides fonts, icons and colors.

 */

use orbtk_theming::{config::ThemeConfig, prelude::*};

/// provides `constants` associated to fonts.
pub mod fonts;
pub mod prelude;
/// provides information processed by the `graphic render` (e.g. glyphs, icons).
pub mod vector_graphics;

/// Resource file of default theme
pub const THEME_FLUENT: &str = include_str!("../theme/theme_fluent.ron");

/// The default dark theme colors resource file.
pub const THEME_FLUENT_COLORS_DARK: &str = include_str!("../theme/theme_fluent_colors_dark.ron");

/// The default light theme colors resource file.
pub const THEME_FLUENT_COLORS_LIGHT: &str = include_str!("../theme/theme_fluent_colors_light.ron");

/// The font resources of the default theme
pub const THEME_FLUENT_FONTS: &str = include_str!("../theme/theme_fluent_fonts.ron");

/// Returns the fluent OrbTk theme.
pub fn theme_fluent() -> Theme {
    theme_fluent_dark()
}

/// Creates OrbTks fluent dark theme.
pub fn theme_fluent_dark() -> Theme {
    register_fluent_fonts(Theme::from_config(
        ThemeConfig::from(THEME_FLUENT)
            .extend(ThemeConfig::from(THEME_FLUENT_COLORS_DARK))
            .extend(ThemeConfig::from(THEME_FLUENT_FONTS)),
    ))
}

/// Creates OrbTks fluent light theme.
pub fn theme_fluent_light() -> Theme {
    register_fluent_fonts(Theme::from_config(
        ThemeConfig::from(THEME_FLUENT)
            .extend(ThemeConfig::from(THEME_FLUENT_COLORS_LIGHT))
            .extend(ThemeConfig::from(THEME_FLUENT_FONTS)),
    ))
}

/// Register roboto and material icon fonts to the given theme.
#[cfg(not(target_arch = "wasm32"))]
pub fn register_fluent_fonts(theme: Theme) -> Theme {
    theme
        .register_font("Sego-UI-Regular", crate::fonts::SEGOE_UI_REGULAR_FONT)
        .register_font("Sego-UI-Bold", crate::fonts::SEGOE_UI_BOLD_FONT)
        .register_font("Sego-UI-Light", crate::fonts::SEGOE_UI_LIGHT_FONT)
        .register_font("Sego-UI-Semilight", crate::fonts::SEGOE_UI_SEMI_LIGHT_FONT)
        .register_font("Sego-UI-Semibold", crate::fonts::SEGOE_UI_SEMI_BOLD_FONT)
        .register_font(
            "Segoe-MDL2-Assets-Regular",
            crate::fonts::SEG_MDL_ICONS_FONT,
        )
        // register also material icon fonts because OrbTk's default widget library relies on its availability.
        .register_font(
            "MaterialIcons-Regular",
            orbtk_theme_default::fonts::MATERIAL_ICONS_FONT,
        )
}

/// Dummy implementation for web to be compatible to other platforms.
#[cfg(target_arch = "wasm32")]
pub fn register_fluent_fonts(theme: Theme) -> Theme {
    theme
}
