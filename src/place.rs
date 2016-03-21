use super::{CopyCell, Rect};

pub trait Place: Sized {
    fn rect(&self) -> &CopyCell<Rect>;
    fn position(self, x: i32, y: i32) -> Self {
        let mut rect = self.rect().get();
        rect.x = x;
        rect.y = y;
        self.rect().set(rect);

        self
    }

    fn size(self, width: u32, height: u32) -> Self {
        let mut rect = self.rect().get();
        rect.width = width;
        rect.height = height;
        self.rect().set(rect);

        self
    }
}
