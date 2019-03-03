use orbclient;
use std::cell::{Cell, RefCell};
use std::cmp;
use std::sync::Arc;

use cell::{CheckSet, CloneCell};
use event::Event;
use point::Point;
use rect::Rect;
use thickness::Thickness;
use theme::Selector;
use traits::{Click, Place, Style};
use widgets::{HorizontalPlacement, VerticalPlacement, Widget};
use std::ops::Index;
use primitives::Rectangle;

/// An entry in a list
/// Each entry stores widgets within.
pub struct Entry {
    pub rect: Cell<Rect>,
    local_position: Cell<Point>,
    vertical_placement: Cell<VerticalPlacement>,
    horizontal_placement: Cell<HorizontalPlacement>,
    margin: Cell<Thickness>,
    children: RefCell<Vec<Arc<Widget>>>,
    click_callback: RefCell<Option<Arc<Fn(&Entry, Point)>>>,
    highlighted: Cell<bool>,
    selector: CloneCell<Selector>,
}

impl Entry {
    pub fn new(h: u32) -> Arc<Self> {
        // todo: this selector as child of list selector
        let selector = CloneCell::new(Selector::new(Some("entry")));

        let background = Rectangle::new();
        background.placement(VerticalPlacement::Stretch, HorizontalPlacement::Stretch);
        background.selector().bind(&selector);

        Arc::new(Entry {
            rect: Cell::new(Rect::new(0, 0, 0, h)),
            local_position: Cell::new(Point::new(0, 0)),
            vertical_placement: Cell::new(VerticalPlacement::Absolute),
            horizontal_placement: Cell::new(HorizontalPlacement::Stretch),
            margin: Cell::new(Thickness::default()),
            children: RefCell::new(vec![background]),
            click_callback: RefCell::new(None),
            highlighted: Cell::new(false),
            selector,
        })
    }

    /// Adds a widget to the entry
    pub fn add<T: Widget>(&self, widget: &Arc<T>) {
        if let Some(background) = (*self.children().borrow()).get(0) {
            background.add(widget.clone());
        }
    }
}

impl Widget for Entry {
    fn name(&self) -> &str {
        "Rectangle"
    }

    fn rect(&self) -> &Cell<Rect> {
        &self.rect
    }

    fn vertical_placement(&self) -> &Cell<VerticalPlacement> {
        &self.vertical_placement
    }

    fn horizontal_placement(&self) -> &Cell<HorizontalPlacement> {
        &self.horizontal_placement
    }

    fn margin(&self) -> &Cell<Thickness> {
        &self.margin
    }

    fn local_position(&self) -> &Cell<Point> {
        &self.local_position
    }

    fn children(&self) -> &RefCell<Vec<Arc<Widget>>> {
        &self.children
    }

    fn update(&self) {
        let selector = Selector::new(Some("entry")).with_pseudo_class(if self.highlighted.get() {
            "active"
        } else {
            "inactive"
        });

        self.selector.set(selector);
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
    local_position: Cell<Point>,
    vertical_placement: Cell<VerticalPlacement>,
    horizontal_placement: Cell<HorizontalPlacement>,
    margin: Cell<Thickness>,
    children: RefCell<Vec<Arc<Widget>>>,
    pub selector: CloneCell<Selector>,
    v_scroll: Cell<i32>,
    current_height: Cell<u32>,
    entries: RefCell<Vec<Arc<Entry>>>,
    pressed: Cell<bool>,
    selected: Cell<Option<u32>>,
}

impl List {
    pub fn new() -> Arc<Self> {
        let selector = CloneCell::new(Selector::new(Some("list")));

        let background = Rectangle::new();
        background.placement(VerticalPlacement::Stretch, HorizontalPlacement::Stretch);
        background.selector().bind(&selector);

        Arc::new(List {
            rect: Cell::new(Rect::default()),
            local_position: Cell::new(Point::new(0, 0)),
            vertical_placement: Cell::new(VerticalPlacement::Absolute),
            horizontal_placement: Cell::new(HorizontalPlacement::Absolute),
            margin: Cell::new(Thickness::default()),
            children: RefCell::new(vec![background]),
            selector,
            v_scroll: Cell::new(0),
            current_height: Cell::new(0),
            entries: RefCell::new(vec![]),
            pressed: Cell::new(false),
            selected: Cell::new(None),
        })
    }

