use std::sync::Arc;

use crate::prelude::Theme;

pub static DEFAULT_THEME_CSS: &'static str = include_str!("dark.css");
pub static LIGHT_THEME_EXTENSION_CSS: &'static str = include_str!("light.css");

lazy_static! {
    pub static ref DEFAULT_THEME: Arc<Theme> = {
        Arc::new(Theme::create_from_css(DEFAULT_THEME_CSS).build())
    };
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