use super::{Color, Event, Point, Rect, Renderer, Widget};

use std::sync::Arc;
use std::cell::{Cell, RefCell};

extern crate orbclient;
extern crate orbfont;

pub struct WindowRenderer<'a> {
    inner: &'a mut orbclient::Window,
    font: &'a Option<orbfont::Font>
}

impl<'a> WindowRenderer<'a> {
    pub fn new(inner: &'a mut orbclient::Window, font: &'a Option<orbfont::Font>) -> WindowRenderer<'a> {
        WindowRenderer { inner: inner, font: font }
    }
}

impl<'a> Renderer for WindowRenderer<'a> {
    fn clear(&mut self, color: Color) {
        self.inner.set(orbclient::Color { data: color.data });
    }

    fn char(&mut self, pos: Point, c: char, color: Color) {
        if let Some(ref font) = *self.font {
            font.render(&c.to_string(), 16.0).draw(&mut self.inner, pos.x, pos.y, orbclient::Color { data: color.data })
        }else{
            self.inner.char(pos.x, pos.y, c, orbclient::Color { data: color.data });
        }
    }

    fn rect(&mut self, rect: Rect, color: Color) {
        self.inner.rect(rect.x,
                        rect.y,
                        rect.width,
                        rect.height,
                        orbclient::Color { data: color.data });
    }

    fn line(&mut self, start: Point, end: Point, color: Color) {
        self.inner.line(start.x,
                        start.y,
                        end.x,
                        end.y,
                        orbclient::Color { data: color.data });
    }
}

impl<'a> Drop for WindowRenderer<'a> {
    fn drop(&mut self) {
        self.inner.sync();
    }
}

pub struct Window {
    inner: RefCell<orbclient::Window>,
    font: Option<orbfont::Font>,
    pub widgets: RefCell<Vec<Arc<Widget>>>,
    pub widget_focus: Cell<usize>,
    pub bg: Color,
    pub running: Cell<bool>
}

impl Window {
    pub fn new(rect: Rect, title: &str) -> Self {
        Window {
            inner: RefCell::new(orbclient::Window::new(rect.x, rect.y, rect.width, rect.height, title).unwrap()),
            font: orbfont::Font::find(None, None, None).ok(),
            widgets: RefCell::new(Vec::new()),
            widget_focus: Cell::new(0),
            bg: Color::rgb(237, 233, 227),
            running: Cell::new(true),
        }
    }

    pub fn close(&self) {
        self.running.set(false);
    }

    pub fn draw(&self) {
        let mut inner = self.inner.borrow_mut();
        let mut renderer = WindowRenderer::new(&mut *inner, &self.font);
        renderer.clear(self.bg);

        for i in 0..self.widgets.borrow().len() {
            if let Some(widget) = self.widgets.borrow().get(i) {
                widget.draw(&mut renderer, self.widget_focus.get() == i);
            }
        }
    }

    pub fn exec(&self) {
        self.draw();
        'event: while self.running.get() {
            let mut events = Vec::new();

            for orbital_event in self.inner.borrow_mut().events() {
                match orbital_event.to_option() {
                    orbclient::EventOption::Mouse(mouse_event) => {
                        events.push(Event::Mouse {
                            point: Point::new(mouse_event.x, mouse_event.y),
                            left_button: mouse_event.left_button,
                            middle_button: mouse_event.middle_button,
                            right_button: mouse_event.right_button,
                        })
                    }
                    orbclient::EventOption::Key(key_event) => {
                        if key_event.pressed {
                            match key_event.scancode {
                                orbclient::K_BKSP => events.push(Event::Backspace),
                                orbclient::K_DEL => events.push(Event::Delete),
                                orbclient::K_HOME => events.push(Event::Home),
                                orbclient::K_END => events.push(Event::End),
                                orbclient::K_UP => events.push(Event::UpArrow),
                                orbclient::K_DOWN => events.push(Event::DownArrow),
                                orbclient::K_LEFT => events.push(Event::LeftArrow),
                                orbclient::K_RIGHT => events.push(Event::RightArrow),
                                _ => {
                                    match key_event.character {
                                        '\0' => (),
                                        '\x1B' => (),
                                        '\n' => events.push(Event::Enter),
                                        _ => events.push(Event::Text { c: key_event.character }),
                                    }
                                }
                            }
                        }
                    }
                    orbclient::EventOption::Quit(_quit_event) => break 'event,
                    _ => (),
                };
            }

            let mut redraw = false;
            for event in events.iter() {
                for i in 0..self.widgets.borrow().len() {
                    if let Some(widget) = self.widgets.borrow().get(i) {
                        if widget.event(*event, self.widget_focus.get() == i, &mut redraw) {
                            if self.widget_focus.get() != i {
                                self.widget_focus.set(i);
                                redraw = true;
                            }
                        }
                    }
                }
            }

            if redraw {
                self.draw();
            }
        }
    }
}
