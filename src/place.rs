pub trait Place {
    fn position(mut self, x: isize, y: isize) -> Self;
    fn size(mut self, width: usize, height: usize) -> Self;
}
