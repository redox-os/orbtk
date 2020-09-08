use std::collections::HashMap;

use crate::localization::Localization;

use dictionary::Dictionary;

mod dictionary;

/// Used to build a new `RonLocalization` and configure language file path and initial language.
#[derive(Debug, Default, Clone)]
pub struct RonLocalizationBuilder {
    language: String,
    path: String,
}

impl RonLocalizationBuilder {
    /// Sets path of the language files.
    pub fn path(mut self, path: String) -> Self {
        self.path = path;
        self
    }

    /// Sets the initial language.
    pub fn language(mut self, path: String) -> Self {
        self.language = language;
        self
    }

    pub fn build(self) -> RonLocalization {
        // todo load file and set properties
        RonLocalization::default()
    }
}

/// `RonLocalization` represents the default implementation of a localization service based on `ron`.
///
/// # Example
/// tbd
#[derive(Debug, Default, Clone)]
pub struct RonLocalization {
    language: String,
    dictionary: Option<Dictionary>,
}

impl RonLocalization {
    /// Creates a new `RonLocalizationBuilder` to configure the localization service.
    pub fn create() -> RonLocalizationBuilder {
        RonLocalizationBuilder::default()
    }
}

impl Localization for RonLocalization {
    fn language(&self) -> &String {
        &self.language
    }

    fn set_language(&mut self, key: &str) {
        self.language = key.to_string();
    }

    fn text(&self, key: &str) -> String {
        if let Some(word) = self.dictionary[key] {
            return word.clone();
        }

        key.to_string()
    }
}
