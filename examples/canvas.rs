extern crate orbtk;

use orbtk::{Window, Canvas, Rect, Placeable, Point};
use orbtk::callback::Click;

use std::rc::Rc;
use std::cell::RefCell;

fn main() {
    let window = Window::new(Rect::new(100, 100, 420, 420), "Canvas");

    let click_pos: Rc<RefCell<Option<Point>>>= Rc::new(RefCell::new(None));

    let canvas = Canvas::new()
        .position(10, 10)
        .size(400, 400)
        .on_click(move |canvas: &Canvas, point: Point| {
            let click = click_pos.clone();
            {
                let mut prev_opt = click.borrow_mut();
                
                if let Some(prev_position) = *prev_opt {
                    canvas.line(prev_position, point, orbtk::Color::black());
                    *prev_opt = Some(point);
                } else {
                    *prev_opt = Some(point);
                }
            }
        })
        .place(&window);

    window.exec();
}
