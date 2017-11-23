use orbclient::{Color, Renderer};
use std::cell::{Cell, RefCell};
use std::cmp::{min, max};
use std::sync::Arc;

use cell::CheckSet;
use event::Event;
use point::Point;
use rect::Rect;
use theme::{ITEM_BACKGROUND, ITEM_BORDER, ITEM_SELECTION};
use traits::{Border, Click, Place};
use widgets::Widget;

pub struct ProgressBar {
    pub rect: Cell<Rect>,
    pub bg: Cell<Color>,
    pub fg: Cell<Color>,
    pub fg_border: Color,
    pub border: Cell<bool>,
    pub border_radius: Cell<u32>,
    pub value: Cell<i32>,
    pub minimum: Cell<i32>,
    pub maximum: Cell<i32>,
    click_callback: RefCell<Option<Arc<Fn(&ProgressBar, Point)>>>,
    pressed: Cell<bool>,
    pub visible: Cell<bool>,
}

impl ProgressBar {
    pub fn new() -> Arc<Self> {
        Arc::new(ProgressBar {
            rect: Cell::new(Rect::default()),
            bg: Cell::new(ITEM_BACKGROUND),
            fg: Cell::new(ITEM_SELECTION),
            fg_border: ITEM_BORDER,
            border: Cell::new(true),
            border_radius: Cell::new(0),
            value: Cell::new(0),
            minimum: Cell::new(0),
            maximum: Cell::new(100),
            click_callback: RefCell::new(None),
            pressed: Cell::new(false),
            visible: Cell::new(true),
        })
    }

    pub fn value(&self, value: i32) -> &Self {
        self.value.set(value);
        self
    }
}

impl Border for ProgressBar {
    fn border(&self, enabled: bool) -> &Self {
        self.border.set(enabled);
        self
    }

    fn border_radius(&self, radius: u32) -> &Self {
        self.border_radius.set(radius);
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
        if self.visible.get(){
            let rect = self.rect.get();
            let progress_rect = Rect{
                                    width: (rect.width as i32 *
                                            max(0, min(self.maximum.get(), self.value.get() - self.minimum.get())) /
                                            max(1, self.maximum.get() - self.minimum.get())) as u32,
                                    ..self.rect.get()
                                };
    
            let b_r = self.border_radius.get();
            renderer.rounded_rect(rect.x, rect.y, rect.width, rect.height, b_r, true, self.bg.get());
            if progress_rect.width >= b_r * 2 {
                renderer.rounded_rect(progress_rect.x, progress_rect.y,
                                      progress_rect.width, progress_rect.height,
                                      b_r, true, self.fg.get());
            }
            if self.border.get() {
                renderer.rounded_rect(rect.x, rect.y, rect.width, rect.height, b_r, false, self.fg_border);
            }
        }
    }

    fn event(&self, event: Event, focused: bool, redraw: &mut bool) -> bool {
        if self.visible.get(){
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
        }
        focused
    }
    
    fn visible(&self, flag: bool){
        self.visible.set(flag);
    }

    fn name(&self) -> Option<&'static str> {
        Some("ProgressBar")
    }
}
