use crate::{prelude::*, utils};

property!(
    /// `BorderBrush` describes the border brush.
    #[derive(Default)]
    BorderBrush(utils::Brush) : &str,
    String
);
