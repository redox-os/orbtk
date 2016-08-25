use super::{Widget, Window};

use std::sync::Arc;

pub trait Placeable: Sized + Widget {
    fn place(self, window: &Window) -> Arc<Self> {
        let arc = Arc::new(self);
        let mut widgets = window.widgets.borrow_mut();

        widgets.push(arc.clone());

        arc
    }

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
