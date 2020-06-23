/// The selector is used to read a property value from the `Theme`.
#[derive(Debug, Clone, Default)]
pub struct Selector {
    /// Represents the key of a style.
    pub style: Option<String>,

    /// Used to reference the state property list of the given style.
    pub state: Option<String>,

    /// Check if the selector is dirty.
    pub dirty: bool,
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
    }

    /// Clears the current state and reset to default.
    pub fn clear_state(&mut self) {
        self.state = None;
    }
}