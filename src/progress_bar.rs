use super::{Color, Event, Placeable, Point, Rect, Renderer, Widget, WidgetCore};
use super::callback::Click;
use super::cell::CheckSet;

use std::cell::Cell;
use std::cmp::{min, max};
use std::sync::Arc;

pub struct ProgressBar {
    pub core: WidgetCore,
    pub value: Cell<i32>,
    pub minimum: i32,
    pub maximum: i32,
    click_callback: Option<Arc<Fn(&ProgressBar, Point)>>,
    pressed: Cell<bool>,
}

impl ProgressBar {
    pub fn new() -> Self {
        ProgressBar {
            core: WidgetCore::new()
                    .fg(Color::rgb(65, 139, 212)),
            value: Cell::new(0),
            minimum: 0,
            maximum: 100,
            click_callback: None,
            pressed: Cell::new(false),
        }
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

impl Placeable for ProgressBar {}

impl Widget for ProgressBar {
    fn rect(&self) -> &Cell<Rect> {
        &self.core.rect
    }

    fn draw(&self, renderer: &mut Renderer, _focused: bool) {
        let rect = self.core.rect.get();
        renderer.rect(rect, self.core.bg);
        renderer.rect(Rect::new(rect.x,
                                rect.y,
                                ((rect.width as i32 *
                                  max(0, min(self.maximum, self.value.get() - self.minimum))) /
                                 max(1,
                                     self.maximum -
                                     self.minimum)) as u32,
                                rect.height),
                      self.core.fg);
    }

    fn event(&self, event: Event, focused: bool, redraw: &mut bool) -> bool {
        match event {
            Event::Mouse { point, left_button, .. } => {
                let mut click = false;

                let rect = self.core.rect.get();
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
