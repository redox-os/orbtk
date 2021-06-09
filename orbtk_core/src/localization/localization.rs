///
pub trait Localization {
    /// Gets the current language by language key e.g. `en_US` or `de_DE`.
    fn language(&self) -> &String;

    /// Sets the current language by key e.g. `en_US` or `de_DE`.
    fn set_language(&mut self, key: &str);

    /// Gets the translated text for the given key. If there is no given translation the `key` will be returned as result.
    fn text(&self, key: String) -> String;
}
