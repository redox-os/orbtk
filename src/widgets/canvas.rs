use orbclient::Color;
use std::cell::{Cell, RefCell};
use std::sync::Arc;

use event::Event;
use point::Point;
use rect::Rect;
use renderer::Renderer;
use traits::{Click, Place};
use widgets::{Widget, WidgetCore};

pub struct Canvas {
    pub core: WidgetCore,
    data: RefCell<Vec<Color>>,
    click_callback: RefCell<Option<Arc<Fn(&Canvas, Point)>>>,
}

impl Canvas {
    pub fn new() -> Arc<Self> {
        Arc::new(Canvas {
            core: WidgetCore::new().bg(Color::rgb(255, 255, 255)).fg(Color::rgb(0, 0, 0)),
            data: RefCell::new(vec![]),
            click_callback: RefCell::new(None)
        })
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

    pub fn line(&self, start: Point, end: Point, color: Color) {
        let x0 = start.x;
        let y0 = start.y;
        let x1 = end.x;
        let y1 = end.y;

        let dx = (x1 - x0).abs();
        let dy = (y1 - y0).abs();

        let (mut x, mut y) = (x0, y0);

        let sx = if x0 > x1 { -1 } else { 1 };
        let sy = if y0 > y1 { -1 } else { 1 };

        if dx > dy {
            let mut err = dx as f32 / 2.0;
            while x != x1 {
                self.pixel(Point::new(x, y), color);
                err -= dy as f32;
                if err < 0.0 {
                    y += sy;
                    err += dx as f32;
                }
                x += sx;
            }
        } else {
            let mut err = dy as f32 / 2.0;
            while y != y1 {
                self.pixel(Point::new(x, y), color);
                err -= dx as f32;
                if err < 0.0 {
                    x += sx;
                    err += dy as f32;
                }
                y += sy;
            }
        }

        self.pixel(Point::new(x, y), color);
    }
}

impl Click for Canvas {
    fn emit_click(&self, point: Point) {
        if let Some(ref click_callback) = *self.click_callback.borrow() {
            click_callback(self, point);
        }
    }

    fn on_click<T: Fn(&Self, Point) + 'static>(&self, func: T) -> &Self {
        *self.click_callback.borrow_mut() = Some(Arc::new(func));
        self
    }
}

impl Place for Canvas {
    // override default implementation because data
    // must be initialized to proper width and height.
    fn size(&self, width: u32, height: u32) -> &Self {
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

impl Widget for Canvas {
    fn rect(&self) -> &Cell<Rect> {
        &self.core.rect
    }

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
