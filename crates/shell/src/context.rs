use std::sync::mpsc::Sender;

use crate::{prelude::*, render::*, utils::*};

pub struct ShellContext<'a> {
    render_context_2d: &'a mut RenderContext2D,
    request_sender: &'a Sender<ShellRequest>,
}

impl<'a> ShellContext<'a> {
    pub fn new(
        render_context_2d: &'a mut RenderContext2D,
        request_sender: &'a Sender<ShellRequest>,
    ) -> Self {
        ShellContext {
            render_context_2d,
            request_sender,
        }
    }

    pub fn render_context_2d(&mut self) -> &mut RenderContext2D {
        self.render_context_2d
    }

    pub fn request_sender(&self) -> Sender<ShellRequest> {
        self.request_sender.clone()
    }
}
