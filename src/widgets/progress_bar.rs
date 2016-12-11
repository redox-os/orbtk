use orbclient::Color;
use std::cell::{Cell, RefCell};
use std::cmp::{min, max};
use std::sync::Arc;

use cell::CheckSet;
use event::Event;
use point::Point;
use rect::Rect;
use renderer::Renderer;
use theme::{ITEM_BACKGROUND, ITEM_BORDER, ITEM_SELECTION};
use traits::{Click, Place};
use widgets::Widget;

pub struct ProgressBar {
    pub rect: Cell<Rect>,
    pub bg: Color,
    pub fg: Color,
    pub fg_border: Color,
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
            bg: ITEM_BACKGROUND,
            fg: ITEM_SELECTION,
            fg_border: ITEM_BORDER,
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

impl Widget for ProgressBar {
    fn rect(&self) -> &Cell<Rect> {
        &self.rect
    }

    fn draw(&self, renderer: &mut Renderer, _focused: bool) {
        let rect = self.rect.get();
        let progress_rect = Rect::new(rect.x,
                                rect.y,
                                ((rect.width as i32 *
                                  max(0, min(self.maximum, self.value.get() - self.minimum))) /
                                 max(1,
                                     self.maximum -
                                     self.minimum)) as u32,
                                rect.height);
        renderer.rect(rect, self.bg);
        renderer.rect(progress_rect, self.fg);
        renderer.rounded_rect(rect, 0, self.fg_border);
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
                    let click_point: Point = point - rect.point();
                    self.emit_click(click_point);
                }
            }
            _ => (),
        }

        focused
    }
}
