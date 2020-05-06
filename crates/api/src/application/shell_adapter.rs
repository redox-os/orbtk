use dces::prelude::{Entity, World};

use crate::{prelude::*, render, shell, tree::Tree, utils::Point};

/// Represents a window. Each window has its own tree, event pipeline and shell.
pub struct ShellAdapter {
    world: World<Tree, StringComponentStore, render::RenderContext2D>,
    ctx: ContextProvider,
}

impl ShellAdapter {
    pub fn new(
        world: World<Tree, StringComponentStore, render::RenderContext2D>,
        ctx: ContextProvider,
    ) -> Self {
        ShellAdapter { world, ctx }
    }
}

impl ShellAdapter {
    fn root(&mut self) -> Entity {
        self.world
            .entity_component_manager()
            .entity_store()
            .root
            .unwrap()
    }
}

impl shell::ShellAdapter for ShellAdapter {
    fn resize(&mut self, width: f64, height: f64) {
        let root = self.root();
        self.ctx
            .event_queue
            .borrow_mut()
            .register_event_with_strategy(
                WindowEvent::Resize { width, height },
                EventStrategy::Direct,
                root,
            );
    }

    fn mouse(&mut self, x: f64, y: f64) {
        let root = self.root();
        self.ctx.mouse_position.set(Point::new(x, y));
        self.ctx
            .event_queue
            .borrow_mut()
            .register_event(MouseMoveEvent { x, y }, root)
    }

    fn scroll(&mut self, delta_x: f64, delta_y: f64) {
        let root = self.root();
        self.ctx.event_queue.borrow_mut().register_event(
            ScrollEvent {
                delta: Point::new(delta_x, delta_y),
            },
            root,
        )
    }

    fn mouse_event(&mut self, event: shell::MouseEvent) {
        let root = self.root();
        match event.state {
            shell::ButtonState::Up => {
                self.ctx.event_queue.borrow_mut().register_event(
                    MouseUpEvent {
                        x: event.x,
                        y: event.y,
                        button: event.button,
                    },
                    root,
                );
                self.ctx.event_queue.borrow_mut().register_event(
                    GlobalMouseUpEvent {
                        x: event.x,
                        y: event.y,
                        button: event.button,
                    },
                    root,
                );
            }
            shell::ButtonState::Down => self.ctx.event_queue.borrow_mut().register_event(
                MouseDownEvent {
                    x: event.x,
                    y: event.y,
                    button: event.button,
                },
                root,
            ),
        }
    }

    fn mouse_position(&self) -> Point {
        self.ctx.mouse_position.get()
    }

    fn key_event(&mut self, event: shell::KeyEvent) {
        let root = self.root();
        match event.state {
            shell::ButtonState::Up => self
                .ctx
                .event_queue
                .borrow_mut()
                .register_event(KeyUpEvent { event }, root),
            shell::ButtonState::Down => self
                .ctx
                .event_queue
                .borrow_mut()
                .register_event(KeyDownEvent { event }, root),
        }
    }

    fn quit_event(&mut self) {
        let root = self.root();

        self.ctx
            .event_queue
            .borrow_mut()
            .register_event(SystemEvent::Quit, root);
    }

    fn active(&mut self, active: bool) {
        let root = self.root();

        self.ctx
            .event_queue
            .borrow_mut()
            .register_event_with_strategy(
                WindowEvent::ActiveChanged(active),
                EventStrategy::Direct,
                root,
            );
    }

    fn run(&mut self, render_context: &mut render::RenderContext2D) {
        self.world.run_with_context(render_context);
    }
}
