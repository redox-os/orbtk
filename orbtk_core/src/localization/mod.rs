//! This module contains the localization methods, that handle runtime based adaption of involved OrbTk entities.

mod ron_localization;

pub use self::ron_localization::*;

pub trait Localization {
    /// Gets the current language by language key e.g. `en_US`,
    /// `de_DE` or `fr_FR`.
    fn language(&self) -> &String;

    /// Sets the current language by key e.g. `en_US` or `de_DE`.
    fn set_language(&mut self, key: &str);

    /// Gets the translated text for the given key. If there is no
    /// given translation, the `key` will be returned as the result.
    fn text(&self, key: String) -> String;
}
