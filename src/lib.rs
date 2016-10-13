#![crate_name="orbtk"]
#![crate_type="lib"]
#![deny(warnings)]

pub use button::Button;
pub use cell::CloneCell;
pub use canvas::Canvas;
pub use color::Color;
pub use event::Event;
pub use label::Label;
pub use menu::{Menu, Action};
pub use place::Placeable;
pub use point::Point;
pub use progress_bar::ProgressBar;
pub use rect::Rect;
pub use renderer::Renderer;
pub use text_box::TextBox;
pub use widget::{Widget, WidgetCore};
pub use window::Window;

pub mod button;
pub mod canvas;
pub mod cell;
pub mod color;
pub mod event;
pub mod label;
pub mod menu;
pub mod place;
pub mod point;
pub mod progress_bar;
pub mod rect;
pub mod renderer;
pub mod text_box;
pub mod widget;
pub mod window;

use callback::{Click, Enter};
pub mod callback;

pub fn example() {
    let window = Window::new(Rect::new(100, 100, 420, 420), "OrbTK");

    let x = 10;
    let mut y = 0;

    let mut menu = Menu::new("Menu")
        .position(x, y)
        .size(32, 16);

    y += menu.core.rect.get().height as i32 + 10;

    let label = Label::new()
        .position(x, y)
        .size(400, 16)
        .text("Test Label")
        .place(&window);

    y += label.core.rect.get().height as i32 + 10;

    let text_box = TextBox::new()
        .position(x, y)
        .size(342, 16)
        .on_enter(move |text_box: &TextBox| {
            label.text.set(text_box.text.get());
        })
        .place(&window);

    let button = Button::new()
        .position(x + text_box.core.rect.get().width as i32 + 10, y)
        .size(48, text_box.core.rect.get().height)
        .text("Update")
        .on_click(move |_button: &Button, _point: Point| {
            text_box.emit_enter();
        })
        .place(&window);

    y += button.core.rect.get().height as i32 + 10;

    let progress_label = Label::new()
        .text("Progress: 0%")
        .position(x, y)
        .size(400, 16)
        .place(&window);

    y += progress_label.core.rect.get().height as i32 + 10;

    let progress_bar = ProgressBar::new()
        .position(x, y)
        .size(400, 16)
        .on_click(move |progress_bar: &ProgressBar, point: Point| {
            let progress = point.x * 100 / progress_bar.core.rect.get().width as i32;
            progress_label.text.set(format!("Progress: {}%", progress));
            progress_bar.value.set(progress);
        })
        .place(&window);

    y += progress_bar.core.rect.get().height as i32 + 10;

    let multi_line_label = Label::new()
        .text("Multi-Line Text")
        .position(x, y)
        .size(400, 16)
        .place(&window);

    y += multi_line_label.core.rect.get().height as i32 + 10;

    let multi_line_text_box = TextBox::new()
        .position(x, y)
        .size(400, 128)
        .place(&window);

    y += multi_line_text_box.core.rect.get().height as i32 + 10;

    let offset_label = Label::new()
        .position(x, y)
        .size(400, 256)
        .text("Test Offset")
        .text_offset(50, 50)
        .place(&window);

    {
        let offset_label_clone = offset_label.clone();
        menu.add_action(Action::new("Label One")
            .on_click(move |_action: &Action, _point: Point| {
                offset_label_clone.text.set("One".to_owned());
            }));
    }

    {
        let offset_label_clone = offset_label.clone();
        menu.add_action(Action::new("Label Two")
            .on_click(move |_action: &Action, _point: Point| {
                offset_label_clone.text.set("Two".to_owned());
            }));
    }

    menu.add_separator();

    {
        let offset_label_clone = offset_label.clone();
        menu.add_action(Action::new("Reset Label")
            .on_click(move |_action: &Action, _point: Point| {
                offset_label_clone.text.set("Text Offset".to_owned());
            }));
    }

    // TODO: Don't require this to be placed last to be drawn last
    menu.place(&window);

    window.exec();
}
