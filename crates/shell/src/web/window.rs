use std::sync::mpsc;

use stdweb::{
    js,
    traits::*,
    unstable::TryInto,
    web::{document, event, html_element::CanvasElement, window, CanvasRenderingContext2d},
};

use super::EventState;
use crate::{
    event::{ButtonState, Key, KeyEvent, MouseButton, MouseEvent},
    render::RenderContext2D,
    window_adapter::WindowAdapter,
    WindowRequest,
};

use orbtk_utils::Point;

/// Represents a wrapper for a web window. It handles events, propagate them to
/// the window adapter and handles the update and render pipeline.
pub struct Window<A>
where
    A: WindowAdapter,
{
    adapter: A,
    render_context: RenderContext2D,
    request_receiver: Option<mpsc::Receiver<WindowRequest>>,
    event_state: EventState,
    canvas: CanvasElement,
    old_canvas: Option<CanvasElement>,
    update: bool,
    redraw: bool,
    close: bool,
}

impl<A> Window<A>
where
    A: WindowAdapter,
{
    pub fn new(
        adapter: A,
        render_context: RenderContext2D,
        request_receiver: Option<mpsc::Receiver<WindowRequest>>,
        event_state: EventState,
        canvas: CanvasElement,
    ) -> Self {
        let mut adapter = adapter;

        let web_handle = raw_window_handle::web::WebHandle {
            id: 1,
            ..raw_window_handle::web::WebHandle::empty()
        };

        adapter.set_raw_window_handle(raw_window_handle::RawWindowHandle::Web(web_handle));

        Window {
            adapter,
            render_context,
            request_receiver,
            event_state,
            canvas,
            old_canvas: None,
            update: true,
            redraw: true,
            close: false,
        }
    }
}

unsafe impl<A> raw_window_handle::HasRawWindowHandle for Window<A>
where
    A: WindowAdapter,
{
    fn raw_window_handle(&self) -> raw_window_handle::RawWindowHandle {
        let web_handle = raw_window_handle::web::WebHandle {
            id: 1,
            ..raw_window_handle::web::WebHandle::empty()
        };

        raw_window_handle::RawWindowHandle::Web(web_handle)
    }
}

