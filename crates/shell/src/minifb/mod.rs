//! This module contains a platform specific implementation of the window shell.

use std::{
    cell::RefCell,
    char,
    rc::Rc,
    sync::mpsc::{channel, Receiver, Sender},
    time::Duration,
};

pub use super::native::*;

use minifb;

#[cfg(not(target_os = "redox"))]
use raw_window_handle::{HasRawWindowHandle, RawWindowHandle};

use crate::{prelude::*, render::*, utils::*};

pub fn initialize() {}

fn key_event_helper_down<A>(key: &mut KeyHelper, adapter: &mut A, window: &minifb::Window)
where
    A: WindowAdapter,
{
    let key_repeat = match key.1 {
        minifb::Key::Left
        | minifb::Key::Right
        | minifb::Key::Up
        | minifb::Key::Down
        | minifb::Key::Backspace
        | minifb::Key::Delete => minifb::KeyRepeat::Yes,
        _ => minifb::KeyRepeat::No,
    };

    if window.is_key_pressed(key.1, key_repeat) {
        adapter.key_event(KeyEvent {
            key: key.2,
            state: ButtonState::Down,
            text: String::default(),
        });
    }
}

fn key_event_helper_up<A>(key: &mut KeyHelper, adapter: &mut A, window: &minifb::Window)
where
    A: WindowAdapter,
{
    if window.is_key_released(key.1) {
        adapter.key_event(KeyEvent {
            key: key.2,
            state: ButtonState::Up,
            text: String::default(),
        });
    }
}

fn unicode_to_key_event(uni_char: u32) -> Option<KeyEvent> {
    let mut text = String::new();

    let key = if let Some(character) = char::from_u32(uni_char) {
        text = character.to_string();
        Key::from(character)
    } else {
        Key::Unknown
    };

    if key == Key::Up
        || key == Key::Down
        || key == Key::Left
        || key == Key::Right
        || key == Key::Backspace
        || key == Key::Control
        || key == Key::Home
        || key == Key::Escape
        || key == Key::Delete
        || key == Key::Unknown
    {
        return None;
    }

    Some(KeyEvent {
        key,
        state: ButtonState::Down,
        text,
    })
}

struct KeyInputCallBack {
    key_events: Rc<RefCell<Vec<KeyEvent>>>,
}

impl minifb::InputCallback for KeyInputCallBack {
    fn add_char(&mut self, uni_char: u32) {
        if let Some(key_event) = unicode_to_key_event(uni_char) {
            self.key_events.borrow_mut().push(key_event);
        }
    }
}

struct KeyHelper(bool, minifb::Key, Key);

/// Concrete implementation of the window shell.
pub struct WindowShell<A>
where
    A: WindowAdapter,
{
    window: minifb::Window,
    render_context_2_d: RenderContext2D,
    adapter: A,
    mouse_pos: (f32, f32),
    button_down: (bool, bool, bool),
    window_size: (usize, usize),
    key_events: Rc<RefCell<Vec<KeyEvent>>>,
    // todo: temp solution
    key_backspace: KeyHelper,
    key_delete: KeyHelper,
    key_left: KeyHelper,
    key_right: KeyHelper,
    key_up: KeyHelper,
    key_down: KeyHelper,
    key_enter: KeyHelper,
    key_control: KeyHelper,
    key_control_right: KeyHelper,
    key_shift_l: KeyHelper,
    key_shift_r: KeyHelper,
    key_alt: KeyHelper,
    key_alt_r: KeyHelper,
    key_escape: KeyHelper,
    key_home: KeyHelper,
    key_a: KeyHelper,
    key_c: KeyHelper,
    key_v: KeyHelper,
    key_x: KeyHelper,
    update: bool,
    running: bool,
    active: bool,
    request_receiver: Receiver<ShellRequest>,
    request_sender: Sender<ShellRequest>,
}

#[cfg(not(target_os = "redox"))]
unsafe impl<A> HasRawWindowHandle for WindowShell<A>
where
    A: WindowAdapter,
{
    fn raw_window_handle(&self) -> RawWindowHandle {
        self.window.raw_window_handle()
    }
}

