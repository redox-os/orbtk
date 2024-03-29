use std::collections::HashSet;

use dces::prelude::*;

/// `SelectedEntities` describes a list of selected entities.
#[derive(Debug, Default, Clone, Eq, PartialEq)]
pub struct SelectedEntities(pub HashSet<Entity>);

impl From<HashSet<Entity>> for SelectedEntities {
    fn from(i: HashSet<Entity>) -> Self {
        SelectedEntities(i)
    }
}
