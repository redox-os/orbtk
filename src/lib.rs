#![crate_type="lib"]
#![feature(str_char)]

pub use sys::Window;

pub use button::Button;
pub use cell::{CopyCell, CloneCell};
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

use callback::{Click, Enter};
pub mod callback;

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
        .position(x, y)
        .size(400, 16)
        .text("Test Label")
        .place(&mut window);

    y += label.rect.get().height as i32 + 10;

    let text_box = TextBox::new()
        .position(x, y)
        .size(342, 16)
        .on_enter(move |text_box: &TextBox| {
            label.text.set(text_box.text.get());
        })
        .place(&mut window);

    let button = Button::new()
        .position(x + text_box.rect.get().width as i32 + 10, y)
        .size(48, text_box.rect.get().height)
        .text("Update")
        .on_click(move |_button: &Button, _point: Point| {
            text_box.emit_enter();
        })
        .place(&mut window);

    y += button.rect.get().height as i32 + 10;

    let progress_label = Label::new()
        .text("Progress: 0%")
        .position(x, y)
        .size(400, 16)
        .place(&mut window);

    y += progress_label.rect.get().height as i32 + 10;

    let progress_bar = ProgressBar::new()
        .position(x, y)
        .size(400, 16)
        .on_click(move |progress_bar: &ProgressBar, point: Point| {
            let progress = point.x * 100 / progress_bar.rect.get().width as i32;
            progress_label.text.set(format!("Progress: {}%", progress));
            progress_bar.value.set(progress);
        })
        .place(&mut window);

    y += progress_bar.rect.get().height as i32 + 10;

    let multi_line_label = Label::new()
        .text("Multi-Line Text")
        .position(x, y)
        .size(400, 16)
        .place(&mut window);

    y += multi_line_label.rect.get().height as i32 + 10;

    let multi_line_text_box = TextBox::new()
        .position(x, y)
        .size(400, 128)
        .place(&mut window);

    y += multi_line_text_box.rect.get().height as i32 + 10;

    Label::new()
        .position(x, y)
        .size(400, 256)
        .text("Test Offet")
        .text_offset(50, 50)
        .place(&mut window);

    window.exec();
}
