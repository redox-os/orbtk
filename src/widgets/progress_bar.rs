use orbclient::Renderer;
use std::cell::{Cell, RefCell};
use std::cmp::{min, max};
use std::sync::Arc;

use cell::{CloneCell, CheckSet};
use draw::draw_box;
use event::Event;
use point::Point;
use rect::Rect;
use thickness::Thickness;
use theme::{Theme, Selector};
use traits::{Click, Place, Style};
use widgets::{Widget, VerticalPlacement, HorizontalPlacement};

pub struct ProgressBar {
    pub rect: Cell<Rect>,
    children: RefCell<Vec<Arc<Widget>>>,
    local_position: Cell<Point>,
    vertical_placement: Cell<VerticalPlacement>,
    horizontal_placement: Cell<HorizontalPlacement>,
    margin: Cell<Thickness>,
    pub selector: CloneCell<Selector>,
    pub value: Cell<i32>,
    pub minimum: i32,
    pub maximum: i32,
    click_callback: RefCell<Option<Arc<Fn(&ProgressBar, Point)>>>,
    pressed: Cell<bool>,
}

impl ProgressBar {
    pub fn new() -> Arc<Self> {
        Arc::new(ProgressBar {
            rect: Cell::new(Rect::default()),
            local_position: Cell::new(Point::new(0, 0)),
            vertical_placement: Cell::new(VerticalPlacement::Absolute),
            horizontal_placement: Cell::new(HorizontalPlacement::Absolute),
            margin: Cell::new(Thickness::default()),
            children: RefCell::new(vec![]),
            selector: CloneCell::new(Selector::new(Some("progress-bar"))),
            value: Cell::new(0),
            minimum: 0,
            maximum: 100,
            click_callback: RefCell::new(None),
            pressed: Cell::new(false),
        })
    }

    pub fn value(&self, value: i32) -> &Self {
        self.value.set(value);
        self
    }
}

impl Click for ProgressBar {
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

impl Place for ProgressBar {}

impl Style for ProgressBar {
    fn selector(&self) -> &CloneCell<Selector> {
        &self.selector
    }
}

impl Widget for ProgressBar {
    fn name(&self) -> &str {
        "ProgressBar"
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

    fn draw(&self, renderer: &mut Renderer, _focused: bool, theme: &Theme) {
        let rect = self.rect.get();
        let progress_rect = Rect{
                                width: (rect.width as i32 *
                                        max(0, min(self.maximum, self.value.get() - self.minimum)) /
                                        max(1, self.maximum - self.minimum)) as u32,
                                ..self.rect.get()
                            };

        let selector = &self.selector.get();

        draw_box(renderer, rect, theme, selector);

        let b_r = theme.get("border-radius", selector).map(|v| v.uint().unwrap()).unwrap_or(1);
        let b_t = theme.get("border-width", selector).map(|v| v.uint().unwrap()).unwrap_or(0);


        if progress_rect.width >=  b_t + b_r * 2 {
            //TODO: set this selector as the child of self.selector
            draw_box(renderer, progress_rect, theme, &Selector::new(Some("progress")));
        }
    }

    fn event(&self, event: Event, focused: bool, redraw: &mut bool, caught: &mut bool) -> bool {
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

                    *caught = true;
                } else {
                    if !left_button {
                        if self.pressed.check_set(false) {
                            *redraw = true;
                        }
                    }
                }

                if click {
                    let click_point: Point = point - rect.point();
                    self.emit_click(click_point);
                }
            }
            _ => (),
        }

        focused
    }

    fn children(&self) -> &RefCell<Vec<Arc<Widget>>> {
        &self.children
    }
}
