use super::{Color, CopyCell, Event, Place, Point, Rect, Renderer, Widget, Window};
use super::callback::Click;
use super::cell::CheckSet;

use std::cmp::{min, max};
use std::sync::Arc;

pub struct ProgressBar {
    pub rect: CopyCell<Rect>,
    pub value: CopyCell<i32>,
    pub minimum: i32,
    pub maximum: i32,
    pub bg: Color,
    pub fg: Color,
    click_callback: Option<Arc<Fn(&ProgressBar, Point)>>,
    pressed: CopyCell<bool>,
}

impl ProgressBar {
    pub fn new() -> Self {
        ProgressBar {
            rect: CopyCell::new(Rect::default()),
            value: CopyCell::new(0),
            minimum: 0,
            maximum: 100,
            bg: Color::rgb(255, 255, 255),
            fg: Color::rgb(65, 139, 212),
            click_callback: None,
            pressed: CopyCell::new(false),
        }
    }

    pub fn place(self, window: &mut Window) -> Arc<Self> {
        let arc = Arc::new(self);

        window.widgets.push(arc.clone());

        arc
    }

    pub fn value(self, value: i32) -> Self {
        self.value.set(value);
        self
    }
}

impl Click for ProgressBar {
    fn emit_click(&self, point: Point) {
        if let Some(ref click_callback) = self.click_callback {
            click_callback(self, point);
        }
    }

    fn on_click<T: Fn(&Self, Point) + 'static>(mut self, func: T) -> Self {
        self.click_callback = Some(Arc::new(func));

        self
    }
}

impl Place for ProgressBar {
    fn rect(&self) -> &CopyCell<Rect> {
        &self.rect
    }
}

impl Widget for ProgressBar {
    fn draw(&self, renderer: &mut Renderer, _focused: bool) {
        let rect = self.rect.get();
        renderer.rect(rect, self.bg);
        renderer.rect(Rect::new(rect.x,
                                rect.y,
                                ((rect.width as i32 *
                                  max(0, min(self.maximum, self.value.get() - self.minimum))) /
                                 max(1,
                                     self.maximum -
                                     self.minimum)) as u32,
                                rect.height),
                      self.fg);
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
