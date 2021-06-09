/*!

This crate provides the default theme resources of OrbTks fluent theme dark and light.
It provides fonts, icons and colors.

 */
use orbtk_core::theming::*;

/// provides `constants` associated to fonts.
pub mod fluent_fonts;

/// provides information processed by the `graphic render` (e.g. glyphs, icons).
pub mod mdl2_assets_font;

/// Resource file of default theme
pub const THEME_FLUENT: &str = include_str!("../../../assets/themes/fluent/theme_fluent.ron");

/// The default dark theme colors resource file.
pub const THEME_FLUENT_COLORS_DARK: &str =
    include_str!("../../../assets/themes/fluent/theme_fluent_colors_dark.ron");

/// The default light theme colors resource file.
pub const THEME_FLUENT_COLORS_LIGHT: &str =
    include_str!("../../../assets/themes/fluent/theme_fluent_colors_light.ron");

// /// The font resources of the default theme
pub const THEME_FLUENT_FONTS: &str =
    include_str!("../../../assets/themes/fluent/theme_fluent_fonts.ron");

/// Segeo Icon Font map.
pub const MDL2_ASSETS_ICONS: &str = include_str!("../../../assets/fonts/mdl2/mdl2_assets_font.ron");

/// Returns the fluent OrbTk theme.
pub fn theme_fluent() -> Theme {
    theme_fluent_dark()
}

/// Creates OrbTks fluent dark theme.
pub fn theme_fluent_dark() -> Theme {
    register_fluent_fonts(Theme::from_config(
        ThemeConfig::from(THEME_FLUENT)
            .extend(ThemeConfig::from(THEME_FLUENT_COLORS_DARK))
            .extend(ThemeConfig::from(THEME_FLUENT_FONTS))
            .extend(ThemeConfig::from(MDL2_ASSETS_ICONS)),
    ))
}

/// Creates OrbTks fluent light theme.
pub fn theme_fluent_light() -> Theme {
    register_fluent_fonts(Theme::from_config(
        ThemeConfig::from(THEME_FLUENT)
            .extend(ThemeConfig::from(THEME_FLUENT_COLORS_LIGHT))
            .extend(ThemeConfig::from(THEME_FLUENT_FONTS))
            .extend(ThemeConfig::from(MDL2_ASSETS_ICONS)),
    ))
}

/// Register roboto and material icon fonts to the given theme.
#[cfg(not(target_arch = "wasm32"))]
pub fn register_fluent_fonts(theme: Theme) -> Theme {
    theme
        .register_font("Selawik-Regular", fluent_fonts::SELAWIK_REGULAR_FONT)
        .register_font("Selawik-Bold", fluent_fonts::SELAWIK_BOLD_FONT)
        .register_font("Selawik-Light", fluent_fonts::SELAWIK_LIGHT_FONT)
        .register_font("MDL2-Assets-Regular", fluent_fonts::MDL2_ICONS_FONT)
        // register also material icon fonts because OrbTk's default widget library relies on its availability.
        .register_font(
            "MaterialIcons-Regular",
            super::theme_orbtk::orbtk_fonts::MATERIAL_ICONS_FONT,
        )
}

/// Dummy implementation for web to be compatible to other platforms.
#[cfg(target_arch = "wasm32")]
pub fn register_fluent_fonts(theme: Theme) -> Theme {
    theme
}
