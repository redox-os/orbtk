use Entity;

/// Not found error.
#[derive(Debug, PartialEq, Eq)]
pub enum NotFound {
    /// Parent could not be found
    Parent(Entity),
    Child(Entity)
}
