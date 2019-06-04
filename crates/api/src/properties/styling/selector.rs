use crate::{
    css_engine::{Selector as SelectorValue, Specificity},
    Entity,
    EntityComponentManager,
    properties::{get_property, PropertySource},
};

property!(
    /// `Selector` describes the css selector of a widget.
    Selector(SelectorValue)
);

// --- Trait implementations ---

impl Selector {
    pub fn dirty(&self) -> bool {
        self.0.dirty()
    }

    pub fn set_dirty(&mut self, dirty: bool) {
        self.0.set_dirty(dirty);
    }

    pub fn specificity(&self) -> Specificity {
        self.0.specificity()
    }

    pub fn matches(&self, other: &SelectorValue) -> bool {
        self.0.matches(other)
    }

    pub fn with<S: Into<String>>(mut self, element: S) -> Self {
        self.0 = self.0.with(element);
        self
    }

    pub fn id<S: Into<String>>(mut self, id: S) -> Self {
        self.0 = self.0.id(id);
        self
    }

    pub fn class<S: Into<String>>(mut self, class: S) -> Self {
        self.0 = self.0.class(class);
        self
    }

    pub fn without_class<S: Into<String>>(mut self, class: S) -> Self {
        self.0 = self.0.without_class(class);
        self
    }

    pub fn pseudo_class<S: Into<String>>(mut self, pseudo_class: S) -> Self {
        self.0 = self.0.pseudo_class(pseudo_class);
        self
    }

    pub fn without_pseudo_class<S: Into<String>>(mut self, pseudo_class: S) -> Self {
        self.0 = self.0.without_pseudo_class(pseudo_class);
        self
    }
}

// --- Conversions ---

impl From<String> for Selector {
    fn from(s: String) -> Selector {
        Selector(SelectorValue::new().with(s))
    }
}

impl From<&str> for Selector {
    fn from(s: &str) -> Selector {
        Selector(SelectorValue::new().with(s.to_string()))
    }
}