    pub fn push(&self, entry: &Arc<Entry>) {
        let h = entry.rect().get().height;
        self.entries.borrow_mut().push(entry.clone());
        self.current_height.set(self.current_height.get() + h);
        if let Some(background) = (*self.children().borrow()).get(0) {
            background.add(entry.clone());
        }
        self.arrange();
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
                if Rect::new(x, y + current_y - scroll, width, entry.rect().get().height)
                    .contains(p)
                {
                    return Some(i as u32);
                }
                current_y += entry.rect().get().height as i32
            }
        }

        None
    }

    pub fn scroll(&self, y: i32) {
        let mut set_to = self.v_scroll.get() + y;

        let max = cmp::max(
            0,
            self.current_height.get() as i32 - self.rect.get().height as i32,
        );
        if set_to < 0 {
            set_to = 0;
        } else if set_to > max {
            set_to = max;
        }

        self.v_scroll.set(set_to);
    }

    fn change_selection(&self, i: u32) {
        match self.selected.get() {
            Some(i) => match self.entries.borrow().get(i as usize) {
                Some(entry) => {
                    entry.highlighted.set(false);
                }
                None => {}
            },
            _ => {}
        }

        if let Some(entry) = self.entries.borrow().get(i as usize) {
            entry.highlighted.set(true);
            self.selected.set(Some(i));

            let mut y = 0;

            for e in self.entries.borrow().index(0..(i as usize)) {
                y += e.rect().get().height;
            }

            let v_scroll = self.v_scroll.get();

            if y < v_scroll as u32 {
                self.scroll(y as i32 - v_scroll);
            } else if (y + entry.rect().get().height as u32)
                > (v_scroll as u32 + self.rect.get().height)
            {
                self.scroll(
                    (y + entry.rect().get().height) as i32
                        - (v_scroll + self.rect.get().height as i32),
                );
            }
        }
    }
}

impl Style for List {
    fn selector(&self) -> &CloneCell<Selector> {
        &self.selector
    }
}

impl Widget for List {
    fn name(&self) -> &str {
        "List"
    }

    fn rect(&self) -> &Cell<Rect> {
        &self.rect
    }

    fn vertical_placement(&self) -> &Cell<VerticalPlacement> {
        &self.vertical_placement
    }

    fn horizontal_placement(&self) -> &Cell<HorizontalPlacement> {
        &self.horizontal_placement
    }

    fn margin(&self) -> &Cell<Thickness> {
        &self.margin
    }

    fn local_position(&self) -> &Cell<Point> {
        &self.local_position
    }

    fn event(&self, event: Event, focused: bool, redraw: &mut bool, caught: &mut bool) -> bool {
        match event {
            Event::Mouse {
                point, left_button, ..
            } => {
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

                    *caught = true;
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
                        }
                        Some(selected) => if selected != i {
                            self.change_selection(i);
                            *redraw = true;
                        },
                    }
                }
            }
            Event::KeyPressed(key_event) => match key_event.scancode {
                orbclient::K_UP => match self.selected.get() {
                    None => {
                        self.change_selection(0);
                        *redraw = true;
                    }
                    Some(i) => if i > 0 {
                        self.change_selection(i - 1);
                        *redraw = true;
                    },
                },
                orbclient::K_DOWN => match self.selected.get() {
                    None => {
                        self.change_selection(0);
                        *redraw = true;
                    }
                    Some(i) => if i < self.entries.borrow().len() as u32 - 1 {
                        self.change_selection(i + 1);
                        *redraw = true;
                    },
                },
                orbclient::K_HOME => {
                    self.change_selection(0);
                    *redraw = true
                }
                orbclient::K_END => {
                    self.change_selection(self.entries.borrow().len() as u32 - 1);
                    *redraw = true
                }
                orbclient::K_ENTER => match self.selected.get() {
                    Some(i) => match self.entries.borrow().get(i as usize) {
                        Some(entry) => {
                            entry.emit_click(Point { x: 0, y: 0 });
                        }
                        None => {}
                    },
                    _ => {}
                },
                _ => {}
            },
            Event::Scroll { y, .. } => {
                self.scroll(y * -96);
                *redraw = true;
                self.arrange();
            }
            _ => {}
        }
        focused
    }

    fn children(&self) -> &RefCell<Vec<Arc<Widget>>> {
        &self.children
    }

    fn arrange(&self) {
        if let Some(background) = (self.children.borrow()).get(0) {
            let mut current_y = self.rect().get().y;
            background.rect().set(self.rect().get());

            for entry in background.children().borrow().iter() {
                let mut entry_rect = entry.rect().get();
                entry_rect.y = current_y - self.v_scroll.get();
                current_y += entry_rect.height as i32;
                entry.rect().set(entry_rect);
                entry.arrange();
            }
        }
    }
}

impl Place for List {}
