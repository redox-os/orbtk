extern crate orbtk;

use orbtk::*;

fn main() {
    let mut window = Window::new(Rect::new(100, 100, 400, 400), "OrbTK");

    let label = Label::new(Rect::new(20, 20, 80, 16), "Test Label")
        .place(&mut window);

    ProgressBar::new(Rect::new(20, 60, 200, 16), 50)
        .on_click(|progress_bar: &ProgressBar, point: Point| {
            progress_bar.value.set(point.x * 100 / progress_bar.rect.width as isize);
        })
        .place(&mut window);

    Button::new(Rect::new(20, 100, 88, 16), "Test Button")
        .on_click(move |_button: &Button, point: Point| {
            let mut rect = label.rect.get();
            rect.width = {
                let mut text = label.text.borrow_mut();
                *text = format!("{:?}", point);
                text.chars().count() * 8
            };
            label.rect.set(rect);
        })
        .place(&mut window);

    window.exec();
}
