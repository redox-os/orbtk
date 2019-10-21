use crate::{prelude::*, utils::String16};

property!(
    /// `Text` describes the text of a widget.
    #[derive(Default)]
    Text(String16) : &str,
    String
);
