use { InnerWindow, Window, List, Entry, Label };
use traits::{ Place, Text, Click };

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
    fn scan<P: AsRef<Path>>(path: P) -> io::Result<Vec<Result<Self, String>>> {
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



pub struct FileDialog {
    pub title: String,
    pub path: PathBuf,
    curpath: Rc<RefCell<PathBuf>>,
    pub hidden: bool,
}

impl FileDialog {
    pub fn new() -> Self {
        use std;
        FileDialog {
            title: "File Dialog".to_string(),
            curpath: Rc::new(RefCell::new(std::env::current_dir().unwrap())),
            path: PathBuf::from("."),
            hidden: false,
        }
    }

    pub fn exec(&self) -> Option<PathBuf> {
        let path_opt = Rc::new(RefCell::new(
            Some(self.path.clone())
        ));

        let w = 644;
        let h = 484;

        let mut orb_window = Some(InnerWindow::new(-1, -1, w, h, &self.title).unwrap());

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

            //TODO: let the user specify the path with a textbox and change upon pressing enter - error -> popup window
            {
                //add label with current dir path to UI
                let curdir = Label::new();
                curdir.position(2, 2).size(w - 8, 20).text_offset(2, 2);
                let entry = Entry::new(24);
                let curpath = self.curpath.clone();
                curdir.text(curpath.borrow().to_str().unwrap().to_string());
                entry.add(&curdir);
                list.push(&entry);
                //whitespace label
                let whitespace = Label::new();
                whitespace.position(2, 2).size(w - 8, 20).text_offset(2, 2);
                let entry = Entry::new(24);
                whitespace.text("");
                entry.add(&whitespace);
                list.push(&entry);
            }


            match FolderItem::scan(&path) {
                Ok(items) => for item_res in items {
                    match item_res {
                        Ok(item) => if self.hidden || ! item.name.starts_with(".") || item.name == ".." {
                            let mut name = item.name.clone();
                            if item.dir {
                                name.push('/');
                            }

                            //TODO: currently a new window is created for content change
                            //      -> change content from current window without constructing a new one(if possible)
                            // might lead to performance increase
                            let entry = Entry::new(24);

                            let label = Label::new();
                            label.position(2, 2).size(w - 8, 20).text_offset(2, 2);
                            //label.bg.set(Color::rgb(255, 255, 255));
                            label.text(name);
                            entry.add(&label);

                            let window = window.deref() as *const Window;
                            let path_opt = path_opt.clone();

                            //copy the Rc
                            let curpath = self.curpath.clone();

                            entry.on_click(move |_, _| {
                                //change the current path
                                let buf_path = curpath.borrow().join(&item.path).canonicalize().unwrap();
                                *curpath.borrow_mut() = buf_path;

                                *path_opt.borrow_mut() = Some(item.path.clone());
                                unsafe { (*window).close(); }
                            });


                            list.push(&entry);
                        },
                        Err(err) => {
                            let entry = Entry::new(24);

                            let label = Label::new();
                            label.position(2, 2).size(w - 8, 20).text_offset(2, 2);
                            //label.bg.set(Color::rgb(242, 222, 222));
                            label.text(err);
                            entry.add(&label);

                            list.push(&entry);
                        }
                    }
                },
                Err(err) => {
                    //add label with error text
                    let entry = Entry::new(24);

                    let label = Label::new();
                    label.position(2, 2).size(w - 8, 20).text_offset(2, 2);
                    //label.bg.set(Color::rgb(242, 222, 222));
                    label.text(format!("{}", err));
                    entry.add(&label);

                    list.push(&entry);
                }
            }


            window.add(&list);

            window.exec();

            orb_window = Some(window.into_inner());
        }
    }
}
