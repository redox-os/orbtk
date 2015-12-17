#![crate_type="lib"]
#![feature(str_char)]

pub use sys::Window;

pub use button::Button;
pub use cell::{CopyCell, CloneCell};
pub use click::Click;
pub use color::Color;
pub use enter::Enter;
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
pub mod enter;
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
    let mut window = Window::new(Rect::new(100, 100, 420, 420), "OrbTK");

    let x = 10;
    let mut y = 10;

    let label = Label::new()
        .text("Test Label")
        .position(x, y)
        .size(400, 32)
        .place(&mut window);

    y += label.rect.get().height as isize + 10;

    let text_box = TextBox::new()
        .position(x, y)
        .size(302, 32)
        .place(&mut window);

    let text_button = Button::new()
        .text("Test Button")
        .position(x + text_box.rect.get().width as isize + 10, y)
        .size(88, 32)
        .on_click(move |_button: &Button, _point: Point| {
            label.text.set(format!("Input: {}", text_box.text.get()));
        })
        .place(&mut window);

    y += text_button.rect.get().height as isize + 10;

    let progress_label = Label::new()
        .text("Progress: 0%")
        .position(x, y)
        .size(400, 16)
        .place(&mut window);

    y += progress_label.rect.get().height as isize + 10;

    let progress_bar = ProgressBar::new()
        .position(x, y)
        .size(400, 16)
        .on_click(move |progress_bar: &ProgressBar, point: Point| {
            let progress = point.x * 100 / progress_bar.rect.get().width as isize;
            progress_label.text.set(format!("Progress: {}%", progress));
            progress_bar.value.set(progress);
        })
        .place(&mut window);

    y += progress_bar.rect.get().height as isize + 10;

    let multi_line_label = Label::new()
        .text("Multi-Line Text")
        .position(x, y)
        .size(400, 16)
        .place(&mut window);

    y += multi_line_label.rect.get().height as isize + 10;

    TextBox::new()
        .position(x, y)
        .size(400, 128)
        .place(&mut window);

    window.exec();
}
