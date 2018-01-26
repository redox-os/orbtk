extern crate orbfont;

use orbclient::{self, Renderer, WindowFlag};
use orbclient::color::Color;
use std::cell::{Cell, RefCell};
use std::collections::VecDeque;
use std::sync::Arc;
use std::fmt;

use super::{Event, FocusManager, KeyEventArgs, MouseButton, MouseEventArgs, MouseMoveEventArgs,
            Point, Rect, ScrollEventArgs, Widget, Handleable, EventManager};
use theme::Theme;
use traits::Resize;

pub use orbclient::Window as InnerWindow;

pub struct WindowRenderer<'a> {
    inner: &'a mut InnerWindow,
    font: &'a Option<orbfont::Font>,
}

impl<'a> WindowRenderer<'a> {
    pub fn new(inner: &'a mut InnerWindow, font: &'a Option<orbfont::Font>) -> WindowRenderer<'a> {
        WindowRenderer {
            inner: inner,
            font: font,
        }
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
            font.render(&c.encode_utf8(&mut buf), 16.0)
                .draw(self.inner, x, y, color)
        } else {
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
    inner: RefCell<InnerWindow>,
    font: Option<orbfont::Font>,
    pub widgets: RefCell<Vec<Arc<Widget>>>,
    pub running: Cell<bool>,
    pub theme: Theme,
    resize_callback: RefCell<Option<Arc<Fn(&Window, u32, u32)>>>,
    mouse_point: Point,
    mouse_left: bool,
    mouse_middle: bool,
    mouse_right: bool,
    events: VecDeque<Event>,
    redraw: bool,
    event_manager: EventManager,
}

impl Resize for Window {
    fn emit_resize(&self, width: u32, height: u32) {
        if let Some(ref resize_callback) = *self.resize_callback.borrow() {
            resize_callback(self, width, height);
        }
    }

    fn on_resize<T: Fn(&Self, u32, u32) + 'static>(&self, func: T) -> &Self {
        *self.resize_callback.borrow_mut() = Some(Arc::new(func));
        self
    }
}

impl fmt::Debug for Window {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Window (OrbTK)")?;

        for widget in &*self.widgets.borrow() {
            self.fmt_widget(f, widget, "")?;
        }

        Ok(())
    }
}

impl Window {
    pub fn new(rect: Rect, title: &str) -> Self {
        Window::new_flags(rect, title, &[])
    }

    pub fn new_flags(rect: Rect, title: &str, flags: &[WindowFlag]) -> Self {
        Window::from_inner(
            InnerWindow::new_flags(rect.x, rect.y, rect.width, rect.height, title, flags).unwrap(),
        )
    }

    fn fmt_widget(
        &self,
        f: &mut fmt::Formatter,
        widget: &Arc<Widget>,
        spacer: &str,
    ) -> fmt::Result {
        write!(f, "\n{}", spacer)?;
        widget.format(f)?;

        let mut spacer = String::from(spacer);
        spacer.push_str("|    ");

        for child in &*widget.children().borrow() {
            self.fmt_widget(f, child, &spacer)?;
        }

        Ok(())
    }

    pub fn from_inner(inner: InnerWindow) -> Self {
        let mut events = VecDeque::new();
        events.push_back(Event::Init);
        Window {
            inner: RefCell::new(inner),
            font: orbfont::Font::find(None, None, None).ok(),
            widgets: RefCell::new(Vec::new()),
            running: Cell::new(true),
            theme: Theme::new(),
            resize_callback: RefCell::new(None),
            mouse_point: Point::new(0, 0),
            mouse_left: false,
            mouse_right: false,
            mouse_middle: false,
            events: events,
            redraw: true,
            event_manager: EventManager::new(),
        }
    }

    pub fn into_inner(self) -> InnerWindow {
        self.inner.into_inner()
    }

    pub fn x(&self) -> i32 {
        let inner = self.inner.borrow();
        (*inner).x()
    }

    pub fn y(&self) -> i32 {
        let inner = self.inner.borrow();
        (*inner).y()
    }

    pub fn width(&self) -> u32 {
        let inner = self.inner.borrow();
        (*inner).width()
    }

    pub fn height(&self) -> u32 {
        let inner = self.inner.borrow();
        (*inner).height()
    }

    pub fn title(&self) -> String {
        let inner = self.inner.borrow();
        (*inner).title()
    }

    pub fn set_pos(&self, x: i32, y: i32) {
        let mut inner = self.inner.borrow_mut();
        (*inner).set_pos(x, y);
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

        // todo: set focus on first focusable widget
        // if id == 0 {
        //     self.focus_manager.request_focus(&widgets[id]);
        // }

        id
    }

    pub fn draw(&self) {
        let mut inner = self.inner.borrow_mut();
        inner.set(self.theme.color("background", &"window".into()));

        let mut renderer = WindowRenderer::new(&mut *inner, &self.font);
        for widget in self.widgets.borrow().iter() {
            self.draw_widget(&mut renderer, widget);
        }
    }

