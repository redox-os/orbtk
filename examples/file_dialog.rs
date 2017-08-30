extern crate orbtk;

use orbtk::{ Color, Window, List, Rect, Entry, Label };
use orbtk::traits::{ Place, Text, Click };

use std::{fs, io, path};
use std::cell::RefCell;
use std::ops::Deref;
use std::rc::Rc;

fn folder_items<P: AsRef<path::Path>>(path: P) -> io::Result<Vec<Result<String, String>>> {
    let mut items = vec![];

    if path.as_ref().parent().is_some() {
        items.push(Ok("..".to_string()));
    }

    for entry_res in fs::read_dir(path)? {
        let item = match entry_res {
            Ok(entry) => match entry.file_name().into_string() {
                Ok(name) => match entry.file_type() {
                    Ok(file_type) => if file_type.is_dir() {
                        Ok(format!("{}/", name))
                    } else {
                        Ok(name)
                    },
                    Err(err) => Err(format!("{}", err))
                },
                Err(os_str) => Err(format!("Invalid filename: {:?}", os_str))
            },
            Err(err) => Err(format!("{}", err))
        };

        items.push(item);
    }

    items.sort();

    Ok(items)
}

fn main() {
    let path_opt = Rc::new(RefCell::new(
        Some(path::PathBuf::from("."))
    ));

    loop {
        let path = match path_opt.borrow_mut().take() {
            Some(path) => path,
            None => return
        };

        let w = 640;
        let h = 480;

        let mut window = Box::new(Window::new(Rect::new(-1, -1, w, h), "File Dialog"));

        let list = List::new();
        list.position(2, 2).size(w - 4, h - 4);

        match folder_items(&path) {
            Ok(items) => for item in items {
                let entry = Entry::new(24);

                let label = Label::new();
                label.position(2, 2).size(w - 8, 20).text_offset(2, 2);
                label.bg.set(Color::rgb(255, 255, 255));
                entry.add(&label);

                match item {
                    Ok(name) => {
                        {
                            let window = window.deref() as *const Window;
                            let path_opt = path_opt.clone();
                            let path = {
                                let mut p = path.clone();
                                p.push(&name);
                                p
                            };
                            let name = name.clone();
                            entry.on_click(move |_, _| {
                                println!("{}", name);
                                *path_opt.borrow_mut() = Some(path.clone());
                                unsafe { (*window).close(); }
                            });
                        }

                        label.text(name);
                    },
                    Err(err) => {
                        label.text(err);
                    }
                }

                list.push(&entry);
            },
            Err(err) => {
                let entry = Entry::new(24);

                let label = Label::new();
                label.position(2, 2).size(w - 8, 20).text_offset(2, 2);
                label.bg.set(Color::rgb(242, 222, 222));
                entry.add(&label);

                label.text(format!("{}", err));

                list.push(&entry);
            }
        }

        window.add(&list);

        window.exec();
    }
}
