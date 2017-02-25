extern crate orbtk;

use orbtk::{ Window, List, Rect, Entry, Button };
use orbtk::traits::{ Place, Text, Click };

fn main() {
    let window = Window::new(Rect::new(100, 100, 420, 500), "OrbTK");

    let list = List::new();
    list.position(5, 5).size(400, 400);

    for i in 0..10 {
        let button = Button::new();
        button.text(format!("{}", i)).position(10, 10).size(50, 30);

        let entry = Entry::new(60);
        let list_clone = list.clone();
        entry.on_click(move |_, _| { println!("{}", i); });
        entry.add(&button);
        list.push(&entry);  
    }

    window.add(&list);

    window.exec();
}