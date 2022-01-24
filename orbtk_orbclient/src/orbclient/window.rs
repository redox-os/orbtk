use std::sync::{
    atomic::{AtomicBool, Ordering},
    mpsc, Arc,
};

#[cfg(not(target_os = "redox"))]
use std::thread;

use super::MouseState;
use crate::{
    event::{ButtonState, Key, KeyEvent, MouseButton, MouseEvent},
    render::RenderContext2D,
    window_adapter::WindowAdapter,
    WindowRequest,
};

#[cfg(not(target_os = "redox"))]
use sdl2::event;

use orbclient::Renderer;

#[cfg(not(target_os = "redox"))]
use raw_window_handle::HasRawWindowHandle;

use orbtk_utils::Point;

/// Represents a wrapper structure consumed by an orbclient window.
///
/// Events are handled and propagated to the window adapter. The
/// window adapter will update corresponding entities, that are marked
/// `dirty`. If needed, the render pipeline will process those
/// entities and update the render buffer. Results are forwarded to
/// the output device.
pub struct Window<A>
where
    A: WindowAdapter,
{
    adapter: A,
    close: bool,
    has_clipboard_update: bool,
    mouse: MouseState,
    update: bool,
    redraw: Arc<AtomicBool>,
    render_context: RenderContext2D,
    request_receiver: Option<mpsc::Receiver<WindowRequest>>,
    #[cfg(not(target_os = "redox"))]
    _sdl2_sync_thread: Option<thread::JoinHandle<()>>,
    //window_state: WindowState,
    window: orbclient::Window,
}

// Internal sync method betwee OrbClient and sdl2 backend
#[cfg(not(target_os = "redox"))]
fn init_sync(
    window: &orbclient::Window,
    receiver: mpsc::Receiver<WindowRequest>,
) -> (mpsc::Receiver<WindowRequest>, thread::JoinHandle<()>) {
    // channel: sends will never block, they are asynchronous
    let (internal_sender, internal_receiver) = mpsc::channel();

    let event_sender = window.event_sender();
    let id = window.id();

    let _sdl2_sync_thread = thread::spawn(move || loop {
        for request in receiver.iter() {
            let _ = internal_sender.send(request.clone());
            if request == WindowRequest::Redraw {
                let _ = event_sender.push_event(event::Event::Window {
                    win_event: event::WindowEvent::None,
                    window_id: id,
                    timestamp: 0,
                });
            }
        }
    });

    (internal_receiver, _sdl2_sync_thread)
}

impl<A> Window<A>
where
    A: WindowAdapter,
{
    #[cfg(target_os = "redox")]
    pub fn new(
        adapter: A,
        render_context: RenderContext2D,
        request_receiver: Option<mpsc::Receiver<WindowRequest>>,
        window: orbclient::Window,
    ) -> Self {
        Window {
            adapter,
            render_context,
            request_receiver,
            // window_state: WindowState::default(),
            mouse: MouseState::default(),
            update: true,
            redraw: Arc::new(AtomicBool::new(true)),
            close: false,
            has_clipboard_update: true,
            window,
        }
    }

    #[cfg(not(target_os = "redox"))]
    pub fn new(
        adapter: A,
        render_context: RenderContext2D,
        request_receiver: Option<mpsc::Receiver<WindowRequest>>,
        window: orbclient::Window,
    ) -> Self {
        let mut adapter = adapter;
        let redraw: Arc<AtomicBool> = Arc::new(AtomicBool::new(true));

        adapter.set_raw_window_handle(window.raw_window_handle());

        let (request_receiver, _sdl2_sync_thread) = {
            if let Some(receiver) = request_receiver {
                let (rec, sync) = init_sync(&window, receiver);
                (Some(rec), Some(sync))
            } else {
                (None, None)
            }
        };

        Window {
            adapter,
            close: false,
            has_clipboard_update: true,
            mouse: MouseState::default(),
            redraw,
            render_context,
            request_receiver,
            _sdl2_sync_thread,
            update: true,
            // window_state: WindowState::default(),
            window,
        }
    }

    #[cfg(not(target_os = "redox"))]
    fn has_clipboard_update(&self) -> bool {
        self.has_clipboard_update
    }

    // todo: workaround until clipboard update events available on orbital
    #[cfg(target_os = "redox")]
    fn has_clipboard_update(&self) -> bool {
        true
    }
}

