extern crate orbtk;

use orbtk::{Action, Button, Label, Menu, Point, ProgressBar, Rect, Separator, TextBox, Window};
use orbtk::traits::{Click, Enter, Place, Text};

fn main() {
    let window = Window::new(Rect::new(100, 100, 420, 420), "OrbTK");

    let x = 10;
    let mut y = 0;

    let menu = Menu::new("Menu");
    menu.position(x, y)
        .size(32, 16);

    y += menu.core.rect.get().height as i32 + 10;

    let label = Label::new();
    label.position(x, y)
        .size(400, 16)
        .text("Test Label");
    window.add(&label);

    y += label.core.rect.get().height as i32 + 10;

    let text_box = TextBox::new();
    text_box.position(x, y)
        .size(342, 16)
        .on_enter(move |text_box: &TextBox| {
            label.text.set(text_box.text.get());
        });
    window.add(&text_box);

    let button = Button::new();
    button.position(x + text_box.core.rect.get().width as i32 + 10 - 8, y - 4)
        .size(48 + 8, text_box.core.rect.get().height + 8)
        .text("Update")
        .text_offset(4, 4)
        .on_click(move |_button: &Button, _point: Point| {
            text_box.emit_enter();
        });
    window.add(&button);

    y += button.core.rect.get().height as i32 + 10;

    let progress_label = Label::new();
    progress_label.text("Progress: 0%")
        .position(x, y)
        .size(400, 16);
    window.add(&progress_label);

    y += progress_label.core.rect.get().height as i32 + 10;

    let progress_bar = ProgressBar::new();
    progress_bar.position(x, y)
        .size(400, 16)
        .on_click(move |progress_bar: &ProgressBar, point: Point| {
            let progress = point.x * 100 / progress_bar.core.rect.get().width as i32;
            progress_label.text.set(format!("Progress: {}%", progress));
            progress_bar.value.set(progress);
        });
    window.add(&progress_bar);

    y += progress_bar.core.rect.get().height as i32 + 10;

    let multi_line_label = Label::new();
    multi_line_label.text("Multi-Line Text")
        .position(x, y)
        .size(400, 16);
    window.add(&multi_line_label);

    y += multi_line_label.core.rect.get().height as i32 + 10;

    let multi_line_text_box = TextBox::new();
    multi_line_text_box.position(x, y)
        .size(400, 128);
    window.add(&multi_line_text_box);

    y += multi_line_text_box.core.rect.get().height as i32 + 10;

    let offset_label = Label::new();
    offset_label.position(x, y)
        .size(400, 256)
        .text("Test Offset")
        .text_offset(50, 50);
    window.add(&offset_label);

    {
        let action = Action::new("Label One");
        let offset_label_clone = offset_label.clone();
        action.on_click(move |_action: &Action, _point: Point| {
            offset_label_clone.text.set("One".to_owned());
        });
        menu.add(&action);
    }

    {
        let action = Action::new("Label Two");
        let offset_label_clone = offset_label.clone();
        action.on_click(move |_action: &Action, _point: Point| {
            offset_label_clone.text.set("Two".to_owned());
        });
        menu.add(&action);
    }

    menu.add(&Separator::new());

    {
        let action = Action::new("Reset Label");
        let offset_label_clone = offset_label.clone();
        action.on_click(move |_action: &Action, _point: Point| {
            offset_label_clone.text.set("Text Offset".to_owned());
        });
        menu.add(&action);
    }

    // TODO: Don't require this to be placed last to be drawn last
    window.add(&menu);

    window.exec();
}
