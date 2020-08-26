use std::sync::mpsc;

use super::MouseState;
use crate::{
    event::{ButtonState, Key, KeyEvent, MouseButton, MouseEvent},
    render::RenderContext2D,
    window_adapter::WindowAdapter,
    WindowRequest,
};

use orbclient::Renderer;
use raw_window_handle::HasRawWindowHandle;

use orbtk_utils::Point;

/// Represents a wrapper for a orbclient window. It handles events, propagate them to
/// the window adapter and handles the update and render pipeline.
pub struct Window<A>
where
    A: WindowAdapter,
{
    window: orbclient::Window,
    adapter: A,
    render_context: RenderContext2D,
    request_receiver: Option<mpsc::Receiver<WindowRequest>>,
    //window_state: WindowState,
    mouse: MouseState,
    update: bool,
    redraw: bool,
    close: bool,
}

impl<A> Window<A>
where
    A: WindowAdapter,
{
    pub fn new(
        window: orbclient::Window,
        adapter: A,
        render_context: RenderContext2D,
        request_receiver: Option<mpsc::Receiver<WindowRequest>>,
    ) -> Self {
        let mut adapter = adapter;
        adapter.set_raw_window_handle(window.raw_window_handle());

        Window {
            window,
            adapter,
            render_context,
            request_receiver,
            // window_state: WindowState::default(),
            mouse: MouseState::default(),
            update: true,
            redraw: true,
            close: false,
        }
    }
}

#[cfg(not(target_os = "redox"))]
unsafe impl<A> raw_window_handle::HasRawWindowHandle for Window<A>
where
    A: WindowAdapter,
{
    fn raw_window_handle(&self) -> raw_window_handle::RawWindowHandle {
        self.window.raw_window_handle()
    }
}

