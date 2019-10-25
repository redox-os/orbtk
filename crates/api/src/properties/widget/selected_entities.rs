use std::collections::HashSet;

use crate::prelude::*;

/// `SelectedEntities` describes a list of selected entities.
#[derive(Clone, Default, Debug)]
pub struct SelectedEntities(pub HashSet<Entity>);

impl From<HashSet<Entity>> for SelectedEntities {
    fn from(i: HashSet<Entity>) -> Self {
        SelectedEntities(i)
    }
}