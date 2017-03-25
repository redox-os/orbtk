use orbclient::{Renderer, Color};
use orbimage;
use std::cell::{ Cell, RefCell };
use std::sync::Arc;

use cell::CheckSet;
use event::Event;
use point::Point;
use rect::Rect;
use theme::{ ITEM_BACKGROUND, WINDOW_BACKGROUND, ITEM_SELECTION };
use traits::{ Click, Place };
use widgets::Widget;
use std::ops::Index;

/// An entry in a list
/// Each entry stores widgets within. 
pub struct Entry {
    pub height: Cell<u32>,
    click_callback: RefCell<Option<Arc<Fn(&Entry, Point)>>>,
    widgets: RefCell<Vec<Arc<Widget>>>,
    pub highlight: Cell<Color>,
    highlighted: Cell<bool>,
}

impl Entry {
    pub fn new(h: u32) -> Arc<Self> {
        Arc::new(Entry {
            height: Cell::new(h),
            click_callback: RefCell::new(None),
            widgets: RefCell::new(vec![]),
            highlight: Cell::new(ITEM_SELECTION),
            highlighted: Cell::new(false),
        })
    }

    /// Adds a widget to the entry
    pub fn add<T: Widget>(&self, widget: &Arc<T>) {
        let mut widgets = self.widgets.borrow_mut();
        widgets.push(widget.clone());
    }

    fn widgets(&self) -> &RefCell<Vec<Arc<Widget>>> {
        &self.widgets
    }
}

impl Click for Entry {
    fn emit_click(&self, point: Point) {
        if let Some(ref click_callback) = *self.click_callback.borrow() {
            click_callback(self, point);
        }
    }

    fn on_click<T: Fn(&Self, Point) + 'static>(&self, func: T) -> &Self {
        *self.click_callback.borrow_mut() = Some(Arc::new(func));
        self
    }
}

pub struct List {
    pub rect: Cell<Rect>,
    v_scroll: Cell<i32>,
    current_height: Cell<u32>,
    entries: RefCell<Vec<Arc<Entry>>>,
    pressed: Cell<bool>,
    selected: Cell<Option<u32>>,
}

impl List {
    pub fn new() -> Arc<Self> {
        Arc::new(List {
            rect: Cell::new(Rect::default()),
            v_scroll: Cell::new(0),
            current_height: Cell::new(0),
            entries: RefCell::new(vec![]),
            pressed: Cell::new(false),
            selected: Cell::new(None),
        })
    }

    pub fn push(&self, entry: &Arc<Entry>) {
        let h = entry.height.get();
        self.entries.borrow_mut().push(entry.clone());
        self.current_height.set(self.current_height.get() + h);
    }

    // Given absolute coordinates, returns the list entry index
    // drawn at that point.
    fn get_entry_index(&self, p: Point) -> Option<u32> {
        if self.rect.get().contains(p) {
            let mut current_y = 0;
            let x = self.rect.get().x;
            let y = self.rect.get().y;
            let width = self.rect.get().width;
            let scroll = self.v_scroll.get();

            for (i, entry) in self.entries.borrow().iter().enumerate() {
                if Rect::new(x, y+current_y-scroll, width, entry.height.get()).contains(p) {
                    return Some(i as u32)
                }
                current_y += entry.height.get() as i32
            }
        }

        None
    }

    pub fn scroll(&self, y: i32) {
        let mut set_to = self.v_scroll.get() + y;
        if set_to < 0 {
            set_to = 0;
        } else if self.rect.get().height as i32 + set_to > self.current_height.get() as i32 {
            set_to = self.v_scroll.get() as i32;
        }
        self.v_scroll.set(set_to);
    }

