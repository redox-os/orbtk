use std::collections::HashSet;

use crate::prelude::*;

property!(
    /// `SelectedEntities` describes a list of selected entities.
    SelectedEntities(HashSet<Entity>)
);
