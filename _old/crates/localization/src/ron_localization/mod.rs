use std::collections::HashMap;

use crate::localization::Localization;

use dictionary::Dictionary;

mod dictionary;

/// Used to build a new `RonLocalization` and configure language file path and initial language.
#[derive(Debug, Default, Clone)]
pub struct RonLocalizationBuilder {
    language: String,
    dictionaries: HashMap<String, Dictionary>,
}

impl RonLocalizationBuilder {
    /// Adds a new dictionary.
    pub fn dictionary(mut self, key: impl Into<String>, dictionary: &str) -> Self {
        self.dictionaries
            .insert(key.into(), Dictionary::from(dictionary));
        self
    }

    /// Sets the initial language.
    pub fn language(mut self, language: impl Into<String>) -> Self {
        self.language = language.into();
        self
    }

    /// Builds a new ron localization service.
    pub fn build(self) -> RonLocalization {
        RonLocalization {
            language: self.language,
            dictionaries: self.dictionaries,
        }
    }
}

/// `RonLocalization` represents the default implementation of a localization service based on `ron`.
///
/// # Example
///
/// ```rust
/// pub const EN_US: &str = include_str!("../assets/dictionary_en_US.ron");
///
/// let localization = RonLocalization::create().language("en_US").dictionary("en_US", EN_US).build();
/// if let Some(text) = localization.text("hello") {
///     println!("{}", text);
/// }
/// ```
#[derive(Debug, Default, Clone)]
pub struct RonLocalization {
    language: String,
    dictionaries: HashMap<String, Dictionary>,
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

    fn text(&self, key: String) -> String {
        if let Some(dictionary) = self.dictionaries.get(&self.language) {
            if let Some(word) = dictionary.words.get(&key) {
                return word.clone();
            }
        }

        key
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_text() {
        let de_de = r#"
        Dictionary( 
            words: {
                "hello": "Hallo",
                "world": "Welt",
            }
        )
        "#;

        let localization = RonLocalization::create()
            .language("de_DE")
            .dictionary("de_DE", de_de)
            .build();

        assert_eq!(localization.text("hello".to_string()), "Hallo".to_string());
        assert_eq!(localization.text("world".to_string()), "Welt".to_string());
        assert_eq!(localization.text("test".to_string()), "test".to_string());
    }
}
