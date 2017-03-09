extern crate orbtk;

use orbtk::{Color, Window, Image, Rect, Point, Renderer};
use orbtk::traits::{Click, Place};

use std::rc::Rc;
use std::cell::RefCell;

fn main() {
    let mut window = Window::new(Rect::new(100, 100, 420, 420), "Canvas");

    let click_pos: Rc<RefCell<Option<Point>>>= Rc::new(RefCell::new(None));

    let canvas = Image::from_color(400, 400, Color::rgb(255, 255, 255));
    canvas.position(10, 10)
        .on_click(move |canvas: &Image, point: Point| {
            let click = click_pos.clone();
            {
                let mut prev_opt = click.borrow_mut();

                if let Some(prev_position) = *prev_opt {
                    let mut image = canvas.image.borrow_mut();
                    image.line(prev_position.x, prev_position.y, point.x, point.y, orbtk::Color::rgb(0, 0, 0));
                    *prev_opt = Some(point);
                } else {
                    *prev_opt = Some(point);
                }
            }
        });
    window.add(&canvas);

    window.exec();
}
