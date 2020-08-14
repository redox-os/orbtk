use std::{cell::RefCell, rc::Rc, sync::mpsc};

use super::{KeyState, MouseState, WindowState};
use crate::{
    event::{ButtonState, KeyEvent, MouseButton, MouseEvent},
    render::RenderContext2D,
    window_adapter::WindowAdapter,
    WindowRequest,
};

use orbclient::Renderer;

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
    window_state: WindowState,
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
        Window {
            window,
            adapter,
            render_context,
            request_receiver,
            window_state: WindowState::default(),
            mouse: MouseState::default(),
            update: true,
            redraw: true,
            close: false,
        }
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

    /// Drain events and propagate the events to the adapter.
    pub fn drain_events(&mut self) {
        // self.window.update();
        for _event in self.window.events() {}
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

                self.window
                    .data_mut()
                    .clone_from_slice(color_data.as_slice());

                self.window.sync();
                // CONSOLE.time_end("render");
                self.redraw = false;
                //super::CONSOLE.time_end("complete");
            }
        }
    }
}
