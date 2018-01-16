use widgets::{Widget, VerticalPlacement, HorizontalPlacement};

pub trait Place: Sized + Widget {
    fn position(&self, x: i32, y: i32) -> &Self {
        let mut position = self.local_position().get();
        let mut rect = self.rect().get();
        position.x = x;
        position.y = y;
        rect.x = x;
        rect.y = y;
        self.local_position().set(position);
        self.rect().set(rect);
        self.arrange();
        self
    }

    fn size(&self, width: u32, height: u32) -> &Self {
        let mut rect = self.rect().get();
        rect.width = width;
        rect.height = height;
        self.rect().set(rect);
        self.arrange();
        self
    }

     fn placement(&self, vertical_placement: VerticalPlacement, horizontal_placement: HorizontalPlacement) -> &Self {
        self.vertical_placement().set(vertical_placement);
        self.horizontal_placement().set(horizontal_placement);
        self.arrange();
        self
    }
}
