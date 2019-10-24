use crate::{prelude::*, utils};

property!(
    /// `Brush` describes drawing brush of a visual element.
    #[derive(Default)]
    Brush(utils::Brush) : &str,
    String
);
