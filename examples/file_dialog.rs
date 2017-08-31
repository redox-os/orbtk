extern crate orbclient;
extern crate orbtk;

use orbtk::{ Color, Window, List, Entry, Label };
use orbtk::traits::{ Place, Text, Click };

use std::{fs, io};
use std::cell::RefCell;
use std::cmp::Ordering;
use std::ops::Deref;
use std::path::{Path, PathBuf};
use std::rc::Rc;

#[derive(Clone, Debug, Eq, PartialEq)]
struct FolderItem {
    path: PathBuf,
    name: String,
    dir: bool,
}

impl Ord for FolderItem {
    fn cmp(&self, other: &Self) -> Ordering {
        if self.dir && ! other.dir {
            Ordering::Less
        } else if ! self.dir && other.dir {
            Ordering::Greater
        } else {
            self.name.cmp(&other.name)
        }
    }
}

impl PartialOrd for FolderItem {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl FolderItem {
    pub fn scan<P: AsRef<Path>>(path: P) -> io::Result<Vec<Result<Self, String>>> {
        let canon = path.as_ref().canonicalize()?;

        let mut items = vec![];

        if let Some(parent) = canon.parent() {
            items.push(Ok(FolderItem {
                path: parent.to_owned(),
                name: "..".to_string(),
                dir: true,
            }));
        }

        for entry_res in fs::read_dir(&canon)? {
            let item = match entry_res {
                Ok(entry) => match entry.file_name().into_string() {
                    Ok(name) => match entry.file_type() {
                        Ok(file_type) => Ok(FolderItem {
                            path: entry.path(),
                            name: name,
                            dir: file_type.is_dir(),
                        }),
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
}

fn file_dialog<P: AsRef<Path>>(path: P) -> Option<PathBuf> {
    let path_opt = Rc::new(RefCell::new(
        Some(path.as_ref().to_owned())
    ));

    let w = 640;
    let h = 480;

    let mut orb_window = Some(orbclient::Window::new(-1, -1, w, h, "File Dialog").unwrap());

    loop {
        let path = match path_opt.borrow_mut().take() {
            Some(path) => if ! path.is_dir() {
                return Some(path);
            } else {
                path
            },
            None => return None
        };

        let mut window = Box::new(Window::from_inner(orb_window.take().unwrap()));

        let list = List::new();
        list.position(2, 2).size(w - 4, h - 4);

        match FolderItem::scan(&path) {
            Ok(items) => for item_res in items {
                let entry = Entry::new(24);

                let label = Label::new();
                label.position(2, 2).size(w - 8, 20).text_offset(2, 2);
                label.bg.set(Color::rgb(255, 255, 255));
                entry.add(&label);

                match item_res {
                    Ok(item) => {
                        let mut name = item.name.clone();
                        if item.dir {
                            name.push('/');
                        }
                        label.text(name);

                        let window = window.deref() as *const Window;
                        let path_opt = path_opt.clone();
                        entry.on_click(move |_, _| {
                            *path_opt.borrow_mut() = Some(item.path.clone());
                            unsafe { (*window).close(); }
                        });
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

        orb_window = Some(window.into_inner());
    }
}

fn main() {
    println!("{:?}", file_dialog("."));
}
