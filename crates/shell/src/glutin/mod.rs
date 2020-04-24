use std::{
    rc::Rc,
    cell::RefCell,
    sync::mpsc::{channel, Receiver, Sender},
};

pub use super::native::*;

use crate::{prelude::*, render::*, utils::*};

pub fn initialize() {

}


/// Concrete implementation of the window shell.
pub struct WindowShell<A>
where
    A: WindowAdapter,
{
    pub flip: bool,
    adapter: A,
    update: bool,
    running: bool,
    request_receiver: Receiver<ShellRequest>,
    request_sender: Sender<ShellRequest>,
    render_context_2_d: RenderContext2D
}

// unsafe impl<A> HasRawWindowHandle for WindowShell<A>
// where
//     A: WindowAdapter,
// {
//     fn raw_window_handle(&self) -> RawWindowHandle {
//         // let handle = WebHandle {
//         //     id: 0,
//         //     ..WebHandle::empty()
//         // };

//         // RawWindowHandle::N
//     }
// }

impl<A> WindowShell<A>
where
    A: WindowAdapter,
{
    /// Gets if the shell is running.
    pub fn running(&self) -> bool {
        self.running
    }

    /// Gets a a new sender to send request to the window shell.
    pub fn request_sender(&self) -> Sender<ShellRequest> {
        self.request_sender.clone()
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

    /// Sets the background color of the window.
    pub fn set_background_color(&mut self, red: u8, green: u8, blue: u8) {
   
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
       
    }

    pub fn flip(&mut self) {
      
        self.flip = false;
    }
}

/// Implementation of the OrbClient based shell runner.
pub struct ShellRunner<A>
where
    A: WindowAdapter + 'static,
{
    pub window_shell: Rc<RefCell<WindowShell<A>>>,
    pub updater: Box<dyn Updater>,
}

impl<A> ShellRunner<A>
where
    A: WindowAdapter,
{
    pub fn run(mut self) {
      
    }
}

/// Constructs the window shell
pub struct WindowBuilder<A>
where
    A: WindowAdapter,
{
    title: String,

    borderless: bool,

    resizeable: bool,

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

     /// Does nothing on web.
    pub fn always_on_top(self, always_on_top: bool) -> Self {
        self
    }

    /// Sets the bounds.
    pub fn bounds(mut self, bounds: impl Into<Rectangle>) -> Self {
        self.bounds = bounds.into();
        self
    }

    /// Builds the window shell.
    pub fn build(self) -> WindowShell<A> {
        let (request_sender, request_receiver) = channel();

        WindowShell {
            flip: false,
            update: true,
            running: true,
            request_receiver,
            request_sender,
            render_context_2_d: RenderContext2D::new(0.0, 0.0),
            adapter: self.adapter
        }
    }
}

