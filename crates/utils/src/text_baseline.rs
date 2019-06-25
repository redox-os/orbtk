/// Text baseline being used when drawing text
pub enum TextBaseline {
    /// Text baseline is top of the em square
    Top,

    /// Text baseline is the hanging baseline.
    Hanging,

    /// Text baseline is the middle of the em square.
    Middle,

    /// Text baseline is the normal alphabetic baseline. (default)
    Alphabetic,

    /// Text baseline is the ideographic baseline
    Ideographic,

    /// Text baseline is the bottom of the bounding box.
    Bottom,
}

impl Default for TextBaseline {
    fn default() -> Self {
        TextBaseline::Alphabetic
    }
}
