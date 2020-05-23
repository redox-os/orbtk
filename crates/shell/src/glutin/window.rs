use std::{cell::RefCell, rc::Rc, sync::mpsc};

use glutin::{event, event_loop::ControlFlow, window, ContextWrapper, PossiblyCurrent};

use derive_more::Constructor;

use crate::{
    event::{ButtonState, KeyEvent, MouseButton, MouseEvent},
    render::RenderContext2D,
    window_adapter::WindowAdapter,
    WindowRequest,
};

/// Represents a wrapper for a glutin window. It handles events, propagate them to
/// the window adapter and handles the update and redraw pipeline.
#[derive(Constructor)]
pub struct Window<A>
where
    A: WindowAdapter,
{
    gl_context: ContextWrapper<PossiblyCurrent, window::Window>,
    adapter: A,
    render_context: RenderContext2D,
    request_receiver: Option<mpsc::Receiver<WindowRequest>>,
    update: bool,
    redraw: bool,
    close: bool,
    mouse_pos: (f64, f64),
}

impl<A> Window<A>
where
    A: WindowAdapter,
{
    /// Returns an glutin specific window id.
    pub fn id(&self) -> window::WindowId {
        self.gl_context.window().id()
    }

    /// Check if the window is open.
    pub fn is_open(&self) -> bool {
        true
    }

    /// Drain events and propagate the events to the adapter.
    pub fn drain_events(&mut self, control_flow: &mut ControlFlow, event: &event::Event<()>) {
        match event {
            event::Event::WindowEvent {
                event: event::WindowEvent::Resized(s),
                window_id,
            } => {
                if !window_id.eq(&self.id()) {
                    return;
                }
                self.adapter.resize(s.width as f64, s.height as f64);
                self.render_context.resize(s.width as f64, s.height as f64);
                self.update = true;
                *control_flow = ControlFlow::Wait;
            }
            event::Event::WindowEvent {
                event: event::WindowEvent::CloseRequested,
                window_id,
            } => {
                if !window_id.eq(&self.id()) {
                    return;
                }
                self.adapter.quit_event();
                *control_flow = ControlFlow::Exit;
            }
            event::Event::WindowEvent {
                event: event::WindowEvent::KeyboardInput { input, .. },
                // todo: implement
                ..
            } => *control_flow = ControlFlow::Wait,
            event::Event::WindowEvent {
                event: event::WindowEvent::MouseInput { state, button, .. },
                ..
            } => {
                let button = {
                    match button {
                        event::MouseButton::Left => MouseButton::Left,
                        event::MouseButton::Right => MouseButton::Right,
                        event::MouseButton::Middle => MouseButton::Middle,
                        event::MouseButton::Other(_) => MouseButton::Left,
                    }
                };

                let state = {
                    match state {
                        event::ElementState::Pressed => ButtonState::Down,
                        event::ElementState::Released => ButtonState::Up,
                    }
                };

                let mouse_pos = self.mouse_pos;

                self.adapter.mouse_event(MouseEvent {
                    x: mouse_pos.0,
                    y: mouse_pos.1,
                    button,
                    state,
                });
                self.update = true;
                self.redraw = true;
                *control_flow = ControlFlow::Wait;
            }
            event::Event::WindowEvent {
                event: event::WindowEvent::MouseWheel { delta, .. },
                window_id,
            } => {
                if !window_id.eq(&self.id()) {
                    return;
                }
                match delta {
                    event::MouseScrollDelta::LineDelta(_, _) => {}
                    event::MouseScrollDelta::PixelDelta(p) => {
                        self.adapter.scroll(p.x, p.y);
                    }
                }
                self.redraw = true;
                self.update = true;
                *control_flow = ControlFlow::Wait;
            }
            event::Event::WindowEvent {
                event: event::WindowEvent::CursorMoved { position, .. },
                window_id,
            } => {
                if !window_id.eq(&self.id()) {
                    return;
                }
                self.mouse_pos = (position.x, position.y);
                self.adapter.mouse(position.x, position.y);
                self.update = true;
                self.redraw = true;
                *control_flow = ControlFlow::Wait;
            }
            _ => *control_flow = ControlFlow::Wait,
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
                        // todo fix
                        // self.window.set_title(&title);
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
            self.gl_context.swap_buffers().unwrap();
            self.redraw = false;
        }
    }
}
