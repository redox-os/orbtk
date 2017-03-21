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
            let mut buf = [0; 4];
            font.render(&c.encode_utf8(&mut buf), 16.0).draw(self.inner, x, y, color)
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
    pub bg: Cell<Color>,
    pub running: Cell<bool>,
    mouse_point: Point,
    mouse_left: bool,
    mouse_middle: bool,
    mouse_right: bool,
    events: Vec<Event>,
    redraw: bool,
}

impl Window {
    pub fn new(rect: Rect, title: &str) -> Self {
        Window {
            inner: RefCell::new(orbclient::Window::new(rect.x, rect.y, rect.width, rect.height, title).unwrap()),
            font: orbfont::Font::find(None, None, None).ok(),
            widgets: RefCell::new(Vec::new()),
            widget_focus: Cell::new(0),
            bg: Cell::new(WINDOW_BACKGROUND),
            running: Cell::new(true),
            mouse_point: Point::new(0, 0),
            mouse_left: false,
            mouse_right: false,
            mouse_middle: false,
            events: vec![Event::Init],
            redraw: true,
        }
    }

    pub fn set_size(&self, width: u32, height: u32) {
        let mut inner = self.inner.borrow_mut();
        (*inner).set_size(width, height);
    }

    pub fn set_title(&self, title: &str) {
        let mut inner = self.inner.borrow_mut();
        (*inner).set_title(title);
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
        inner.set(self.bg.get());

        let mut renderer = WindowRenderer::new(&mut *inner, &self.font);
        for i in 0..self.widgets.borrow().len() {
            if let Some(widget) = self.widgets.borrow().get(i) {
                widget.draw(&mut renderer, self.widget_focus.get() == i);
            }
        }
    }

    pub fn step(&mut self) {
        self.drain_orbital_events();
        self.drain_events();
    }

    pub fn drain_events(&mut self) {
        for event in self.events.drain(..) {
            for i in 0..self.widgets.borrow().len() {
                if let Some(widget) = self.widgets.borrow().get(i) {
                    if widget.event(event, self.widget_focus.get() == i, &mut self.redraw) {
                        if self.widget_focus.get() != i {
                            self.widget_focus.set(i);
                            self.redraw = true;
                        }
                    }
                }
            }
        }
    }

    pub fn drain_orbital_events(&mut self) {
        for orbital_event in self.inner.borrow_mut().events() {
            match orbital_event.to_option() {
                orbclient::EventOption::Mouse(mouse_event) => {
                    self.mouse_point.x = mouse_event.x;
                    self.mouse_point.y = mouse_event.y;
                    
                    self.events.push(Event::Mouse {
                        point: self.mouse_point,
                        left_button: self.mouse_left,
                        middle_button: self.mouse_middle,
                        right_button: self.mouse_right,
                    })
                }
                orbclient::EventOption::Button(button_event) => {
                    self.mouse_left = button_event.left;
                    self.mouse_middle = button_event.middle;
                    self.mouse_right = button_event.right;

                    self.events.push(Event::Mouse {
                        point: self.mouse_point,
                        left_button: self.mouse_left,
                        middle_button: self.mouse_middle,
                        right_button: self.mouse_right,
                    })
                }
                orbclient::EventOption::Scroll(scroll_event) => {
                    self.events.push(Event::Scroll {
                        x: scroll_event.x,
                        y: scroll_event.y,
                    })
                }
                orbclient::EventOption::Key(key_event) => {
                    if key_event.pressed {
                        match key_event.scancode {
                            orbclient::K_BKSP => self.events.push(Event::Backspace),
                            orbclient::K_DEL => self.events.push(Event::Delete),
                            orbclient::K_HOME => self.events.push(Event::Home),
                            orbclient::K_END => self.events.push(Event::End),
                            orbclient::K_UP => self.events.push(Event::UpArrow),
                            orbclient::K_DOWN => self.events.push(Event::DownArrow),
                            orbclient::K_LEFT => self.events.push(Event::LeftArrow),
                            orbclient::K_RIGHT => self.events.push(Event::RightArrow),
                            _ => {
                                match key_event.character {
                                    '\0' => (),
                                    '\x1B' => (),
                                    '\n' => self.events.push(Event::Enter),
                                    _ => self.events.push(Event::Text { c: key_event.character }),
                                }
                            }
                        }
                    }
                }
                orbclient::EventOption::Quit(_quit_event) => { self.running.set(false); },
                _ => (),
            };
        }

    }

    pub fn exec(&mut self) {
        'event: while self.running.get() {
            self.drain_events();
            self.draw_if_needed();
            self.drain_orbital_events();
        }
    }

    pub fn needs_redraw(&mut self) {
        self.redraw = true;
    }

    pub fn draw_if_needed(&mut self) {
        if self.redraw {
            self.draw();
            self.redraw = false;
        }
    }
}
