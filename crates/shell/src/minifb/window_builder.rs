use std::{cell::RefCell, char, collections::HashMap, rc::Rc, sync::mpsc, time::Duration};

use minifb;

use super::{KeyState, MouseState, Shell, Window, WindowState};
use crate::{
    event::{ButtonState, Key, KeyEvent},
    render::RenderContext2D,
    utils::Rectangle,
    WindowRequest,
    WindowSettings,
    window_adapter::WindowAdapter,
};

/// The `WindowBuilder` is used to construct a window shell for the minifb backend.
pub struct WindowBuilder<'a, A>
where
    A: WindowAdapter,
{
    shell: &'a mut Shell<A>,
    adapter: A,
    title: String,
    resizeable: bool,
    always_on_top: bool,
    borderless: bool,
    fonts: HashMap<String, &'static [u8]>,
    bounds: Rectangle,
    request_receiver: Option<mpsc::Receiver<WindowRequest>>
}

impl<'a, A> WindowBuilder<'a, A>
where
    A: WindowAdapter,
{
    /// Creates a new window builder.
    pub fn new(shell: &'a mut Shell<A>, adapter: A) -> Self {
        WindowBuilder {
            shell,
            adapter,
            title: String::default(),
            resizeable: false,
            always_on_top: false,
            borderless: false,
            fonts: HashMap::new(),
            bounds: Rectangle::new(0.0, 0.0, 100.0, 75.0),
            request_receiver: None,
        }
    }

    /// Creates the window builder from a settings object.
    pub fn from_settings(settings: WindowSettings, shell: &'a mut Shell<A>, adapter: A) -> Self {
        WindowBuilder {
            shell,
            adapter,
            title: settings.title,
            resizeable: settings.resizeable,
            always_on_top: settings.always_on_top,
            borderless: settings.borderless,
            fonts: settings.fonts,
            bounds: Rectangle::new(settings.position.0, settings.position.1, settings.size.0, settings.size.1),
            request_receiver: None
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

    /// Registers a new font with family key.
    pub fn font(mut self, family: impl Into<String>, font_file: &'static [u8]) -> Self {
        self.fonts.insert(family.into(), font_file);
        self
    }

    /// Register a window request receiver to communicate with the window shell from outside.
    pub fn request_receiver(mut self, request_receiver: mpsc::Receiver<WindowRequest>) -> Self {
        self.request_receiver = Some(request_receiver);
        self
    }

    /// Builds the window shell and add it to the application `Shell`.
    pub fn build(self) {
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

        let mut render_context = RenderContext2D::new(self.bounds.width, self.bounds.height);

        for (family, font) in self.fonts {
            render_context.register_font(&family, font);
        }

        self.shell.window_shells.push(Window::new(
            window,
            self.adapter,
            render_context,
            self.request_receiver,
            WindowState::default(),
            MouseState::default(),
            true,
            true,
            false,
            vec![
                KeyState::new(minifb::Key::Backspace, Key::Backspace),
                KeyState::new(minifb::Key::Left, Key::Left),
                KeyState::new(minifb::Key::Right, Key::Right),
                KeyState::new(minifb::Key::Up, Key::Up),
                KeyState::new(minifb::Key::Down, Key::Down),
                KeyState::new(minifb::Key::Delete, Key::Delete),
                KeyState::new(minifb::Key::Enter, Key::Enter),
                KeyState::new(minifb::Key::LeftCtrl, Key::Control),
                KeyState::new(minifb::Key::RightCtrl, Key::Control),
                KeyState::new(minifb::Key::LeftShift, Key::ShiftL),
                KeyState::new(minifb::Key::RightShift, Key::ShiftR),
                KeyState::new(minifb::Key::LeftAlt, Key::Alt),
                KeyState::new(minifb::Key::RightAlt, Key::Alt),
                KeyState::new(minifb::Key::Escape, Key::Escape),
                KeyState::new(minifb::Key::Home, Key::Home),
                KeyState::new(minifb::Key::A, Key::A(false)),
                KeyState::new(minifb::Key::C, Key::C(false)),
                KeyState::new(minifb::Key::V, Key::V(false)),
                KeyState::new(minifb::Key::X, Key::X(false)),
            ],
            key_events,
        ));
    }
}

// -- Helpers --

// minifb key input helper
struct KeyInputCallBack {
    key_events: Rc<RefCell<Vec<KeyEvent>>>,
}

impl KeyInputCallBack {
    fn uni_char_to_key_event(&mut self, uni_char: u32) {
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
            return;
        }

        self.key_events.borrow_mut().push(KeyEvent {
            key,
            state: ButtonState::Down,
            text,
        });
    }
}

impl minifb::InputCallback for KeyInputCallBack {
    fn add_char(&mut self, uni_char: u32) {
        self.uni_char_to_key_event(uni_char);
    }
}

// -- Helpers --