#[cfg(not(target_os = "redox"))]
// TODO: Safety: Explain why we are allowed to consume a raw window handle
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
    /// Drain events and propagate the events to the adapter.
    pub fn drain_events(&mut self) {
        for event in self.window.events() {
            match event.to_option() {
                orbclient::EventOption::Key(event) => {
                    self.push_key_event(event);
                    self.update = true;
                }
                orbclient::EventOption::TextInput(event) => {
                    self.adapter.text_input(event.character.to_string());
                    self.update = true;
                }
                orbclient::EventOption::Mouse(event) => {
                    self.mouse.mouse_pos = (event.x as f32, event.y as f32);
                    self.adapter.mouse(event.x as f64, event.y as f64);
                    self.update = true;
                }
                orbclient::EventOption::MouseRelative(_) => {}
                orbclient::EventOption::ClipboardUpdate(_) => {
                    self.has_clipboard_update = true;
                }
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
                    self.redraw.store(true, Ordering::Relaxed);
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
                orbclient::EventOption::Hover(_) => {}
            }
        }
    }

    /// Check if the window is open.
    pub fn is_open(&self) -> bool {
        !self.close
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
            if key_event.character != '\0'
                && key_event.character != '\n'
                && key_event.character != '\u{1b}'
            {
                key_event.character.to_string()
            } else {
                match key_event.scancode {
                    orbclient::K_ALT => key = Key::Alt,
                    orbclient::K_BKSP => key = Key::Backspace,
                    orbclient::K_CAPS => key = Key::CapsLock,
                    orbclient::K_CTRL => key = Key::Control,
                    orbclient::K_DEL => key = Key::Delete,
                    orbclient::K_DOWN => key = Key::Down,
                    orbclient::K_ENTER => key = Key::Enter,
                    orbclient::K_ESC => key = Key::Escape,
                    orbclient::K_HOME => {
                        key = Key::Home;
                    }
                    orbclient::K_LEFT => key = Key::Left,
                    orbclient::K_LEFT_SHIFT => key = Key::ShiftL,
                    orbclient::K_RIGHT => key = Key::Right,
                    orbclient::K_RIGHT_SHIFT => key = Key::ShiftR,
                    orbclient::K_TAB => key = Key::Tab,
                    orbclient::K_UP => key = Key::Up,
                    _ => key = Key::Unknown,
                };
                String::default()
            }
        };

        self.adapter.key_event(KeyEvent { state, key, text });
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

    /// Receives window request from the application and handles them.
    pub fn receive_requests(&mut self) {
        if let Some(request_receiver) = &self.request_receiver {
            for request in request_receiver.try_iter() {
                match request {
                    WindowRequest::Redraw => {
                        if !self.update && !self.redraw.load(Ordering::Relaxed) {
                            self.update = true;
                            self.redraw.store(true, Ordering::Relaxed)
                        }
                    }
                    WindowRequest::ChangeTitle(title) => {
                        self.window.set_title(title.as_str());
                        self.update = true;
                        self.redraw.store(true, Ordering::Relaxed)
                    }
                    WindowRequest::Close => {
                        self.close = true;
                    }
                }
            }
        }
    }

    /// Swaps the current frame buffer.
    pub fn render(&mut self) {
        if self.redraw.load(Ordering::Relaxed) {
            let bytes = self.render_context.data_u8_mut();
            let len = bytes.len() / std::mem::size_of::<orbclient::Color>();
            let color_data = unsafe {
                std::slice::from_raw_parts_mut(bytes.as_mut_ptr() as *mut orbclient::Color, len)
            };

            if color_data.len() == self.window.data().len() {
                self.window.data_mut().clone_from_slice(color_data);

                // CONSOLE.time_end("render");
                self.redraw.store(false, Ordering::Relaxed)
                //super::CONSOLE.time_end("complete");
            }
        }

        self.window.sync();
    }

    /// Triggers an update for the given window adapter.
    pub fn update(&mut self) {
        //super::CONSOLE.time("complete");
        if !self.update {
            return;
        }

        self.adapter.run(&mut self.render_context);
        self.update = false;
        self.redraw.store(true, Ordering::Relaxed)
    }
    /// Updates the clipboard.
    pub fn update_clipboard(&mut self) {
        let mut clipboard_value = if self.has_clipboard_update() {
            self.has_clipboard_update = false;
            Some(self.window.clipboard())
        } else {
            None
        };

        self.adapter.clipboard_update(&mut clipboard_value);

        if let Some(value) = clipboard_value {
            self.window.set_clipboard(value.as_str());
        }
    }
}