    fn draw_widget(&self, renderer: &mut Renderer, widget: &Arc<Widget>) {
        widget.update();
        widget.draw(renderer, &self.theme);

        for child in widget.children().borrow().iter() {
            self.draw_widget(renderer, child);
        }
    }

    pub fn set_theme(&mut self, theme: Theme) {
        self.theme = theme;
    }

    pub fn step(&mut self) {
        self.drain_orbital_events();
        self.drain_events();
    }

    pub fn drain_events(&mut self) {
        while let Some(event) = self.events.pop_front() {
            match event {
                Event::Resize { width, height } => {
                    self.emit_resize(width, height);
                }
                _ => (),
            }
        }

        self.redraw = self.redraw || self.event_manager.drain_events(&*self.widgets.borrow());
    }

    pub fn drain_orbital_events(&mut self) {
        for orbital_event in self.inner.borrow_mut().events() {
            match orbital_event.to_option() {
                orbclient::EventOption::Mouse(mouse_event) => {
                    self.mouse_point.x = mouse_event.x;
                    self.mouse_point.y = mouse_event.y;

                    self.event_manager
                        .push_back(Event::MouseMoveEvent(MouseMoveEventArgs::new(
                            self.mouse_point,
                        )))
                }
                orbclient::EventOption::Button(button_event) => {
                    let old_mouse_left = self.mouse_left;
                    let old_mouse_right = self.mouse_right;
                    self.mouse_left = button_event.left;
                    self.mouse_middle = button_event.middle;
                    self.mouse_right = button_event.right;

                    if self.mouse_left || self.mouse_middle || self.mouse_right {
                        let button: MouseButton = {
                            if old_mouse_left && !self.mouse_left {
                                MouseButton::Left
                            } else if old_mouse_right && !self.mouse_right {
                                MouseButton::Right
                            } else {
                                MouseButton::Middle
                            }
                        };
                        self.event_manager
                            .push_back(Event::MouseDownEvent(MouseEventArgs::new(
                                self.mouse_point,
                                button,
                            )))
                    } else {
                        let button: MouseButton = {
                            if old_mouse_left && !self.mouse_left {
                                MouseButton::Left
                            } else if old_mouse_right && !self.mouse_right {
                                MouseButton::Right
                            } else {
                                MouseButton::Middle
                            }
                        };
                        self.event_manager
                            .push_back(Event::MouseUpEvent(MouseEventArgs::new(
                                self.mouse_point,
                                button,
                            )))
                    }
                }
                orbclient::EventOption::Scroll(scroll_event) => {
                    self.event_manager.push_back(Event::Scroll {
                        x: scroll_event.x,
                        y: scroll_event.y,
                    })
                }
                orbclient::EventOption::Key(key_event) => {
                    if key_event.pressed {
                        self.event_manager.push_back(Event::KeyDownEvent(
                            KeyEventArgs::from_orbital_key_event(key_event),
                        ));
                    } else {
                        self.event_manager.push_back(Event::KeyUpEvent(
                            KeyEventArgs::from_orbital_key_event(key_event),
                        ));
                    }
                }
                orbclient::EventOption::Resize(resize_event) => {
                    self.redraw = true;
                    self.events.push_back(Event::Resize {
                        width: resize_event.width,
                        height: resize_event.height,
                    });
                }
                orbclient::EventOption::Quit(_quit_event) => {
                    self.running.set(false);
                }
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

pub struct WindowBuilder<'a> {
    rect: Rect,
    title: &'a str,
    font: Option<orbfont::Font>,
    theme: Option<Theme>,
    flags: Option<&'a [WindowFlag]>,
}

impl<'a> WindowBuilder<'a> {
    pub fn new(rect: Rect, title: &'a str) -> Self {
        WindowBuilder {
            rect: rect,
            title: title,
            font: orbfont::Font::find(None, None, None).ok(),
            theme: None,
            flags: None,
        }
    }

    pub fn font(mut self, font: orbfont::Font) -> Self {
        self.font = Some(font);
        self
    }

    pub fn theme(mut self, theme: Theme) -> Self {
        self.theme = Some(theme);
        self
    }

    pub fn flags(mut self, flags: &'a [WindowFlag]) -> Self {
        self.flags = Some(flags);
        self
    }

    pub fn build(self) -> Window {
        let (rect, title, font) = (self.rect, self.title, self.font);

        let flags = match self.flags {
            Some(flags) => flags,
            None => &[],
        };

        let inner =
            InnerWindow::new_flags(rect.x, rect.y, rect.width, rect.height, title, flags).unwrap();

        let theme = match self.theme {
            Some(theme) => theme,
            None => Theme::new(),
        };

        let mut events = VecDeque::new();
        events.push_back(Event::Init);

        Window {
            inner: RefCell::new(inner),
            font: font,
            widgets: RefCell::new(Vec::new()),
            running: Cell::new(true),
            theme: theme,
            resize_callback: RefCell::new(None),
            mouse_point: Point::new(0, 0),
            mouse_left: false,
            mouse_right: false,
            mouse_middle: false,
            events: events,
            redraw: true,
            event_manager: EventManager::new(),
        }
    }
}
