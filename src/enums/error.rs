use Entity;

/// Used as return type if a requested widget is not found on the tree.
#[derive(Debug, PartialEq, Eq)]
pub enum NotFound {
    /// Parent could not be found.
    Parent(Entity),

    /// Child could not be found
    Child(Entity)
}
