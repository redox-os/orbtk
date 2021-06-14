use std::collections::HashMap;

use ron::{de::from_str, Value};
use serde_derive::{Deserialize, Serialize};

use crate::theming::config::StyleConfig;

pub static BASE_STYLE: &str = "base";
pub static RESOURCE_KEY: &str = "$";

/// Used to store and read properties that could be requested by a
/// given property name and a selector.
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
#[serde(rename = "Theme")]
pub struct ThemeConfig {
    #[serde(default)]
    pub styles: HashMap<String, StyleConfig>,
    #[serde(default)]
    pub resources: HashMap<String, Value>,
}

impl<'a> ThemeConfig {
    /// Extends the given theme with another theme. Replaces the
    /// current name with the new choosen name `other`.  If `other`
    /// contains a style with the same key entry, this key will be
    /// replaced in the current theme.
    pub fn extend(mut self, other: ThemeConfig) -> Self {
        let mut other = other;

        for style in other.styles.drain() {
            self.styles.insert(style.0, style.1);
        }

        for resource in other.resources.drain() {
            self.resources.insert(resource.0, resource.1);
        }

        self
    }
}

impl From<&str> for ThemeConfig {
    fn from(s: &str) -> Self {
        from_str(s).unwrap()
    }
}
