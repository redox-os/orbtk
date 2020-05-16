use std::{cell::RefCell, rc::Rc, sync::mpsc};

use glutin::{ContextWrapper, PossiblyCurrent, window};

use derive_more::Constructor;

use crate::{
    event::{ButtonState, KeyEvent, MouseButton, MouseEvent},
    render::RenderContext2D,
    window_adapter::WindowAdapter,
    WindowRequest,
};

/// Represents a wrapper for a glutin window. It handles events, propagate them to
/// the window adapter and handles the update and render pipeline.
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
    close: bool
}

impl<A> Window<A>
where
    A: WindowAdapter,
{
    /// Check if the window is open.
    pub fn is_open(&self) -> bool {
        true
    }

    /// Drain events and propagate the events to the adapter.
    pub fn drain_events(&mut self) {
       
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
            
            self.redraw = false;
        }
    }
}
