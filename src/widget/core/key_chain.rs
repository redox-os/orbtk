use std::rc::Rc;
use std::cell::Cell;
use std::collections::HashMap;

use dces::Entity;

/// The `WidgetKey` struct is used to get access of a widget by using a string.
pub struct WidgetKey {
    /// The `key` string is used to call a widget with a string.
    key: String,

    /// The `entity` will generated on runtime and is used to access the widget.
    pub entity: Rc<Cell<Option<Entity>>>,
}

impl From<&str> for WidgetKey {
    fn from(s: &str) -> WidgetKey {
        WidgetKey {
            key: s.to_string(),
            entity: Rc::new(Cell::new(None)),
        }
    }
}

impl From<String> for WidgetKey {
    fn from(s: String) -> WidgetKey {
        WidgetKey {
            key: s,
            entity: Rc::new(Cell::new(None)),
        }
    }
}

impl Clone for WidgetKey {
    fn clone(&self) -> Self {
        WidgetKey {
            key: self.key.clone(),
            entity: self.entity.clone(),
        }
    }
}

/// The `KeyChain` structs provides all widget children keys.
pub struct KeyChain {
    key_chain: HashMap<String, WidgetKey>,
}

impl Default for KeyChain {
    fn default() -> Self {
        KeyChain {
            key_chain: HashMap::new(),
        }
    }
}

impl KeyChain {
    /// If the the key chain contains the requested key it will return the entity of the requested widget, otherweise it will return `None`.
    pub fn get(&self, key: &str) -> Option<Entity> {
        if let Some(key) = self.key_chain.get(key) {
            return key.entity.get();
        } else {
            None
        }
    }

    /// Registers a new key in the key chain.
    pub fn register_key(&mut self, key: WidgetKey) {
        self.key_chain.insert(key.key.clone(), key);
    }
}