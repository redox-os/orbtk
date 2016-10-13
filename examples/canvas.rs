extern crate orbtk;

use orbtk::{Window, Canvas, Rect, Place, WidgetPlace, Point, ProgressBar};
use orbtk::callback::Click;

fn main() {
    let window = Window::new(Rect::new(100, 100, 420, 420), "Canvas");

    let canvas = Canvas::new()
        .position(10, 10)
        .size(400, 400)
        .on_click(move |canvas: &Canvas, point: Point| {
            canvas.pixel(point, orbtk::Color::black());
        })
        .place(&window);

    window.exec();
}