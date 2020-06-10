use std::{cell::RefCell, rc::Rc, sync::mpsc};

use derive_more::Constructor;

use super::{KeyState, MouseState, WindowState, CONSOLE};
use crate::{
    event::{ButtonState, KeyEvent, MouseButton, MouseEvent},
    render::RenderContext2D,
    window_adapter::WindowAdapter,
    WindowRequest,
};

/// Represents a wrapper for a minifb window. It handles events, propagate them to
/// the window adapter and handles the update and render pipeline.
#[derive(Constructor)]
pub struct Window<A>
where
    A: WindowAdapter,
{
    window: minifb::Window,
    adapter: A,
    render_context: RenderContext2D,
    request_receiver: Option<mpsc::Receiver<WindowRequest>>,
    window_state: WindowState,
    mouse: MouseState,
    update: bool,
    redraw: bool,
    close: bool,
    key_states: Vec<KeyState>,
    key_events: Rc<RefCell<Vec<KeyEvent>>>,
}

impl<A> Window<A>
where
    A: WindowAdapter,
{
    fn push_mouse_event(&mut self, pressed: bool, button: MouseButton) {
        let state = if pressed {
            ButtonState::Down
        } else {
            ButtonState::Up
        };

        self.adapter.mouse_event(MouseEvent {
            x: self.mouse.mouse_pos.0 as f64,
            y: self.mouse.mouse_pos.1 as f64,
            button,
            state,
        });
    }

    fn push_key_down_event(&mut self, index: usize) {
        let key_repeat = match self.key_states.get(index).unwrap().minifb_key {
            minifb::Key::Left
            | minifb::Key::Right
            | minifb::Key::Up
            | minifb::Key::Down
            | minifb::Key::Backspace
            | minifb::Key::Delete => minifb::KeyRepeat::Yes,
            _ => minifb::KeyRepeat::No,
        };

        if self
            .window
            .is_key_pressed(self.key_states.get(index).unwrap().minifb_key, key_repeat)
        {
            self.adapter.key_event(KeyEvent {
                key: self.key_states.get(index).unwrap().key,
                state: ButtonState::Down,
                text: String::default(),
            });

            self.update = true;
        }
    }

    fn push_key_up_event(&mut self, index: usize) {
        if self
            .window
            .is_key_released(self.key_states.get(index).unwrap().minifb_key)
        {
            self.adapter.key_event(KeyEvent {
                key: self.key_states.get(index).unwrap().key,
                state: ButtonState::Up,
                text: String::default(),
            });

            self.update = true;
        }
    }

    /// Check if the window is open.
    pub fn is_open(&self) -> bool {
        self.window.is_open() && !self.close
    }

    /// Drain events and propagate the events to the adapter.
    pub fn drain_events(&mut self) {
        self.window.update();

        // mouse move
        if let Some(pos) = self.window.get_mouse_pos(minifb::MouseMode::Discard) {
            if (pos.0.floor(), pos.1.floor()) != self.mouse.mouse_pos {
                self.adapter.mouse(pos.0 as f64, pos.1 as f64);
                self.mouse.mouse_pos = (pos.0.floor(), pos.1.floor());
                self.update = true;
            }
        }

        // mouse buttons
        let left_button_down = self.window.get_mouse_down(minifb::MouseButton::Left);
        let middle_button_down = self.window.get_mouse_down(minifb::MouseButton::Middle);
        let right_button_down = self.window.get_mouse_down(minifb::MouseButton::Right);

        if left_button_down != self.mouse.button_left {
            if left_button_down {
                self.push_mouse_event(true, MouseButton::Left);
            } else {
                self.push_mouse_event(false, MouseButton::Left);
            }
            self.mouse.button_left = left_button_down;
            self.update = true;
        }

        if middle_button_down != self.mouse.button_middle {
            if middle_button_down {
                self.push_mouse_event(true, MouseButton::Middle);
            } else {
                self.push_mouse_event(false, MouseButton::Middle);
            }
            self.mouse.button_middle = middle_button_down;
            self.update = true;
        }

        if right_button_down != self.mouse.button_right {
            if right_button_down {
                self.push_mouse_event(true, MouseButton::Right);
            } else {
                self.push_mouse_event(false, MouseButton::Right);
            }
            self.mouse.button_right = right_button_down;
            self.update = true;
        }

        // scroll
        if let Some(delta) = self.window.get_scroll_wheel() {
            self.adapter.scroll(delta.0 as f64, delta.1 as f64);
            self.update = true;
        }

        // resize
        if self.window_state.size != self.window.get_size() {
            self.window_state.size = self.window.get_size();
            self.render_context.resize(
                self.window_state.size.0 as f64,
                self.window_state.size.1 as f64,
            );
            self.adapter.resize(
                self.window_state.size.0 as f64,
                self.window_state.size.1 as f64,
            );
            self.update = true;
        }

        if self.window_state.active != self.window.is_active() {
            self.adapter.active(self.window.is_active());
            self.window_state.active = self.window.is_active();
        }

        // keys
        while let Some(event) = self.key_events.borrow_mut().pop() {
            self.adapter.key_event(event);
            self.update = true;
        }

        for i in 0..self.key_states.len() {
            self.push_key_down_event(i);
            self.push_key_up_event(i);
        }
    }

    /// Receives window request from the application and handles them.
    pub fn receive_requests(&mut self) {
        if let Some(request_receiver) = &self.request_receiver {
            for request in request_receiver.try_iter() {
                match request {
                    WindowRequest::Redraw => {
                        self.update = true;
                        self.redraw = true;
                    }
                    WindowRequest::ChangeTitle(title) => {
                        self.window.set_title(&title);
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
                let _ = self.window.update_with_buffer(
                    data,
                    self.window_state.size.0 as usize,
                    self.window_state.size.1 as usize,
                );
                CONSOLE.time_end("render");
                self.redraw = false;
            }
        }
    }
}
