use crate::{prelude::*, utils::*};

property!(
    /// `Foreground` describes the foreground brush of a visual element.
    #[derive(Default)]
    Foreground(Brush) : &str,
    String
);
