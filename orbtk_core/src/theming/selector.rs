use std::fmt;

/// The selector is used to read a property value from the `Theme`.
#[derive(Debug, Clone, Default, PartialEq)]
pub struct Selector {
    /// Represents the key of a style.
    pub style: Option<String>,

    // Used to reference the state property list of the given
    // style. The state on the top of the vector is the active one.
    states: Vec<String>,

    /// Check if the selector is dirty.
    dirty: bool,
}

impl Selector {
    /// Creates a new selector with the given style key.
    pub fn new(style: impl Into<String>) -> Self {
        Selector {
            style: Some(style.into()),
            states: vec![],
            dirty: true,
        }
    }

    /// Returns a reference to list of active states.
    pub fn states(&self) -> &Vec<String> {
        &self.states
    }

    /// Pushes a state to the states vector.
    pub fn push_state(&mut self, state: impl Into<String>) {
        let state = state.into();

        if self.states.contains(&state) {
            return;
        }

        self.states.push(state);
        self.dirty = true;
    }

    /// Removes all states with a similar name compared to the given pattern.
    pub fn remove_all_similar_states(&mut self, pattern: &str) {
        while let Some(pos) = self.states.iter().position(|x| x.contains(pattern)) {
            self.states.remove(pos);
            self.dirty = true;
        }
    }

    /// Removes all instances of the give state from the vector and returns it.
    pub fn remove_state(&mut self, state: impl Into<String>) -> Option<String> {
        let state: String = state.into();

        if !state.is_empty() && self.states.contains(&state) {
            while let Some(pos) = self.states.iter().position(|x| *x == state) {
                self.states.remove(pos);
            }

            self.dirty = true;
            return Some(state);
        }

        None
    }

    /// Removes the last state from a vector and returns it, or None
    /// if it is empty. If there is a new last vector it will be the
    /// new active state.
    pub fn pop_state(&mut self) -> Option<String> {
        self.dirty = true;
        self.states.pop()
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
        self.states.contains(&state.to_string())
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