impl<A> WindowShell<A>
where
    A: WindowAdapter,
{
    /// Creates a new window shell with an adapter.
    pub fn new(
        window: minifb::Window,
        adapter: A,
        key_events: Rc<RefCell<Vec<KeyEvent>>>,
    ) -> WindowShell<A> {
        let size = window.get_size();
        let render_context_2_d = RenderContext2D::new(size.0 as f64, size.1 as f64);
        let (request_sender, request_receiver) = channel();

        WindowShell {
            window,
            // window_size: window_size,
            // mouse_buttons: (false, false, false),
            // mouse_position: Point::default(),
            render_context_2_d,
            adapter,
            mouse_pos: (0.0, 0.0),
            window_size: size,
            button_down: (false, false, false),
            key_events,
            key_backspace: KeyHelper(false, minifb::Key::Backspace, Key::Backspace),
            key_left: KeyHelper(false, minifb::Key::Left, Key::Left),
            key_right: KeyHelper(false, minifb::Key::Right, Key::Right),
            key_up: KeyHelper(false, minifb::Key::Up, Key::Up),
            key_down: KeyHelper(false, minifb::Key::Down, Key::Down),
            key_delete: KeyHelper(false, minifb::Key::Delete, Key::Delete),
            key_enter: KeyHelper(false, minifb::Key::Enter, Key::Enter),
            key_control: KeyHelper(false, minifb::Key::LeftCtrl, Key::Control),
            key_control_right: KeyHelper(false, minifb::Key::RightCtrl, Key::Control),
            key_shift_l: KeyHelper(false, minifb::Key::LeftShift, Key::ShiftL),
            key_shift_r: KeyHelper(false, minifb::Key::RightShift, Key::ShiftR),
            key_alt: KeyHelper(false, minifb::Key::LeftAlt, Key::Alt),
            key_alt_r: KeyHelper(false, minifb::Key::RightAlt, Key::Alt),
            key_escape: KeyHelper(false, minifb::Key::Escape, Key::Escape),
            key_home: KeyHelper(false, minifb::Key::Home, Key::Home),
            key_a: KeyHelper(false, minifb::Key::A, Key::A(false)),
            key_c: KeyHelper(false, minifb::Key::C, Key::C(false)),
            key_v: KeyHelper(false, minifb::Key::V, Key::V(false)),
            key_x: KeyHelper(false, minifb::Key::X, Key::X(false)),
            running: true,
            update: true,
            active: false,
            request_receiver,
            request_sender,
        }
    }

    /// Gets if the shell is running.
    pub fn running(&self) -> bool {
        self.running
    }

    /// Gets a a new sender to send request to the window shell.
    pub fn request_sender(&self) -> Sender<ShellRequest> {
        self.request_sender.clone()
    }

    /// Sets the background color of the window.
    pub fn set_background_color(&mut self, red: u8, green: u8, blue: u8) {
        self.window
            .set_background_color(red as usize, green as usize, blue as usize);
    }

    /// Sets running.
    pub fn set_running(&mut self, running: bool) {
        self.running = running;
    }

    /// Get if the shell should be updated.
    pub fn update(&self) -> bool {
        self.update
    }

    /// Sets update.
    pub fn set_update(&mut self, update: bool) {
        self.update = update;
    }

    /// Gets the shell adapter.
    pub fn adapter(&mut self) -> &mut A {
        &mut self.adapter
    }

    /// Gets the render ctx 2D.
    pub fn render_context_2_d(&mut self) -> &mut RenderContext2D {
        &mut self.render_context_2_d
    }

    fn drain_events(&mut self) {
        // mouse move
        if let Some(pos) = self.window.get_mouse_pos(minifb::MouseMode::Discard) {
            if (pos.0.floor(), pos.1.floor()) != self.mouse_pos {
                self.adapter.mouse(pos.0 as f64, pos.1 as f64);
                self.mouse_pos = (pos.0.floor(), pos.1.floor());
            }
        }

        // mouse
        let left_button_down = self.window.get_mouse_down(minifb::MouseButton::Left);
        let middle_button_down = self.window.get_mouse_down(minifb::MouseButton::Middle);
        let right_button_down = self.window.get_mouse_down(minifb::MouseButton::Right);

        if self.active != self.window.is_active() {
            self.adapter.active(self.window.is_active());
            self.active = self.window.is_active();
        }

        if left_button_down != self.button_down.0 {
            if left_button_down {
                self.push_mouse_event(true, MouseButton::Left);
            } else {
                self.push_mouse_event(false, MouseButton::Left);
            }
            self.button_down.0 = left_button_down;
        }

        if middle_button_down != self.button_down.1 {
            if middle_button_down {
                self.push_mouse_event(true, MouseButton::Middle);
            } else {
                self.push_mouse_event(false, MouseButton::Middle);
            }
            self.button_down.1 = middle_button_down;
        }

        if right_button_down != self.button_down.2 {
            if right_button_down {
                self.push_mouse_event(true, MouseButton::Right);
            } else {
                self.push_mouse_event(false, MouseButton::Right);
            }
            self.button_down.2 = right_button_down;
        }

        // scroll
        if let Some(delta) = self.window.get_scroll_wheel() {
            self.adapter.scroll(delta.0 as f64, delta.1 as f64);
        }

        // key
        while let Some(event) = self.key_events.borrow_mut().pop() {
            self.adapter.key_event(event);
        }

        key_event_helper_down(&mut self.key_backspace, &mut self.adapter, &self.window);
        key_event_helper_down(&mut self.key_delete, &mut self.adapter, &self.window);
        key_event_helper_down(&mut self.key_left, &mut self.adapter, &self.window);
        key_event_helper_down(&mut self.key_right, &mut self.adapter, &self.window);
        key_event_helper_down(&mut self.key_up, &mut self.adapter, &self.window);
        key_event_helper_down(&mut self.key_down, &mut self.adapter, &self.window);
        key_event_helper_down(&mut self.key_enter, &mut self.adapter, &self.window);
        key_event_helper_down(&mut self.key_control, &mut self.adapter, &self.window);
        key_event_helper_down(&mut self.key_control_right, &mut self.adapter, &self.window);
        key_event_helper_down(&mut self.key_shift_l, &mut self.adapter, &self.window);
        key_event_helper_down(&mut self.key_shift_r, &mut self.adapter, &self.window);
        key_event_helper_down(&mut self.key_alt, &mut self.adapter, &self.window);
        key_event_helper_down(&mut self.key_alt_r, &mut self.adapter, &self.window);
        key_event_helper_down(&mut self.key_escape, &mut self.adapter, &self.window);
        key_event_helper_down(&mut self.key_home, &mut self.adapter, &self.window);

        key_event_helper_up(&mut self.key_backspace, &mut self.adapter, &self.window);
        key_event_helper_up(&mut self.key_delete, &mut self.adapter, &self.window);
        key_event_helper_up(&mut self.key_left, &mut self.adapter, &self.window);
        key_event_helper_up(&mut self.key_right, &mut self.adapter, &self.window);
        key_event_helper_up(&mut self.key_up, &mut self.adapter, &self.window);
        key_event_helper_up(&mut self.key_down, &mut self.adapter, &self.window);
        key_event_helper_up(&mut self.key_enter, &mut self.adapter, &self.window);
        key_event_helper_up(&mut self.key_control, &mut self.adapter, &self.window);
        key_event_helper_up(&mut self.key_control_right, &mut self.adapter, &self.window);
        key_event_helper_up(&mut self.key_shift_l, &mut self.adapter, &self.window);
        key_event_helper_up(&mut self.key_shift_r, &mut self.adapter, &self.window);
        key_event_helper_up(&mut self.key_alt, &mut self.adapter, &self.window);
        key_event_helper_up(&mut self.key_alt_r, &mut self.adapter, &self.window);
        key_event_helper_up(&mut self.key_escape, &mut self.adapter, &self.window);
        key_event_helper_up(&mut self.key_home, &mut self.adapter, &self.window);

        key_event_helper_down(&mut self.key_a, &mut self.adapter, &self.window);
        key_event_helper_down(&mut self.key_c, &mut self.adapter, &self.window);
        key_event_helper_down(&mut self.key_v, &mut self.adapter, &self.window);
        key_event_helper_down(&mut self.key_x, &mut self.adapter, &self.window);
        key_event_helper_up(&mut self.key_a, &mut self.adapter, &self.window);
        key_event_helper_up(&mut self.key_c, &mut self.adapter, &self.window);
        key_event_helper_up(&mut self.key_v, &mut self.adapter, &self.window);
        key_event_helper_up(&mut self.key_x, &mut self.adapter, &self.window);

        // resize
        if self.window_size != self.window.get_size() {
            self.window_size = self.window.get_size();
            self.render_context_2_d
                .resize(self.window_size.0 as f64, self.window_size.1 as f64);
            self.adapter
                .resize(self.window_size.0 as f64, self.window_size.1 as f64);
        }

        // receive request
        let mut update = self.update();

        for request in self.request_receiver.try_iter() {
            if update {
                break;
            }

            match request {
                ShellRequest::Update => {
                    update = true;
                }
            }
        }

        self.set_update(update);
    }

    fn push_mouse_event(&mut self, pressed: bool, button: MouseButton) {
        let state = if pressed {
            ButtonState::Down
        } else {
            ButtonState::Up
        };

        self.adapter.mouse_event(MouseEvent {
            x: self.mouse_pos.0 as f64,
            y: self.mouse_pos.1 as f64,
            button,
            state,
        });
    }

    pub fn flip(&mut self) -> bool {
        if let Some(data) = self.render_context_2_d.data() {
            let _ = self
                .window
                .update_with_buffer(data, self.window_size.0, self.window_size.1);
            CONSOLE.time_end("render");
            return true;
        }

        false
    }
}

