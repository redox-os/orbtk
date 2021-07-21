/// Clipboard leads you read and store a value.
///
/// To access the value of systems clipboard it must be used in
/// combination with a window shell.
///
/// # Examples
/// ```
/// impl State for MyState {
///     fn update(&mut self, registry: &mut Registry, _: &mut Context) {
///         let mut clipboard = registry.get_mut;:<Clipboard>("clipboard");
///         println!("{:?}", clipboard.get());
///         clipboard.set("paste");
///     }
/// }
/// ```
#[derive(Clone, Default, Debug)]
pub struct Clipboard {
    value: Option<String>,
}

impl Clipboard {
    /// Creates a new clipboard with default values.
    pub fn new() -> Self {
        Clipboard::default()
    }

    /// Return the latest value of the clipboard.
    /// If there is no value present on the clipboard it will return `None`.
    pub fn get(&self) -> Option<String> {
        self.value.clone()
    }

    /// Sets the value of the clipboard.
    pub fn set(&mut self, value: impl Into<String>) {
        self.value = Some(value.into());
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    /// A quick test to ensure that the items are properly set.
    fn get_set() {
        let mut clipboard = Clipboard::new();
        let test = String::from("test");
        clipboard.set(test.clone());
        assert_eq!(test, clipboard.get().unwrap());
    }
}
