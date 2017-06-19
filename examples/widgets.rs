extern crate orbtk;

use orbtk::{Action, Button, Grid, Image, Label, Menu, Point, ProgressBar, Rect, Separator, TextBox, Window, ControlKnob};
use orbtk::traits::{Border, Click, Enter, Place, Text};

fn main() {
    let mut window = Window::new(Rect::new(100, 100, 420, 730), "OrbTK");

    let x = 10;
    let mut y = 0;

    let menu = Menu::new("Menu");
    menu.position(x, y)
        .size(32, 16);

    y += menu.rect.get().height as i32 + 10;

    let label = Label::new();
    label.position(x, y)
        .size(400, 16)
        .text("Test Label");
    window.add(&label);

    y += label.rect.get().height as i32 + 10;

    let text_box = TextBox::new();
    text_box.position(x, y)
        .size(332, 28)
        .text_offset(6, 6)
        .on_enter(move |text_box: &TextBox| {
            label.text.set(text_box.text.get());
        });
    window.add(&text_box);

    let button = Button::new();
    button.position(x + text_box.rect.get().width as i32 + 8, y)
        .size(48 + 12, text_box.rect.get().height)
        .text("Update")
        .text_offset(6, 6)
        .on_click(move |_button: &Button, _point: Point| {
            text_box.emit_enter();
        });
    window.add(&button);

    y += button.rect.get().height as i32 + 10;

    let progress_label = Label::new();
    progress_label.text("Progress: 0%")
        .position(x, y)
        .size(400, 16);
    window.add(&progress_label);

    y += progress_label.rect.get().height as i32 + 10;

    let progress_bar = ProgressBar::new();
    progress_bar.fg.set(orbtk::Color::rgb(0,255,0));  //set foreground color
    progress_bar.position(x, y)
        .size(400, 16)
        .on_click(move |progress_bar: &ProgressBar, point: Point| {
            let progress = point.x * 100 / progress_bar.rect.get().width as i32;
            progress_label.text.set(format!("Progress: {}%", progress));
            progress_bar.value.set(progress);
        });
    window.add(&progress_bar);

    y += progress_bar.rect.get().height as i32 + 10;






    let multi_line_label = Label::new();
    multi_line_label.text("Multi-Line Text")
        .position(x, y)
        .size(400, 16);
    window.add(&multi_line_label);

    y += multi_line_label.rect.get().height as i32 + 10;

    let multi_line_text_box = TextBox::new();
    multi_line_text_box.position(x, y)
        .size(400, 130)
        .text_offset(1, 1);
    window.add(&multi_line_text_box);

    y += multi_line_text_box.rect.get().height as i32 + 10;

    let offset_label = Label::new();
    offset_label.position(x, y)
        .size(400, 120)
        .border(true)
        .text("Test Offset")
        .text_offset(50, 50)
        .on_click(|label: &Label, _point: Point| {
            label.text("Clicked");
        });
    window.add(&offset_label);

    y += offset_label.rect.get().height as i32 + 10;

    match Image::from_path("res/icon_small.png") {
        Ok(image) => {
            image.position(x, y);
            window.add(&image);

            y += image.rect.get().height as i32 + 10;
        },
        Err(err) => {
            let label = Label::new();
            label.position(x, y)
                .size(400, 16)
                .text(err);
            window.add(&label);

            y += label.rect.get().height as i32 + 10;
        }
    }

    let volume_label = Label::new();
    volume_label.text("Volume: ").position(x+250, y-100).size(128, 16);
    volume_label.fg.set(orbtk::Color::rgb(0,0,255));  //set foreground color
    window.add(&volume_label);

    let volume = ControlKnob::new(); 
    let volume_label_clone = volume_label.clone();
    volume.border.set(true);
    volume.position(x+280, y-80)
        .size(40, 40)   //size.x must be equal to size.y so the circle is exactly inside the rect 
        .on_click(move |volume: &ControlKnob, point: Point| {
                      let progress = Point{ x: point.x ,
                                            y:point.y};
                      volume_label_clone.text.set(format!("Volume: {} {}", progress.x , progress.y));
                      volume.value.set(progress);
                  });
    window.add(&volume);


    let hidebutton = Button::new();
    let hidebutton_clone=hidebutton.clone();
    hidebutton.position(x + 120 + 8, y-100)
        .size(72 , 36)
        .text("Hide me")
        .text_offset(6, 6)
        .on_click(move |_button: &Button, _point: Point| {
            hidebutton_clone.visible.set(false);
        });
    window.add(&hidebutton);




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
    grid.position(x, y)
        .spacing(8, 8);

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
            cell.size(text.len() as u32 * 8 + 2, 18).text(text).text_offset(1, 1);
            grid.insert(col, row, &cell);
            i += 1;
        }
    }
    grid.arrange(true);

    window.add(&grid);

    // Add this last to put it on top
    window.add(&menu);

    window.exec();
}