impl<A> Drop for WindowShell<A>
where
    A: WindowAdapter,
{
    fn drop(&mut self) {}
}

/// Implementation of the OrbClient based shell runner.
pub struct ShellRunner<A>
where
    A: WindowAdapter,
{
    pub window_shell: Rc<RefCell<WindowShell<A>>>,
    pub updater: Box<dyn Updater>,
}

impl<A> ShellRunner<A>
where
    A: WindowAdapter,
{
    pub fn run(&mut self) {
        loop {
            if !self.window_shell.borrow().running() || !self.window_shell.borrow().window.is_open()
            {
                break;
            }

            // CONSOLE.time("complete run");

            self.updater.update();

            if self.window_shell.borrow_mut().update() {
                self.window_shell.borrow_mut().set_update(false);
            }

            if !self.window_shell.borrow_mut().flip() {
                self.window_shell.borrow_mut().window.update();
            }

            self.window_shell.borrow_mut().drain_events();

            // CONSOLE.time_end("complete run");
        }
    }
}

/// Constructs the window shell
pub struct WindowBuilder<A>
where
    A: WindowAdapter,
{
    title: String,

    resizeable: bool,

    always_on_top: bool,

    borderless: bool,

    bounds: Rectangle,

    adapter: A,
}

impl<A> WindowBuilder<A>
where
    A: WindowAdapter,
{
    /// Create a new window builder with the given adapter.
    pub fn new(adapter: A) -> Self {
        WindowBuilder {
            adapter,
            title: String::default(),
            borderless: false,
            resizeable: false,
            always_on_top: false,
            bounds: Rectangle::default(),
        }
    }

    /// Sets the title.
    pub fn title(mut self, title: impl Into<String>) -> Self {
        self.title = title.into();
        self
    }

    /// Sets borderless.
    pub fn borderless(mut self, borderless: bool) -> Self {
        self.borderless = borderless;
        self
    }

    /// Sets resizeable.
    pub fn resizeable(mut self, resizeable: bool) -> Self {
        self.resizeable = resizeable;
        self
    }

    /// Sets always_on_top.
    pub fn always_on_top(mut self, always_on_top: bool) -> Self {
        self.always_on_top = always_on_top;
        self
    }

    /// Sets the bounds.
    pub fn bounds(mut self, bounds: impl Into<Rectangle>) -> Self {
        self.bounds = bounds.into();
        self
    }

    /// Builds the window shell.
    pub fn build(self) -> WindowShell<A> {
        let window_options = minifb::WindowOptions {
            resize: self.resizeable,
            topmost: self.always_on_top,
            borderless: self.borderless,
            title: !self.borderless,
            scale_mode: minifb::ScaleMode::UpperLeft,
            ..Default::default()
        };

        let mut window = minifb::Window::new(
            self.title.as_str(),
            self.bounds.width as usize,
            self.bounds.height as usize,
            window_options,
        )
        .unwrap_or_else(|e| {
            panic!("{}", e);
        });

        // Limit to max ~60 fps update rate
        window.limit_update_rate(Some(Duration::from_micros(64000)));

        let key_events = Rc::new(RefCell::new(vec![]));

        window.set_input_callback(Box::new(KeyInputCallBack {
            key_events: key_events.clone(),
        }));

        window.set_position(self.bounds.x as isize, self.bounds.y as isize);

        WindowShell::new(window, self.adapter, key_events)
    }
}
