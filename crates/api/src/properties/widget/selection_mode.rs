use crate::{prelude::*, utils::SelectionMode as SelMod};

property!(
    /// Represents a selection mode.
    #[derive(Default)]
    SelectionMode(SelMod) : &str
);
