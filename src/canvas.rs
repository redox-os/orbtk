use super::{Color, Event, Place, Point, Rect, Renderer, Widget, WidgetCore, WidgetPlace};
use super::callback::{Click};

use std::cell::{Cell, RefCell};
use std::sync::Arc;

pub struct Canvas {
    pub core: WidgetCore,
    data: RefCell<Vec<Color>>,
    click_callback: Option<Arc<Fn(&Canvas, Point)>>,
}

impl Canvas {
    pub fn new() -> Self {
        Canvas {
            core: WidgetCore::new(Color::white(), Color::black()),
            data: RefCell::new(vec![]),
            click_callback: None
        }
    }

    pub fn clear(&self, color: Color) {
        let rect = self.rect().get();
        
        *(self.data.borrow_mut()) = vec![color; (rect.width*rect.height) as usize];
    }

    // TODO(ca1ek): implement the commented out functions,
    // I would keep them uncommented, but the lint 
    // level disallows this.
    /*pub fn char(&self, pos: Point, c: char, color: Color) {
        unimplemented!()
    }*/

    /*pub fn draw_rect(&self, rect: Rect, color: Color) {
        unimplemented!()
    }*/

    pub fn pixel(&self, point: Point, color: Color) {
        let rect = self.rect().get();
        if let Some(color_ref) = self.data.borrow_mut().get_mut((point.y*rect.width as i32 + point.x) as usize) {
            *color_ref = color;
        } 
    }

    /*pub fn line(&self, start: Point, end: Point, color: Color) {
        unimplemented!()
    }*/
}

impl Click for Canvas {
    fn emit_click(&self, point: Point) {
        if let Some(ref click_callback) = self.click_callback {
            click_callback(self, point);
        }
    }

    fn on_click<T: Fn(&Self, Point) + 'static>(mut self, func: T) -> Self {
        self.click_callback = Some(Arc::new(func));
        self
    }
}

impl Place for Canvas {
    fn rect(&self) -> &Cell<Rect> {
        &self.core.rect
    }

    // override default implementation because data 
    // must be initialized to proper width and height.
    fn size(self, width: u32, height: u32) -> Self {
        *(self.data.borrow_mut()) = vec![self.core.bg; (width*height) as usize];

        // code below should be the same as default 
        // Place::size() implementation
        let mut rect = self.rect().get();
        rect.width = width;
        rect.height = height;
        self.rect().set(rect);

        self
    }
}

impl WidgetPlace for Canvas {}

impl Widget for Canvas {
    fn draw(&self, renderer: &mut Renderer, _focused: bool) {
        let rect = self.core.rect.get();
        renderer.rect(rect, self.core.bg);

        let data = self.data.borrow();

        for x in 0..rect.width {
            for y in 0..rect.height {
                if let Some(pixel) = data.get((y*rect.width + x) as usize) {
                    renderer.pixel(Point::new(rect.x + x as i32, rect.y + y as i32), pixel.clone());
                }
            }
        }
    }

    fn event(&self, event: Event, focused: bool, redraw: &mut bool) -> bool {
        match event {
            Event::Mouse { point, left_button, .. } => {
                let rect = self.core.rect.get();

                // if mouse is in canvas and LMB is pressed
                if rect.contains(point) && left_button {
                    // point is a position inside the window,
                    // subtracting the position of the canvas
                    // makes it a position inside the canvas.
                    let click_point: Point = point - rect.point();
                    self.emit_click(click_point);
                    *redraw = true;
                } 
            }
            _ => (),
        }

        focused
    }
}