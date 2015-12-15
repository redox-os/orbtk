#![crate_type="lib"]

pub use sys::Window;

pub use button::Button;
pub use cell::{CopyCell, CloneCell};
pub use click::Click;
pub use color::Color;
pub use event::Event;
pub use label::Label;
pub use place::Place;
pub use point::Point;
pub use progress_bar::ProgressBar;
pub use rect::Rect;
pub use renderer::Renderer;
pub use text_box::TextBox;
pub use widget::Widget;

pub mod button;
pub mod cell;
pub mod click;
pub mod color;
pub mod event;
pub mod label;
pub mod place;
pub mod point;
pub mod progress_bar;
pub mod rect;
pub mod renderer;
pub mod text_box;
pub mod widget;

#[cfg(target_os = "redox")]
#[path="orbital/mod.rs"]
pub mod sys;

#[cfg(not(target_os = "redox"))]
#[path="sdl2/mod.rs"]
pub mod sys;

pub fn example() {
    let mut window = Window::new(Rect::new(100, 100, 400, 400), "OrbTK");

    let x = 20;
    let mut y = 20;

    let label = Label::new("Test Label")
        .position(x, y)
        .size(80, 16)
        .place(&mut window);

    y += 16 + 10;

    Button::new("Test Button")
        .position(x, y)
        .size(88, 16)
        .on_click(move |_button: &Button, point: Point| {
            let text = format!("{:?}", point);

            let mut rect = label.rect.get();
            rect.width = text.chars().count() * 8;
            label.rect.set(rect);

            label.text.set(text);
        })
        .place(&mut window);

    y += 16 + 10;

    ProgressBar::new(50)
        .position(x, y)
        .size(300, 16)
        .on_click(|progress_bar: &ProgressBar, point: Point| {
            progress_bar.value.set(point.x * 100 / progress_bar.rect.get().width as isize);
        })
        .place(&mut window);

    y += 16 + 10;

    Label::new("Test Input")
        .position(x, y)
        .size(80, 16)
        .place(&mut window);

    TextBox::new("")
        .position(x + 80 + 10, y)
        .size(210, 16)
        .place(&mut window);

    window.exec();
}
