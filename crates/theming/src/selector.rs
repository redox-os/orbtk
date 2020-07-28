use std::fmt;

/// The selector is used to read a property value from the `Theme`.
#[derive(Debug, Clone, Default, PartialEq)]
pub struct Selector {
    /// Represents the key of a style.
    pub style: Option<String>,

    /// Used to reference the state property list of the given style.
    pub state: Option<String>,

    /// Check if the selector is dirty.
    dirty: bool,
}

impl Selector {
    /// Creates a new selector with the given style key.
    pub fn new(style: impl Into<String>) -> Self {
        Selector {
            style: Some(style.into()),
            state: None,
            dirty: true,
        }
    }

    /// Set the current state of the selector.
    pub fn set_state(&mut self, state: impl Into<String>) {
        self.state = Some(state.into());
        self.dirty = true;
    }

    /// Clears the current state and reset to default.
    pub fn clear_state(&mut self) {
        self.state = None;
        self.dirty = true;
    }

    /// Gets the dirty flag.
    pub fn dirty(&self) -> bool {
        self.dirty
    }

    /// Sets the dirty flag.
    pub fn set_dirty(&mut self, dirty: bool) {
        self.dirty = dirty;
    }

    /// Check if the selector has the given state.
    pub fn has_state(&self, state: &str) -> bool {
        if let Some(st) = &self.state {
            if st.as_str() == state {
                return true;
            }
        }

        return false;
    }
}

impl fmt::Display for Selector {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if let Some(style) = &self.style {
            return write!(f, "Selector ( style: {} )", style);
        }
        write!(f, "Selector ( empty )")
    }
}

impl From<&str> for Selector {
    fn from(s: &str) -> Self {
        Selector::new(s)
    }
}

impl From<String> for Selector {
    fn from(s: String) -> Self {
        Selector::new(s)
    }
}
