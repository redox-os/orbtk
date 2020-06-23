use std::collections::HashMap;

use ron::{de::from_str, Value};
use serde_derive::{Deserialize, Serialize};

use crate::{config::StyleConfig, Selector};

pub static BASE_STYLE: &str = "base";
pub static RESOURCE_KEY: &str = "$";

// Used to store and read properties that could be requested by a given property name and a selector.
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
#[serde(rename = "Theme")]
pub struct ThemeConfig {
    #[serde(default)]
    pub styles: HashMap<String, StyleConfig>,
    #[serde(default)]
    pub resources: HashMap<String, Value>,
}

impl<'a> ThemeConfig {
    /// Extends the given theme with a other theme. Replaces the current name with name of other.
    /// If a style with the same key is on other, it will replace the style in the current theme.
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

    /// Gets a property by the given name and a selector.
    pub fn property(&'a self, property: &str, selector: &Selector) -> Option<Value> {
        if let Some(style) = &selector.style {
            if let Some(style) = self.styles.get(style) {
                return self.get_property(property, style, selector);
            }
        }

        // if there is no style read value from base style.
        if let Some(base_style) = self.styles.get(BASE_STYLE) {
            return self.get_property(property, base_style, selector);
        }

        None
    }

    fn get_property(
        &'a self,
        property: &str,
        style: &'a StyleConfig,
        selector: &Selector,
    ) -> Option<Value> {
        // state properties has the most priority
        if let Some(state) = &selector.state {
            if let Some(properties) = style.states.get(state) {
                return self.get_property_value(property, properties);
            }

            // load state properties from based style if there are no other states (recursive through base style).
            if style.base.is_empty() {
                return None;
            }

            if let Some(base_style) = self.styles.get(&style.base) {
                if let Some(properties) = base_style.states.get(state) {
                    return self.get_property_value(property, properties);
                }
            }
        }

        let value = self.get_property_value(property, &style.properties);

        if value.is_some() {
            return value;
        }

        // load properties from based style if there are no other states (recursive through base style).
        if style.base.is_empty() {
            return None;
        }

        if let Some(base_style) = self.styles.get(&style.base) {
            return self.get_property(property, base_style, selector);
        }

        None
    }

    fn get_property_value(
        &self,
        property: &str,
        properties: &'a HashMap<String, Value>,
    ) -> Option<Value> {
        let property = properties.get(property)?;

        // load from resources if the value is a key.
        if let Ok(key) = property.clone().into_rust::<String>() {
            if key.starts_with(RESOURCE_KEY) {
                return Some(self.resources.get(&key.replace(RESOURCE_KEY, ""))?.clone());
            }
        }

        Some(property.clone())
    }
}

impl From<&str> for ThemeConfig {
    fn from(s: &str) -> Self {
        from_str(s).unwrap()
    }
}