    fn change_selection(&self, i: u32) {
        match self.selected.get() {
            Some(i) => {
                match self.entries.borrow().get(i as usize) {
                    Some(entry) => {
                        entry.highlighted.set(false);
                    },
                    None => {},
                }
            },
            _ => {},
        }

        if let Some(entry) = self.entries.borrow().get(i as usize) {
            entry.highlighted.set(true);
            self.selected.set(Some(i));

            let mut y = 0;

            for e in self.entries.borrow().index(0..(i as usize)) {
                y += e.height.get();
            }

            let v_scroll = self.v_scroll.get();

            if y < v_scroll as u32 {
                self.scroll(y as i32 - v_scroll);
            } else if (y + entry.height.get() as u32) > (v_scroll as u32 + self.rect.get().height) {
                self.scroll((y + entry.height.get()) as i32 - (v_scroll + self.rect.get().height as i32));
            }
        }
    }
}

impl Widget for List {
    fn rect(&self) -> &Cell<Rect> {
        &self.rect
    }

    fn draw(&self, renderer: &mut Renderer, _focused: bool) {
        let mut current_y = 0;
        let x = self.rect.get().x;
        let y = self.rect.get().y;
        let width = self.rect.get().width;
        let height = self.rect.get().height;

        let mut target = orbimage::Image::new(width, height);
        target.set(WINDOW_BACKGROUND);

        for entry in self.entries.borrow().iter() {
            let mut image = orbimage::Image::new(width, entry.height.get());

            if entry.highlighted.get() {
                image.set(entry.highlight.get());
            } else {
                image.set(ITEM_BACKGROUND);
            }

            for widget in entry.widgets().borrow().iter() {
                widget.draw(&mut image, false)
            }

            let image = image.data();
            target.image(x, current_y-self.v_scroll.get(), width, entry.height.get(), &image);

            current_y += entry.height.get() as i32
        }
        let target = target.data();
        renderer.image(x, y, width, height, &target)
    }

    fn event(&self, event: Event, focused: bool, redraw: &mut bool) -> bool {
        match event {
            Event::Mouse { point, left_button, .. } => {
                let mut click = false;

                let rect = self.rect.get();
                if rect.contains(point) {
                    if left_button {
                        if self.pressed.check_set(true) {
                            *redraw = true;
                        }
                    } else {
                        if self.pressed.check_set(false) {
                            click = true;
                            *redraw = true;
                        }
                    }
                } else {
                    if !left_button {
                        if self.pressed.check_set(false) {
                            *redraw = true;
                        }
                    }
                }

                if let Some(i) = self.get_entry_index(point) {
                    if click {
                        if let Some(entry) = self.entries.borrow().get(i as usize) {
                            entry.emit_click(point);
                        }
                    }

                    match self.selected.get() {
                        None => {
                            self.change_selection(i);
                            *redraw = true;
                        },
                        Some(selected) => {
                            if selected != i {
                                self.change_selection(i);
                                *redraw = true;
                            }
                        },
                    }
                }
            },
            Event::UpArrow => {
                match self.selected.get() {
                    None => {
                        self.change_selection(0);
                        *redraw = true;
                    },
                    Some(i) => {
                        if i > 0 {
                            self.change_selection(i - 1);
                            *redraw = true;
                        }
                    }
                }
            },
            Event::DownArrow => {
                match self.selected.get() {
                    None => {
                        self.change_selection(0);
                        *redraw = true;
                    },
                    Some(i) => {
                        if i < self.entries.borrow().len() as u32 - 1 {
                            self.change_selection(i + 1);
                            *redraw = true;
                        }
                    }
                }
            },
            Event::Home => {
                self.change_selection(0);
                *redraw = true
            },
            Event::End => {
                self.change_selection(self.entries.borrow().len() as u32 - 1);
                *redraw = true
            },
            Event::Enter => {
                match self.selected.get() {
                    Some(i) => {
                        match self.entries.borrow().get(i as usize) {
                            Some(entry) => {
                                entry.emit_click(Point { x: 0, y: 0});
                            },
                            None => {},
                        }
                    },
                    _ => {},
                }
            },
            Event::Scroll { y, .. } => {
                self.scroll(y * -96);
                *redraw = true;
            },
            _ => {}
        }
        focused
    }
}

impl Place for List {}

