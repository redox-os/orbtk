use orbtk_theming::{config::ThemeConfig, prelude::*};

/// The fluent dark theme resource file.
pub const FLUENT_DARK_RON: &str = include_str!("../assets/dark.ron");

/// The fluent light theme resource file.
pub const FLUENT_LIGHT_RON: &str = include_str!("../assets/dark.ron");

/// The fluent theme colors resource file.
pub const FLUENT_COLORS_RON: &str = include_str!("../assets/colors.ron");

/// The common fonts resource file.
pub const FLUENT_FONTS_RON: &str = include_str!("../assets/fonts.ron");

pub mod prelude;

/// Creates OrbTks fluent dark theme.
pub fn fluent_dark() -> Theme {
    Theme::from_config(
        ThemeConfig::from(FLUENT_DARK_RON)
            .extend(ThemeConfig::from(FLUENT_COLORS_RON))
            .extend(ThemeConfig::from(FLUENT_FONTS_RON)),
    )
}

/// Creates OrbTks fluent light theme.
pub fn fluent_light() -> Theme {
    Theme::from_config(
        ThemeConfig::from(FLUENT_LIGHT_RON)
            .extend(ThemeConfig::from(FLUENT_COLORS_RON))
            .extend(ThemeConfig::from(FLUENT_FONTS_RON)),
    )
}
