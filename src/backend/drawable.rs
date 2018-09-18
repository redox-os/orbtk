use {Backend, Rect, Selector};

pub struct Drawable {
    draw_fn: Box<Fn(&Rect, &Selector, &mut Box<Backend>)>,
}

impl Drawable {
    pub fn new(draw_fn: Box<Fn(&Rect, &Selector, &mut Box<Backend>)>) -> Self {
        Drawable { draw_fn }
    }

    pub fn draw(&self, bounds: &Rect, selector: &Selector, renderer: &mut Box<Backend>) {
        (self.draw_fn)(bounds, selector, renderer)
    }
}
