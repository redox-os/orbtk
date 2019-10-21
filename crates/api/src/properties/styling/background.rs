use crate::{prelude::*, utils::*};

property!(
    /// `Background` describes the background brush of a visual element.
    #[derive(Default)]
    Background(Brush) : &str,
    String
);
