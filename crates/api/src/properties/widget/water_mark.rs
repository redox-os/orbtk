use crate::{prelude::*, utils::String16};

property!(
    /// `WaterMark` describes a placeholder text.
    #[derive(Default)]
    WaterMark(String16) : &str,
    String
);
