extern crate orbtk;

use orbtk::*;

fn main() {
    let mut window = Window::new(Rect::new(100, 100, 400, 400), "OrbTK");

    let label = Label::new("Test Label")
        .position(20, 20)
        .size(80, 16)
        .place(&mut window);

    ProgressBar::new(50)
        .position(20, 60)
        .size(200, 16)
        .on_click(|progress_bar: &ProgressBar, point: Point| {
            progress_bar.value.set(point.x * 100 / progress_bar.rect.get().width as isize);
        })
        .place(&mut window);

    Button::new("Test Button")
        .position(20, 100)
        .size(88, 16)
        .on_click(move |_button: &Button, point: Point| {
            let text = format!("{:?}", point);

            let mut rect = label.rect.get();
            rect.width = text.chars().count() * 8;
            label.rect.set(rect);

            label.text.set(text);
        })
        .place(&mut window);

    window.exec();
}
