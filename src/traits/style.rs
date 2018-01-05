pub trait Style {
    fn with_class<S: Into<String>>(&self, class: S) -> &Self;
    fn with_pseudo_class<S: Into<String>>(&self, pseudo_class: S) -> &Self;
}
