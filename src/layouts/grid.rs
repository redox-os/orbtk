use orbclient::Renderer;
use std::cell::{Cell, RefCell};
use std::collections::BTreeMap;
use std::sync::Arc;

use cell::CheckSet;
use event::Event;
use rect::Rect;
use point::Point;
use thickness::Thickness;
use theme::{Theme};
use traits::Place;
use widgets::{Widget, VerticalPlacement, HorizontalPlacement};

pub struct Grid {
    pub rect: Cell<Rect>,
    local_position: Cell<Point>,
    vertical_placement: Cell<VerticalPlacement>,
    horizontal_placement: Cell<HorizontalPlacement>,
    margin: Cell<Thickness>,
    children: RefCell<Vec<Arc<Widget>>>,
    space_x: Cell<i32>,
    space_y: Cell<i32>,
    columns: Cell<usize>,
    row_count: Cell<usize>,
    column_count: Cell<usize>,
    entries: RefCell<BTreeMap<(usize, usize), Arc<Widget>>>,
    focused: Cell<Option<(usize, usize)>>
}

impl Grid {
    pub fn new() -> Arc<Self> {
        Arc::new(Grid {
            rect: Cell::new(Rect::default()),
            local_position: Cell::new(Point::new(0, 0)),
            vertical_placement: Cell::new(VerticalPlacement::Absolute),
            horizontal_placement: Cell::new(HorizontalPlacement::Absolute),
            margin: Cell::new(Thickness::default()),
            children: RefCell::new(vec![]),
            space_x: Cell::new(0),
            space_y: Cell::new(0),
            columns: Cell::new(0),
            row_count: Cell::new(0),
            column_count: Cell::new(0),
            entries: RefCell::new(BTreeMap::new()),
            focused: Cell::new(None),
        })
    }

    pub fn columns(&self, columns: usize) -> &Self {
        self.columns.set(columns);
        self
    }

    pub fn add<T: Widget>(&self, entry: &Arc<T>) {
        if self.column_count.get() == self.columns.get() {
            self.row_count.set(self.row_count.get() + 1);
            self.column_count.set(0);
        }

        self.entries.borrow_mut().insert((self.column_count.get(), self.row_count.get()), entry.clone());
        self.column_count.set(self.column_count.get() + 1);
        self.arrange(false);
    }

    pub fn insert<T: Widget>(&self, col: usize, row: usize, entry: &Arc<T>) {
        self.entries.borrow_mut().insert((col, row), entry.clone());
        self.arrange(false);
    }

    pub fn clear(&self) {
        self.entries.borrow_mut().clear();
    }

    pub fn remove(&self, col: usize, row: usize) {
        self.entries.borrow_mut().remove(&(col, row));
    }

    pub fn spacing(&self, x: i32, y: i32) -> &Self {
        self.space_x.set(x);
        self.space_y.set(y);
        self
    }

    pub fn arrange(&self, resize: bool) {
        let mut cols = Vec::new();
        let mut rows = Vec::new();
        for (&(col, row), entry) in self.entries.borrow().iter() {
            while col >= cols.len() {
                cols.push(Rect::default());
            }
            while row >= rows.len() {
                rows.push(Rect::default());
            }
            let rect = entry.rect().get();
            if rect.width >= cols[col].width {
                cols[col as usize].width = rect.width;
            }
            if rect.width >= rows[row].width {
                rows[row as usize].width = rect.width;
            }
            if rect.height >= cols[col].height {
                cols[col as usize].height = rect.height;
            }
            if rect.height >= rows[row].height {
                rows[row as usize].height = rect.height;
            }
        }

        let rect = self.rect.get();
        let space_x = self.space_x.get();
        let space_y = self.space_y.get();

        let mut x = rect.x;
        for col in cols.iter_mut() {
            col.x = x;
            x += col.width as i32 + space_x;
        }

        let mut y = rect.y;
        for row in rows.iter_mut() {
            row.y = y;
            y += row.height as i32 + space_y;
        }

        for (&(col, row), entry) in self.entries.borrow().iter() {
            let mut rect = entry.rect().get();
            rect.x = cols[col].x;
            rect.y = rows[row].y;
            if resize {
                rect.width = cols[col].width;
                rect.height = rows[row].height;
            }
            entry.rect().set(rect);
        }
    }
}

impl Place for Grid {
    fn position(&self, x: i32, y: i32) -> &Self {
        let mut rect = self.rect().get();
        rect.x = x;
        rect.y = y;
        self.rect().set(rect);

        self.arrange(false);

        self
    }
}

impl Widget for Grid {
    fn name(&self) -> &str {
        "Grid"
    }

    fn rect(&self) -> &Cell<Rect> {
        &self.rect
    }

    fn local_position(&self) -> &Cell<Point> {
        &self.local_position
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

    fn draw(&self, renderer: &mut Renderer, _focused: bool, theme: &Theme) {
        for (&(col, row), entry) in self.entries.borrow().iter() {
            entry.draw(renderer, self.focused.get() == Some((col, row)), theme);
        }
    }

    fn event(&self, event: Event, mut focused: bool, redraw: &mut bool, caught: &mut bool) -> bool {
        for (&(col, row), entry) in self.entries.borrow().iter() {
            let is_focused = self.focused.get() == Some((col, row));
            if entry.event(event, focused && is_focused, redraw, caught) {
                if self.focused.check_set(Some((col, row))) || ! focused {
                    focused = true;
                    *redraw = true;
                }
            } else if is_focused {
                self.focused.set(None);
            }

            if *caught {
                break;
            }
        }

        focused
    }

    fn children(&self) -> &RefCell<Vec<Arc<Widget>>> {
        &self.children
    }
}
