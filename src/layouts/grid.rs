use std::cell::{Cell, RefCell};
use std::collections::BTreeMap;
use std::sync::Arc;

use point::Point;
use rect::Rect;
use thickness::Thickness;
use traits::Place;
use widgets::{HorizontalPlacement, VerticalPlacement, Widget};

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
    entries: RefCell<BTreeMap<(usize, usize), usize>>,
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

        self.children().borrow_mut().push(entry.clone());
        self.entries.borrow_mut().insert(
            (self.column_count.get(), self.row_count.get()),
            self.children.borrow().len() - 1,
        );
        self.column_count.set(self.column_count.get() + 1);
        self.arrange_children(false);
    }

    pub fn insert<T: Widget>(&self, col: usize, row: usize, entry: &Arc<T>) {
        self.children().borrow_mut().push(entry.clone());
        self.entries
            .borrow_mut()
            .insert((col, row), self.children.borrow().len() - 1);
        self.arrange_children(false);
    }

    pub fn clear(&self) {
        self.entries.borrow_mut().clear();
        self.children.borrow_mut().clear();
    }

    pub fn remove(&self, col: usize, row: usize) {
        if self.children.borrow().len() == 0 || !self.entries.borrow_mut().contains_key(&(col, row))
        {
            return;
        }

        let mut entries = self.entries.borrow_mut().clone();
        let mut removed_index = 0;

        if let Some(index) = self.entries.borrow_mut().get(&(col, row)) {
            self.children.borrow_mut().remove(*index);
            entries.remove(&(col, row));

            removed_index = *index;
        }

        self.entries.borrow_mut().clear();

        for (&(col, row), old_index) in entries.iter() {
            if *old_index > removed_index {
                self.entries.borrow_mut().insert((col, row), old_index - 1);
            } else {
                self.entries.borrow_mut().insert((col, row), *old_index);
            }
        }
    }

    pub fn spacing(&self, x: i32, y: i32) -> &Self {
        self.space_x.set(x);
        self.space_y.set(y);
        self
    }

    pub fn arrange_children(&self, resize: bool) {
        let mut cols = Vec::new();
        let mut rows = Vec::new();
        for (&(col, row), index) in self.entries.borrow().iter() {
            if let Some(entry) = self.children().borrow().get(*index) {
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

                entry.arrange();
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

        for (&(col, row), index) in self.entries.borrow().iter() {
            if let Some(entry) = self.children().borrow().get(*index) {
                let mut rect = entry.rect().get();
                rect.x = cols[col].x;
                rect.y = rows[row].y;
                if resize {
                    rect.width = cols[col].width;
                    rect.height = rows[row].height;
                }
                entry.rect().set(rect);
                entry.arrange();
            }
        }
    }
}

impl Place for Grid {
    fn position(&self, x: i32, y: i32) -> &Self {
        let mut rect = self.rect().get();
        rect.x = x;
        rect.y = y;
        self.rect().set(rect);

        self.arrange_children(false);

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

    fn children(&self) -> &RefCell<Vec<Arc<Widget>>> {
        &self.children
    }
}