impl<A> Window<A>
where
    A: WindowAdapter,
{
    /// Check if the window is open.
    pub fn is_open(&self) -> bool {
        !self.close
    }

    fn push_mouse_event(&mut self, pressed: bool, button: MouseButton) {
        let state = if pressed {
            ButtonState::Down
        } else {
            ButtonState::Up
        };

        self.adapter.mouse_event(MouseEvent {
            position: Point::new(self.mouse.mouse_pos.0 as f64, self.mouse.mouse_pos.1 as f64),
            button,
            state,
        });
    }

    fn push_key_event(&mut self, key_event: orbclient::KeyEvent) {
        let mut key = Key::from(key_event.character);
        let state = {
            if key_event.pressed {
                ButtonState::Down
            } else {
                ButtonState::Up
            }
        };

        let text = {
            if key_event.character != '\0' && key_event.character != '\n' {
                key_event.character.to_string()
            } else {
                match key_event.scancode {
                    orbclient::K_BKSP => key = Key::Backspace,
                    orbclient::K_LEFT => key = Key::Left,
                    orbclient::K_RIGHT => key = Key::Right,
                    orbclient::K_UP => key = Key::Up,
                    orbclient::K_DOWN => key = Key::Down,
                    orbclient::K_DEL => key = Key::Delete,
                    orbclient::K_ENTER => key = Key::Enter,
                    orbclient::K_CTRL => key = Key::Control,
                    orbclient::K_LEFT_SHIFT => key = Key::ShiftL,
                    orbclient::K_RIGHT_SHIFT => key = Key::ShiftR,
                    orbclient::K_ALT => key = Key::Alt,
                    orbclient::K_ESC => key = Key::Escape,
                    orbclient::K_CAPS => key = Key::CapsLock,
                    orbclient::K_HOME => {
                        key = Key::Home;
                    }
                    _ => key = Key::Unknown,
                };
                String::default()
            }
        };

        self.adapter.key_event(KeyEvent { key, text, state });
    }

    /// Updates the clipboard.
    pub fn update_clipboard(&mut self) {
        // update clipboard
        let mut clipboard_value = Some(self.window.clipboard());

        self.adapter.clipboard_update(&mut clipboard_value);

        if let Some(value) = clipboard_value {
            self.window.set_clipboard(value.as_str());
        }
    }

    /// Drain events and propagate the events to the adapter.
    pub fn drain_events(&mut self) {
        for event in self.window.events() {
            match event.to_option() {
                orbclient::EventOption::Key(event) => {
                    self.push_key_event(event);
                    self.update = true;
                }
                orbclient::EventOption::Mouse(event) => {
                    self.mouse.mouse_pos = (event.x as f32, event.y as f32);
                    self.adapter.mouse(event.x as f64, event.y as f64);
                    self.update = true;
                }
                orbclient::EventOption::MouseRelative(_) => {}
                orbclient::EventOption::Button(event) => {
                    if event.left != self.mouse.button_left {
                        if event.left {
                            self.push_mouse_event(true, MouseButton::Left);
                        } else {
                            self.push_mouse_event(false, MouseButton::Left);
                        }
                        self.mouse.button_left = event.left;
                        self.update = true;
                    }

                    if event.middle != self.mouse.button_middle {
                        if event.middle {
                            self.push_mouse_event(true, MouseButton::Middle);
                        } else {
                            self.push_mouse_event(false, MouseButton::Middle);
                        }
                        self.mouse.button_middle = event.middle;
                        self.update = true;
                    }

                    if event.right != self.mouse.button_right {
                        if event.right {
                            self.push_mouse_event(true, MouseButton::Right);
                        } else {
                            self.push_mouse_event(false, MouseButton::Right);
                        }
                        self.mouse.button_right = event.right;
                        self.update = true;
                    }
                }
                orbclient::EventOption::Scroll(event) => {
                    self.adapter.scroll(event.x as f64, event.y as f64);
                    self.update = true;
                }
                orbclient::EventOption::Quit(_) => {
                    self.close = true;
                    self.update = true
                }
                orbclient::EventOption::Focus(_) => {}
                orbclient::EventOption::Move(_) => {}
                orbclient::EventOption::Resize(event) => {
                    self.adapter.resize(event.width as f64, event.height as f64);
                    self.render_context
                        .resize(event.width as f64, event.height as f64);
                    self.update = true;
                    self.redraw = true;
                }
                orbclient::EventOption::Screen(_) => {}
                orbclient::EventOption::Clipboard(_) => {}
                orbclient::EventOption::Drop(event) => {
                    self.window.sync_path();
                    if let Some(text) = self.window.pop_drop_content() {
                        if event.kind == orbclient::DROP_FILE {
                            self.adapter.file_drop_event(text);
                        } else {
                            self.adapter.text_drop_event(text);
                        }
                    }
                }
                orbclient::EventOption::Unknown(_) => {}
                orbclient::EventOption::None => {}
            }
        }
    }

    /// Receives window request from the application and handles them.
    pub fn receive_requests(&mut self) {
        if let Ok(result) = self.render_context.finish_receiver().try_recv() {
            if result {
                self.redraw = true;
            }
        }
        if let Some(request_receiver) = &self.request_receiver {
            for request in request_receiver.try_iter() {
                match request {
                    WindowRequest::Redraw => {
                        self.update = true;
                        self.redraw = true;
                    }
                    WindowRequest::ChangeTitle(title) => {
                        self.window.set_title(title.as_str());
                        self.update = true;
                        self.redraw = true;
                    }
                    WindowRequest::Close => {
                        self.close = true;
                    }
                }
            }
        }
    }

    /// Runs update on the adapter.
    pub fn update(&mut self) {
        //super::CONSOLE.time("complete");
        if !self.update {
            return;
        }
        self.adapter.run(&mut self.render_context);
        self.update = false;
        self.redraw = true;
    }

    /// Swaps the current frame buffer.
    pub fn render(&mut self) {
        if self.redraw {
            if let Some(data) = self.render_context.data() {
                let color_data: Vec<orbclient::Color> =
                    data.iter().map(|v| orbclient::Color { data: *v }).collect();

                if color_data.len() == self.window.data().len() {
                    self.window
                        .data_mut()
                        .clone_from_slice(color_data.as_slice());

                    // CONSOLE.time_end("render");
                    self.redraw = false;
                    //super::CONSOLE.time_end("complete");
                }
            }

            self.window.sync();
        }
    }
}
