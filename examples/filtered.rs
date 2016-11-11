extern crate orbtk;

use orbtk::{Window, TextBox, Rect, Placeable, Label};
use orbtk::callback::EventFilter;
use orbtk::Event;

use std::rc::Rc;
use std::cell::RefCell;

fn main() {
    let window = Window::new(Rect::new(100, 100, 420, 420), "Filtered Textbox");

    let label = Label::new().text("Field below will ignore all 'e' chars.")
                            .position(10, 10).size(400, 16).place(&window);

    let text_field = TextBox::new().position(10, 32).size(400, 16).event_filter(|widget, event, focused, redraw| {
        match event {
            Event::Text { c: 'e' } => {
                None
            }
            _ => {
                Some(event)
            }
        }
    }).place(&window);

    let label = Label::new().text("Field below will only accept numbers \n(as defined by unicode)")
                            .position(10, 32+16+6).size(400, 32).place(&window);
                            
    let text_field = TextBox::new().position(10, 32+32+12+16).size(400, 16).event_filter(|widget, event, focused, redraw| {
        match event {
            Event::Text { c } => {
                if c.is_numeric() {
                    Some(event)
                } else {
                    None
                }
            }
            _ => {
                Some(event)
            }
        }
    }).place(&window);

    window.exec();
}