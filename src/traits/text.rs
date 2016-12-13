pub trait Text {
    fn text<S: Into<String>>(&self, text: S) -> &Self;
    fn text_offset(&self, x: i32, y: i32) -> &Self;
}
