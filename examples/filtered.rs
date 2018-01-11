extern crate orbtk;

use orbtk::{Window, TextBox, Rect, Label, Event};
use orbtk::traits::{EventFilter, Place, Text};

fn main() {
    let mut window = Window::new(Rect::new(100, 100, 420, 420), "Filtered Textbox");

    let label = Label::new();
    label.text("Field below will ignore all 'e' chars.")
         .position(10, 10).size(400, 16);
    window.add(&label);

    let text_field = TextBox::new();
    text_field.position(10, 32).size(400, 28).event_filter(|_widget, event, _focused, _redraw| {
        match event {
            Event::KeyPressed(key_event) => {
                if let Some(character) = key_event.character {
                    if character == 'e' {
                        return None
                    }
                }
            }
            _ => {}
        }

        Some(event)
    });
    text_field.text_offset(6, 6);
    window.add(&text_field);

    let label = Label::new();
    label.text("Field below will only accept numbers \n(as defined by unicode)")
         .position(10, 32+28+6).size(400, 32);       
    window.add(&label);

    let text_field = TextBox::new();
    text_field.position(10, 32+32+12+28).size(400, 28).event_filter(|_widget, event, _focused, _redraw| {
       match event {
            Event::KeyPressed (key_event) => {
                if let Some(character) = key_event.character {
                    if character.is_numeric() {
                        return Some(event)
                    }       
                } 
            },
            _ => {}
        }

        Some(event)
    });

    text_field.text_offset(6, 6);
    window.add(&text_field);

    window.exec();
}
