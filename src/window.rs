use orbclient::{self, Renderer};
use orbclient::color::Color;
use std::cell::{Cell, RefCell};
use std::sync::Arc;

use super::{Event, Point, Rect, Widget};
use theme::WINDOW_BACKGROUND;

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
    fn width(&self) -> u32 {
        self.inner.width()
    }

    fn height(&self) -> u32 {
        self.inner.height()
    }

    fn data(&self) -> &[Color] {
        self.inner.data()
    }

    fn data_mut(&mut self) -> &mut [Color] {
        self.inner.data_mut()
    }

    fn sync(&mut self) -> bool {
        self.inner.sync()
    }

    fn char(&mut self, x: i32, y: i32, c: char, color: Color) {
        if let Some(ref font) = *self.font {
            font.render(&c.to_string(), 16.0).draw(&mut self.inner, x, y, color)
        }else{
            self.inner.char(x, y, c, color);
        }
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
            bg: WINDOW_BACKGROUND,
            running: Cell::new(true),
        }
    }

    pub fn close(&self) {
        self.running.set(false);
    }

    pub fn add<T: Widget>(&self, widget: &Arc<T>) -> usize {
        let mut widgets = self.widgets.borrow_mut();
        let id = widgets.len();
        widgets.push(widget.clone());
        id
    }

    pub fn draw(&self) {
        let mut inner = self.inner.borrow_mut();
        inner.set(self.bg);

        let mut renderer = WindowRenderer::new(&mut *inner, &self.font);
        for i in 0..self.widgets.borrow().len() {
            if let Some(widget) = self.widgets.borrow().get(i) {
                widget.draw(&mut renderer, self.widget_focus.get() == i);
            }
        }
    }

    pub fn exec(&self) {
        let mut events = Vec::new();
        events.push(Event::Init);

        let mut redraw = true;

        'event: while self.running.get() {
            for event in events.drain(..) {
                for i in 0..self.widgets.borrow().len() {
                    if let Some(widget) = self.widgets.borrow().get(i) {
                        if widget.event(event, self.widget_focus.get() == i, &mut redraw) {
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
                redraw = false;
            }

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
        }
    }
}
