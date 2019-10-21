use std::collections::HashSet;

use crate::prelude::*;

property!(
    /// `SelectedEntities` describes a list of selected entities.
    #[derive(Default)]
    SelectedEntities(HashSet<Entity>)
);