impl<A> Window<A>
where
    A: WindowAdapter,
{
    /// Check if the window is open.
    pub fn is_open(&self) -> bool {
        true
    }

    /// Updates the clipboard.
    pub fn update_clipboard(&mut self) {
        // todo
    }

    /// Drain events and propagate the events to the adapter.
    pub fn drain_events(&mut self) {
        while let Some(event) = self.event_state.mouse_move_events.borrow_mut().pop() {
            self.adapter
                .mouse(event.client_x() as f64, event.client_y() as f64);
            self.update = true;
        }

        while let Some(event) = self.event_state.mouse_down_events.borrow_mut().pop() {
            self.adapter.mouse_event(MouseEvent {
                position: Point::new(event.client_x() as f64, event.client_y() as f64),
                button: get_mouse_button(event.button()),
                state: ButtonState::Down,
            });
            self.update = true;
        }

        while let Some(event) = self.event_state.mouse_up_events.borrow_mut().pop() {
            self.adapter.mouse_event(MouseEvent {
                position: Point::new(event.client_x() as f64, event.client_y() as f64),
                button: get_mouse_button(event.button()),
                state: ButtonState::Up,
            });
            self.update = true;
        }

        while let Some(event) = self.event_state.scroll_events.borrow_mut().pop() {
            self.adapter.scroll(event.delta_x(), event.delta_y());
            self.update = true;
        }

        // TODO: cleanup given temporary solution to map touch events to mouse event
        while let Some(event) = self.event_state.touch_start_events.borrow_mut().pop() {
            self.adapter.mouse_event(MouseEvent {
                position: Point::new(
                    event.changed_touches()[0].client_x() as f64,
                    event.changed_touches()[0].client_y() as f64,
                ),
                button: MouseButton::Left,
                state: ButtonState::Down,
            });
            self.update = true;
        }

        while let Some(event) = self.event_state.touch_end_events.borrow_mut().pop() {
            self.adapter.mouse_event(MouseEvent {
                position: Point::new(
                    event.changed_touches()[0].client_x() as f64,
                    event.changed_touches()[0].client_y() as f64,
                ),
                button: MouseButton::Left,
                state: ButtonState::Up,
            });

            self.update = true;
        }

        while let Some(event) = self.event_state.touch_move_events.borrow_mut().pop() {
            self.adapter.mouse(
                event.changed_touches()[0].client_x() as f64,
                event.changed_touches()[0].client_y() as f64,
            );
            self.update = true;
        }

        while let Some(event) = self.event_state.key_down_events.borrow_mut().pop() {
            let key = get_key(event.code().as_str(), event.key());

            self.adapter.text_input(key.1.clone());

            self.adapter.key_event(KeyEvent {
                key: key.0,
                state: ButtonState::Down,
                text: key.1,
            });
            self.update = true;
        }

        while let Some(event) = self.event_state.key_up_events.borrow_mut().pop() {
            let key = get_key(event.code().as_str(), event.key());

            self.adapter.key_event(KeyEvent {
                key: key.0,
                state: ButtonState::Up,
                text: key.1,
            });
            self.update = true;
        }

        while let Some(_) = self.event_state.resize_events.borrow_mut().pop() {
            let window_size = (
                window().inner_width() as f64,
                window().inner_height() as f64,
            );

            let canvas: CanvasElement = document()
                .create_element("canvas")
                .unwrap()
                .try_into()
                .unwrap();

            canvas.set_width(window_size.0 as u32);
            canvas.set_height(window_size.1 as u32);

            js! {
                document.body.style.padding = 0;
                document.body.style.margin = 0;
                @{&canvas}.style.display = "block";
                @{&canvas}.style.margin = "0";
            }

            let device_pixel_ratio = window().device_pixel_ratio();
            let ctx: CanvasRenderingContext2d = canvas.get_context().unwrap();

            let backing_store_ratio = js! {
                var ctx = @{&ctx};
                 return ctx.webkitBackingStorePixelRatio ||
                     ctx.mozBackingStorePixelRatio ||
                     ctx.msBackingStorePixelRatio ||
                     ctx.oBackingStorePixelRatio ||
                     ctx.backingStorePixelRatio || 1;
            };

            let ratio: f64 = js! {
                return @{&device_pixel_ratio} / @{&backing_store_ratio};
            }
            .try_into()
            .unwrap();

            if device_pixel_ratio != backing_store_ratio {
                let old_width = canvas.width();
                let old_height = canvas.height();
                canvas.set_width((old_width as f64 * ratio) as u32);
                canvas.set_height((old_height as f64 * ratio) as u32);

                js! {
                    @{&canvas}.style.width = @{&old_width} + "px";
                    @{&canvas}.style.height = @{&old_height} + "px";
                }

                ctx.scale(ratio, ratio);
            }

            self.render_context.set_canvas_render_context_2d(ctx);
            self.adapter.resize(window_size.0, window_size.1);
            self.old_canvas = Some(self.canvas.clone());
            self.canvas = canvas;
            self.update = true;
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
                        document().set_title(title.as_str());
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
        if !self.redraw || !self.old_canvas.is_some() {
            return;
        }

        document()
            .body()
            .unwrap()
            .replace_child(&self.canvas, self.old_canvas.as_ref().unwrap())
            .expect("Could not open document");

        self.old_canvas = None;
        self.redraw = false;
    }
}

// -- Helpers --

fn get_mouse_button(button: event::MouseButton) -> MouseButton {
    match button {
        event::MouseButton::Wheel => MouseButton::Middle,
        event::MouseButton::Right => MouseButton::Right,
        _ => MouseButton::Left,
    }
}

fn get_key(code: &str, key: String) -> (Key, String) {
    let mut text = String::from("");

    let code = match code {
        "AltLeft" => Key::Alt,
        "AltRight" => Key::Alt,
        "ArrowUp" => Key::Up,
        "ArrowLeft" => Key::Left,
        "ArrowRight" => Key::Right,
        "ArrowDown" => Key::Down,
        "Backspace" => Key::Backspace,
        "CapsLock" => Key::CapsLock,
        _ => {
            text = key.clone();
            Key::from(key.chars().next().unwrap())
        }
        "ControlLeft" | "ControlRight" => Key::Control,
        "Delete" => Key::Delete,
        "Enter" => Key::Enter,
        "Escape" => Key::Escape,
        "OSLeft" | "OSRight" => Key::Home,
        "ShiftLeft" => Key::ShiftL,
        "ShiftRight" => Key::ShiftR,
        "Tab" => Key::Tab,
    };

    (code, text)
}

// -- Helpers --
