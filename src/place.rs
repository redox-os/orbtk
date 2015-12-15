pub trait Place {
    fn position(self, x: isize, y: isize) -> Self;
    fn size(self, width: usize, height: usize) -> Self;
}
