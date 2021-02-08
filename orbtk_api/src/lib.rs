//! Immediate mode user interface (ui) shell.

use dces::prelude::*;

use morph_ui_render::RenderContext;

/// Represents an immediate mode user interface (ui) shell.
///
/// It contains the Entity Component System (ECS), that organizes all the widgets of
/// the ui.
///
/// The shell also give access to a frame buffer, that represents the result of the rendered
/// ui. The frame buffer e.g. can be drawn to a window or an image.
///
/// It also handles events from outside that can be injected e.g. from a window.
#[derive(Debug)]
pub struct Shell {
    world: World,
    frame_buffer: Vec<u8>,
    mouse_position: Point,
    // 0 => left, 1 => middle, 2 => right
    mouse_button_states: (bool, bool, bool),
    size: (u32, u32),

    dummy_pos: (i32, i32),
}

impl Shell {
    /// Creates a new shell builder that is used to configure the shell.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use morph_ui_shell::Shell;
    /// // use morph_ui::shell::Shell;
    ///
    /// let shell = Shell::create().ui(|ctx| {}).build();
    /// ```
    pub fn create() -> ShellBuilder {
        ShellBuilder::new()
    }

    /// Resizes the shell (frame buffer) to the given size.
    pub fn resize(&mut self, width: u32, height: u32) {
        self.size = (width, height);
        self.frame_buffer = vec![0; width as usize * height as usize * 4];
        // todo: render
    }

    /// Injects the current position of the mouse pointer.
    pub fn mouse(&mut self, x: f64, y: f64) {
        self.mouse_position = Point::new(x, y);
        println!(
            "mouse not yet implemented, mouse position: {:?}",
            self.mouse_position
        );
    }

    /// Injects the current state of mouse buttons. If `true` the corresponding button
    /// is pressed.
    pub fn mouse_button(&mut self, left: bool, middle: bool, right: bool) {
        let (mouse_button, pressed) = {
            if left != self.mouse_button_states.0 {
                (MouseButton::Left, left)
            } else if middle != self.mouse_button_states.1 {
                (MouseButton::Middle, middle)
            } else {
                (MouseButton::Right, right)
            }
        };

        let mouse_event = MouseEvent::new(self.mouse_position, mouse_button, pressed);

        println!(
            "mouse button not yet implemented, mouse event: {:?}",
            mouse_event
        );
        self.mouse_button_states = (left, middle, right);
    }

    /// Injects the delta x and y of a scroll event.
    pub fn scroll(&mut self, delta_x: f64, delta_y: f64) {
        println!("Scroll delta (x: {}, y: {})", delta_x, delta_y);
    }

    /// Injects the state of a keyboard key if it is changed between `up` and `down`.
    pub fn key(&mut self, scan_code: u8, character: char, pressed: bool) {
        let key_event = KeyEvent::new(scan_code, character, pressed);

        if key_event.key() == Key::ArrowRight && key_event.pressed() {
            self.dummy_pos.0 += 10;
        }

        if key_event.key() == Key::ArrowLeft && key_event.pressed() {
            self.dummy_pos.0 -= 10;
        }

        if key_event.key() == Key::ArrowDown && key_event.pressed() {
            self.dummy_pos.1 += 10;
        }

        if key_event.key() == Key::ArrowUp && key_event.pressed() {
            self.dummy_pos.1 -= 10;
        }

        println!("key not yet implemented, key event: {:?}", key_event);
    }

    /// Injects the text of a text input event.
    pub fn text_input(&mut self, _text: String) {}

    /// Quits the shell.
    pub fn quit(&mut self) {}

    /// Sets the shell as activated (focused).
    pub fn active(&mut self, active: bool) {}

    /// Injects the current position of the container e.g. a Window after its position is changed.
    pub fn moved(&mut self, x: f64, y: f64) {}

    pub fn update_and_draw(&mut self) {
        // todo handle error.
        let mut rctx = RenderContext::new(self.size.0, self.size.1).unwrap();
        rctx.set_fill_brush("blue");
        rctx.fill_rect(self.dummy_pos.0 as f64, self.dummy_pos.1 as f64, 200., 200.);

        self.frame_buffer.copy_from_slice(rctx.frame_buffer());

        // todo: iterator over all entities by name, build query with render component parts like text and text style
    }

    /// Returns a reference to the frame buffer.
    pub fn frame_buffer(&self) -> &[u8] {
        &self.frame_buffer
    }

    /// Returns a mutable reference to the frame buffer.
    pub fn frame_buffer_mut(&mut self) -> &mut [u8] {
        &mut self.frame_buffer
    }
}

/// Shell builder is used to configure a shell and create it.
#[derive(Debug)]
pub struct ShellBuilder {
    world: World,
    size: (u32, u32),
}

impl ShellBuilder {
    /// Creates a new shell builder.
    fn new() -> Self {
        ShellBuilder {
            world: World::default(),
            size: (0, 0),
        }
    }

    /// Builder method that is used to specify the shell size with width and height.
    pub fn size(mut self, width: u32, height: u32) -> Self {
        self.size = (width, height);
        self
    }

    /// Builder method to define the ui of the shell.
    ///
    /// An ui can only be set once. If the method is multiple called, the last set ui will be used.
    pub fn ui<F: Fn(&mut BuildContext)>(mut self, builder: F) -> Self {
        let mut ctx = BuildContext::new(&mut self.world);
        builder(&mut ctx);
        self
    }

    /// Creates a new shell with the given builder settings.
    pub fn build(self) -> Shell {
        Shell {
            world: self.world,
            frame_buffer: vec![0; self.size.0 as usize * self.size.1 as usize * 4],
            size: self.size,
            mouse_position: Point::default(),
            mouse_button_states: (false, false, false),
            dummy_pos: (0, 0),
        }
    }
}
