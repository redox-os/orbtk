use std::collections::HashMap;

use ron::Value;

use crate::theming::{
    config::{ThemeConfig, RESOURCE_KEY},
    Selector, Style, ThemeState,
};

/// Theme is used to read properties for a given selector with a internal state.
#[derive(Debug, Clone, Default, PartialEq)]
pub struct Theme {
    styles: HashMap<String, Style>,
    fonts: HashMap<String, &'static [u8]>,
}

impl Theme {
    /// Creates a theme from the given config.
    pub fn from_config(config: ThemeConfig) -> Self {
        let mut styles = HashMap::new();

        for style_key in config.styles.keys() {
            Theme::read_config(style_key, style_key, &config, &mut styles)
        }

        Theme {
            styles,
            fonts: HashMap::new(),
        }
    }

    /// Registers a new font file as binary.
    pub fn register_font(mut self, key: &str, font: &'static [u8]) -> Self {
        self.fonts.insert(key.to_string(), font);
        self
    }

    /// Returns the map of registered fonts.
    pub fn fonts(&self) -> &HashMap<String, &'static [u8]> {
        &self.fonts
    }

    /// Returns a reference to the style corresponding to the key.
    pub fn style(&self, key: &str) -> Option<&Style> {
        self.styles.get(key)
    }

    pub fn properties(&self, selector: &Selector) -> Option<HashMap<String, Value>> {
        if !selector.dirty() {
            return None;
        }

        if let Some(style) = &selector.style {
            let mut properties = HashMap::new();

            if let Some(style) = self.styles.get(style) {
                for (key, value) in &style.properties {
                    properties.insert(key.clone(), value.clone());
                }

                // reverse order because last active state has highest priority
                for state in style.states.iter().rev() {
                    if selector.states().contains(&state.key) {
                        for (key, value) in &state.properties {
                            // properties of the selected state overrides default properties
                            properties.insert(key.clone(), value.clone());
                        }

                        break;
                    }
                }
            }

            return Some(properties);
        }

        None
    }

    // reads the given config and copy it's data in the given styles map
    fn read_config(
        style_key: &str,
        base_key: &str,
        config: &ThemeConfig,
        styles: &mut HashMap<String, Style>,
    ) {
        if style_key.is_empty() {
            return;
        }

        if !styles.contains_key(style_key) {
            styles.insert(style_key.to_string(), Style::new());
        }

        // start from topmost base
        if let Some(style) = config.styles.get(base_key) {
            if !style.base.is_empty() && style.base != *style_key {
                Theme::read_config(style_key, &style.base, config, styles);
            }
        }

        if let Some(style_config) = config.styles.get(base_key) {
            if let Some(style) = styles.get_mut(style_key) {
                // reads the properties
                for (property_key, property_value) in &style_config.properties {
                    style.properties.insert(
                        property_key.clone(),
                        Theme::read_value(property_value, &config.resources),
                    );
                }

                // reads the states
                for state in &style_config.states {
                    let mut new_state = ThemeState::new(state.key.clone());

                    for (property_key, property_value) in &state.properties {
                        new_state.properties.insert(
                            property_key.clone(),
                            Theme::read_value(property_value, &config.resources),
                        );
                    }

                    // a state with the same name on a lower base style or the indented style overrides the state of a higher base
                    if let Some(pos) = style.states.iter().position(|s| s.key == new_state.key) {
                        style.states.remove(pos);
                        style.states.insert(pos, new_state)
                    } else {
                        style.states.push(new_state);
                    }
                }
            }
        }
    }

    // if the property value is a place holder replace it with the corresponding value of the resources
    fn read_value(property_value: &Value, resources: &HashMap<String, Value>) -> Value {
        if let Ok(value) = property_value.clone().into_rust::<String>() {
            if let Some(replace_value) = resources.get(&value.replace(RESOURCE_KEY, "")) {
                return replace_value.clone();
            }
        }

        property_value.clone()
    }
}
