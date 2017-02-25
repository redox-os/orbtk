use orbclient::Renderer;
use orbimage;
use std::cell::{ Cell, RefCell };
use std::sync::Arc;

use cell::CheckSet;
use event::Event;
use point::Point;
use rect::Rect;
use theme::{ ITEM_BACKGROUND, WINDOW_BACKGROUND };
use traits::{ Click, Place };
use widgets::Widget;

/// An entry in a list
/// Each entry stores widgets within. 
pub struct Entry {
    pub height: Cell<u32>,
    click_callback: RefCell<Option<Arc<Fn(&Entry, Point)>>>,
    widgets: RefCell<Vec<Arc<Widget>>>
}

impl Entry {
    pub fn new(h: u32) -> Arc<Self> {
        Arc::new(Entry {
            height: Cell::new(h),
            click_callback: RefCell::new(None),
            widgets: RefCell::new(vec![]),
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
    pressed: Cell<bool>
}

impl List {
    pub fn new() -> Arc<Self> {
        Arc::new(List {
            rect: Cell::new(Rect::default()),
            v_scroll: Cell::new(0),
            current_height: Cell::new(0),
            entries: RefCell::new(vec![]),
            pressed: Cell::new(false),
        })
    }

    pub fn push(&self, entry: &Arc<Entry>) {
        let h = entry.height.get();
        self.entries.borrow_mut().push(entry.clone());
        self.current_height.set(self.current_height.get() + h);
    }

    // Given absolute coordinates, returns the list entry
    // drawn at that point.
    fn get_entry(&self, p: Point) -> Option<Arc<Entry>> {
        if self.rect.get().contains(p) {
            let mut current_y = 0;
            let x = self.rect.get().x;
            let y = self.rect.get().y;
            let width = self.rect.get().width;
            let scroll = self.v_scroll.get();

            for entry in self.entries.borrow().iter() {
                if Rect::new(x, y+current_y-scroll, width, entry.height.get()).contains(p) {
                    return Some(entry.clone());
                }
                current_y += entry.height.get() as i32
            }

            None
        } else {
            None
        }
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
            image.set(ITEM_BACKGROUND);

            for widget in entry.widgets().borrow().iter() {
                widget.draw(&mut image, false)
            }
            
            let image = image.data();
            target.image(x, y+current_y-self.v_scroll.get(), width, entry.height.get(), &image);

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

                if click {
                    if let Some(entry) = self.get_entry(point) { 
                        entry.emit_click(point)
                    }
                }
            },

            Event::UpArrow => { self.scroll(10); *redraw = true }
            Event::DownArrow => { self.scroll(-10); *redraw = true }

            _ => {}
        }
        focused
    }
}

impl Place for List {}

