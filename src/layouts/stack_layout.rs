use std::cell::{Cell, RefCell};
use std::sync::Arc;

use rect::Rect;
use point::Point;
use thickness::Thickness;
use traits::Place;
use widgets::{Widget, VerticalPlacement, HorizontalPlacement};

/// Describes the orientation of a StackLayout
#[derive(PartialEq, Copy, Clone)]
pub enum Orientation {
    Vertical,
    Horizontal,
}

/// StackLayout arranges it's child widgets into a single line. The children could be
/// stacked horizontal or vertical.
pub struct StackLayout {
    pub rect: Cell<Rect>,
    local_position: Cell<Point>,
    vertical_placement: Cell<VerticalPlacement>,
    horizontal_placement: Cell<HorizontalPlacement>,
    margin: Cell<Thickness>,
    children: RefCell<Vec<Arc<dyn Widget>>>,
    orientation: Cell<Orientation>,
    spacing: Cell<u32>,
}

impl StackLayout {
    /// Creates and returns a new StackLayout with the given orientation.
    pub fn new(orientation: Orientation) -> Arc<Self> {
        Arc::new(StackLayout {
            rect: Cell::new(Rect::default()),
            local_position: Cell::new(Point::new(0, 0)),
            vertical_placement: Cell::new(VerticalPlacement::Absolute),
            horizontal_placement: Cell::new(HorizontalPlacement::Absolute),
            margin: Cell::new(Thickness::default()),
            children: RefCell::new(vec![]),
            orientation: Cell::new(orientation),
            spacing: Cell::new(0),
        })
    }

    /// Borrow the arrangement orientation of stack layout.
    pub fn orientation(&self) -> &Cell<Orientation> {
        &self.orientation
    }

    /// Set the spacing between the children of stack layout.
    pub fn spacing(&self, spacing: u32) -> &Self {
        self.spacing.set(spacing);
        self
    }
}

impl Place for StackLayout {}

impl Widget for StackLayout {
    fn name(&self) -> &str {
        "StackLayout"
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

    fn children(&self) -> &RefCell<Vec<Arc<dyn Widget>>> {
        &self.children
    }

    fn arrange(&self) {
        let parent_rect = self.rect().get();

        let mut x_position = parent_rect.x;
        let mut y_position = parent_rect.y;

        for child in &*self.children().borrow_mut() {
            let child_position = child.local_position().get();
            let mut child_rect = child.rect().get();
            let child_margin = child.margin().get();

            match self.orientation().get() {
                Orientation::Horizontal => {
                    match child.vertical_placement().get() {
                        VerticalPlacement::Absolute => {
                            child_rect.y = parent_rect.y + child_position.y;
                        },
                        VerticalPlacement::Top => {
                            child_rect.y = parent_rect.y + child_margin.top as i32;
                        },
                        VerticalPlacement::Center => {
                            child_rect.y = parent_rect.y + parent_rect.height as i32 / 2
                                - child_rect.height as i32 / 2;
                        },
                        VerticalPlacement::Bottom => {
                            child_rect.y = parent_rect.y + parent_rect.height as i32
                                - child_rect.height as i32
                                - child_margin.bottom as i32;
                        },
                        VerticalPlacement::Stretch => {
                            child_rect.y = parent_rect.y;
                            child_rect.height = parent_rect.height;
                        }
                    }

                    child_rect.x = x_position + child_margin.left as i32;
                    x_position = child_rect.x + child_rect.width as i32 + child_margin.right as i32
                        + self.spacing.get() as i32;
                }
                Orientation::Vertical => {
                    match child.horizontal_placement().get() {
                        HorizontalPlacement::Absolute => {
                            child_rect.x = parent_rect.x + child_position.x;
                        },
                        HorizontalPlacement::Left => {
                            child_rect.x = parent_rect.x + child_margin.left as i32;
                        },
                        HorizontalPlacement::Center => {
                            child_rect.x = parent_rect.x + parent_rect.width as i32 / 2
                                - child_rect.width as i32 / 2;
                        },
                        HorizontalPlacement::Right => {
                            child_rect.x = parent_rect.x + parent_rect.width as i32
                                - child_rect.width as i32
                                - child_margin.right as i32;
                        },
                        HorizontalPlacement::Stretch => {
                            child_rect.x = parent_rect.x;
                            child_rect.width = parent_rect.width;
                        }
                    }

                    child_rect.y = y_position + child_margin.top as i32;
                    y_position = child_rect.y + child_rect.height as i32
                        + child_margin.bottom as i32
                        + self.spacing.get() as i32;
                }
            }

            child.rect().set(child_rect);
            child.arrange();
        }
    }
}
