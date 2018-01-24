extern crate orbtk;

use orbtk::{Action, Button, ComboBox, Grid, Image, Label, Menu, Orientation, Point, ProgressBar,
            Rect, Separator, StackLayout, TextBox, Widget, Window};
use orbtk::traits::{Click, Enter, Place, Text};

// commit: widgets (update, button, label, list, remove test_button, start documentation), widget tree example, css updae

fn main() {
    let mut window = Window::new(Rect::new(100, 100, 420, 768), "OrbTK");

    let stack_layout = StackLayout::new(Orientation::Vertical);
    stack_layout.position(10, 0).size(400, 768).spacing(10);

    let menu = Menu::new("Menu");
    menu.size(32, 16);
    stack_layout.add(menu.clone());

    let label = Label::new();
    label.size(400, 16).text("Test Label");
    stack_layout.add(label);

    let horizontal_stack_layout = StackLayout::new(Orientation::Horizontal);
    horizontal_stack_layout.size(400, 28).spacing(10);
    stack_layout.add(horizontal_stack_layout.clone());

    let text_box = TextBox::new();
    text_box.size(332, 28).text_offset(6, 6);
    horizontal_stack_layout.add(text_box.clone());

    let button = Button::new();
    button
        .text("Update")
        .on_click(move |_button: &Button, _point: Point| {
            text_box.emit_enter();
        });
    horizontal_stack_layout.add(button);

    let progress_label = Label::new();
    progress_label.text("Progress: 0%");
    stack_layout.add(progress_label.clone());

    let progress_bar = ProgressBar::new();
    progress_bar.size(400, 16).value(50).on_click(
        move |progress_bar: &ProgressBar, point: Point| {
            let progress = point.x * 100 / progress_bar.rect.get().width as i32;
            progress_label.text.set(format!("Progress: {}%", progress));
            progress_bar.value.set(progress);
        },
    );
    stack_layout.add(progress_bar);

    let combo_box = ComboBox::new();
    stack_layout.add(combo_box.clone());

    for i in 1..11 {
        combo_box.push(&format!("Entry {}", i));
    }

    let multi_line_label = Label::new();
    multi_line_label.text("Multi-Line Text").size(400, 16);
    stack_layout.add(multi_line_label);

    let multi_line_text_box = TextBox::new();
    multi_line_text_box.size(400, 130).text_offset(1, 1);
    stack_layout.add(multi_line_text_box);

    let offset_label = Label::new();
    offset_label
        .size(400, 120)
        .text("Test Offset")
        .text_offset(50, 50)
        .on_click(|label: &Label, _point: Point| {
            label.text("Clicked");
        });
    stack_layout.add(offset_label.clone());

    match Image::from_path("res/icon_small.png") {
        Ok(image) => {
            stack_layout.add(image.clone());
        }
        Err(err) => {
            let label = Label::new();
            label.size(400, 16).text(err);
            stack_layout.add(label.clone());
        }
    }

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

    let grid = Grid::new();
    grid.spacing(8, 8);

    let label = Label::new();
    label.size(32, 16).text("Grid");
    grid.insert(0, 0, &label);

    let label = Label::new();
    label.size(32, 16).text("Test");
    grid.insert(1, 0, &label);

    let label = Label::new();
    label.size(32, 16).text("With");
    grid.insert(2, 0, &label);

    let label = Label::new();
    label.size(48, 16).text("Resize");
    grid.insert(3, 0, &label);

    let mut i = 0;
    for row in 1..6 {
        for col in 0..5 {
            let cell = TextBox::new();
            let text = format!("{}: {}, {}", i, col, row);
            cell.size(text.len() as u32 * 8 + 2, 18)
                .text(text)
                .text_offset(1, 1);
            grid.insert(col, row, &cell);
            i += 1;
        }
    }

    stack_layout.add(grid.clone());
    grid.arrange(true);

    window.add(&stack_layout);

    println!("{:?}", window);

    window.exec();
}
